use super::attribute_value_parser::{self, parse_attribute_value};
use super::dynamodb_client_trait::IDynamoDbClient;
use anyhow::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::operation::transact_get_items::builders::TransactGetItemsOutputBuilder;
use aws_sdk_dynamodb::types::{
    AttributeValue, Delete, ItemResponse, Put, TransactGetItem, TransactWriteItem,
};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct FakeItem {
    pub hash_map: HashMap<String, AttributeValue>,
}

pub struct DynamoDbClient {
    requirements_table: RwLock<HashMap<String, FakeItem>>,
    spatial_distances_table: RwLock<HashMap<String, FakeItem>>,
}

impl DynamoDbClient {
    pub async fn new() -> Self {
        let requirements_table = RwLock::new(HashMap::new());
        let spatial_distances_table = RwLock::new(HashMap::new());
        DynamoDbClient {
            requirements_table,
            spatial_distances_table,
        }
    }

    fn get_table(&self, table_name: &str) -> &RwLock<HashMap<String, FakeItem>> {
        if table_name.ends_with("Requirements") {
            &self.requirements_table
        } else if table_name.ends_with("SpatialDistances") {
            &self.spatial_distances_table
        } else {
            panic!("Unrecognised table {:?}", table_name);
        }
    }

    fn get_primary_key(&self, table_name: &str) -> &str {
        if table_name.ends_with("Requirements") {
            "RequirementId"
        } else if table_name.ends_with("SpatialDistances") {
            "SourceIndex"
        } else {
            panic!("Unrecognised table {:?}", table_name);
        }
    }

    fn write_put(&self, put: Put) -> Result<(), Error> {
        let table = self.get_table(&put.table_name);
        let primary_key_column = self.get_primary_key(&put.table_name);
        let primary_key = parse_attribute_value::<String>(put.item.get(primary_key_column))?;
        let item = FakeItem {
            hash_map: put.item.clone(),
        };
        let mut hash_map = table.write().unwrap();
        if put.condition_expression.is_some() {
            let existing_item = hash_map.get(&primary_key);
            self.check_put_condition(put, &existing_item)?;
        }
        hash_map.insert(primary_key.to_string(), item);
        Ok(())
    }

    fn write_delete(&self, delete: Delete) -> Result<(), Error> {
        let table = self.get_table(&delete.table_name);
        let primary_key_column = self.get_primary_key(&delete.table_name);
        let primary_key = parse_attribute_value::<String>(delete.key.get(primary_key_column))?;
        let mut hash_map = table.write().unwrap();
        if delete.condition_expression.is_some() {
            let existing_item = hash_map.get(&primary_key);
            self.check_delete_condition(delete, &existing_item)?;
        }
        hash_map.remove(&primary_key.to_string());
        Ok(())
    }

    fn check_put_condition(
        &self,
        put: Put,
        existing_item: &Option<&FakeItem>,
    ) -> Result<(), Error> {
        let expression = put
            .condition_expression
            .ok_or(anyhow::anyhow!("No condition"))?;

        let must_be_new = expression.starts_with("attribute_not_exists");
        match (existing_item, must_be_new) {
            (Some(_), true) => {
                return Err(anyhow::anyhow!("Item already exists"));
            }
            (Some(existing_item), false) => {
                let actual_version =
                    parse_attribute_value::<i32>(existing_item.hash_map.get("version"))?;
                let new_version = parse_attribute_value::<i32>(put.item.get("version"))?;
                if new_version != actual_version + 1 {
                    return Err(anyhow::anyhow!("Version mismatch"));
                }
            }
            (None, false) => {
                return Err(anyhow::anyhow!("Item does not exist"));
            }
            _ => {}
        }
        Ok(())
    }

    fn check_delete_condition(
        &self,
        delete: Delete,
        existing_item: &Option<&FakeItem>,
    ) -> Result<(), Error> {
        let _ = delete
            .condition_expression
            .ok_or(anyhow::anyhow!("No condition"))?;
        let attributes = delete
            .expression_attribute_values
            .ok_or(anyhow::anyhow!("No expression values"))?;
        let expected_version =
            parse_attribute_value::<i32>(attributes.get(&":old_version".to_string()))?;

        match existing_item {
            Some(existing_item) => {
                let actual_version =
                    parse_attribute_value::<i32>(existing_item.hash_map.get("version"))?;
                if expected_version != actual_version {
                    return Err(anyhow::anyhow!("Version mismatch"));
                }
            }
            None => {
                return Err(anyhow::anyhow!("Item does not exist"));
            }
        }
        Ok(())
    }
}

#[async_trait]
impl IDynamoDbClient for DynamoDbClient {
    async fn read_single(&self, item: TransactGetItem) -> Result<Option<ItemResponse>, Error> {
        let get = item.get.ok_or(anyhow::anyhow!("Only Gets are supported"))?;
        let table = self.get_table(&get.table_name);
        let primary_key_column = self.get_primary_key(&get.table_name);
        let hash_map = table.read().unwrap();
        let primary_key = parse_attribute_value::<String>(get.key.get(primary_key_column))?;
        let item = match hash_map.get(&primary_key) {
            Some(item) => item,
            None => return Ok(None),
        };
        let item_response = ItemResponse::builder()
            .set_item(Some(item.hash_map.clone()))
            .build();
        let output = TransactGetItemsOutputBuilder::default()
            .responses(item_response)
            .build();
        let items = output.responses.ok_or(anyhow::anyhow!("No response"))?;
        let item = attribute_value_parser::single(items)?;
        Ok(Some(item))
    }

    async fn write(&self, items: Vec<TransactWriteItem>) -> Result<(), Error> {
        for item in items {
            self.write_single(item).await?;
        }
        Ok(())
    }

    async fn write_single(&self, item: TransactWriteItem) -> Result<(), Error> {
        if let Some(put) = item.put {
            self.write_put(put)?;
        } else if let Some(delete) = item.delete {
            self.write_delete(delete)?;
        } else {
            return Err(anyhow::anyhow!("Only Put/Delete is supported"));
        }
        Ok(())
    }

    async fn query_spatial_distance_item(&self, city_code: &str) -> Result<QueryOutput, Error> {
        let table = self.spatial_distances_table.read().unwrap();
        let mut items = Vec::new();
        for (_key, item) in table.iter() {
            if let Some(AttributeValue::S(code)) = item.hash_map.get("CityCode") {
                if code == city_code {
                    items.push(item.hash_map.clone());
                }
            }
        }
        let query_output = QueryOutput::builder().set_items(Some(items)).build();
        Ok(query_output)
    }
}
