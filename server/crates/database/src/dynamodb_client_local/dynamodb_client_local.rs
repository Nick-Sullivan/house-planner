use super::super::attribute_value_parser::{self, parse_attribute_value};
use super::super::dynamodb_client_trait::IDynamoDbClient;
use anyhow::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::operation::query::{QueryInput, QueryOutput};
use aws_sdk_dynamodb::operation::transact_get_items::builders::TransactGetItemsOutputBuilder;
use aws_sdk_dynamodb::types::{
    AttributeValue, Delete, ItemResponse, Put, TransactGetItem, TransactWriteItem,
};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::sync::RwLock;

const KEY_JOIN_STR: &str = ":";

pub struct FakeItem {
    pub hash_map: HashMap<String, AttributeValue>,
}

type FakeTable = HashMap<String, FakeItem>;

pub struct DynamoDbClient {
    requirements_table: RwLock<FakeTable>,
    spatial_distances_table: RwLock<FakeTable>,
    houses_table: RwLock<FakeTable>,
}

impl DynamoDbClient {
    pub async fn new() -> Result<Self, Error> {
        let requirements_table = RwLock::new(HashMap::new());
        let spatial_distances_items = DynamoDbClient::load_spatial_distances_data()?;
        let spatial_distances_table = RwLock::new(spatial_distances_items);
        let houses_items = DynamoDbClient::load_houses_data()?;
        let houses_table = RwLock::new(houses_items);
        Ok(DynamoDbClient {
            requirements_table,
            spatial_distances_table,
            houses_table,
        })
    }

    fn load_spatial_distances_data() -> Result<FakeTable, Error> {
        let csv_data = include_str!("spatial_distances.csv");
        let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
        let mut items = HashMap::new();
        for result in reader.records() {
            let record = result?;
            let source_index = record[0].to_string();
            let destination_index = record[1].to_string();
            let city_code = record[2].to_string();
            let duration_cycle = (record[3].parse::<f64>()?.round() as i32).to_string();
            let duration_drive = (record[4].parse::<f64>()?.round() as i32).to_string();
            let duration_transit = (record[5].parse::<f64>()?.round() as i32).to_string();
            let duration_walk = (record[6].parse::<f64>()?.round() as i32).to_string();
            let item = FakeItem {
                hash_map: HashMap::from([
                    (
                        "SourceIndex".to_string(),
                        AttributeValue::S(source_index.clone()),
                    ),
                    (
                        "DestinationIndex".to_string(),
                        AttributeValue::S(destination_index.clone()),
                    ),
                    ("CityCode".to_string(), AttributeValue::S(city_code)),
                    (
                        "DurationCycle".to_string(),
                        AttributeValue::N(duration_cycle),
                    ),
                    (
                        "DurationDrive".to_string(),
                        AttributeValue::N(duration_drive),
                    ),
                    (
                        "DurationTransit".to_string(),
                        AttributeValue::N(duration_transit),
                    ),
                    ("DurationWalk".to_string(), AttributeValue::N(duration_walk)),
                ]),
            };
            let key = format!("{}{}{}", source_index, KEY_JOIN_STR, destination_index);
            items.insert(key, item);
        }
        Ok(items)
    }

    fn load_houses_data() -> Result<FakeTable, Error> {
        let csv_data = include_str!("houses.csv");
        let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
        let mut items = HashMap::new();
        for result in reader.records() {
            let record = result?;
            let h3_index = record[0].to_string();
            let address = record[1].to_string();
            let city_code = record[2].to_string();
            let url = record[3].to_string();
            let lat = (record[4].parse::<f64>()?).to_string();
            let lng = (record[5].parse::<f64>()?).to_string();
            let item = FakeItem {
                hash_map: HashMap::from([
                    ("H3Index".to_string(), AttributeValue::S(h3_index.clone())),
                    ("Address".to_string(), AttributeValue::S(address.clone())),
                    ("CityCode".to_string(), AttributeValue::S(city_code)),
                    ("Url".to_string(), AttributeValue::S(url)),
                    ("Lat".to_string(), AttributeValue::S(lat)),
                    ("Lng".to_string(), AttributeValue::S(lng)),
                ]),
            };
            let key = format!("{}{}{}", h3_index, KEY_JOIN_STR, address);
            items.insert(key, item);
        }
        Ok(items)
    }

    fn get_table(&self, table_name: &str) -> &RwLock<HashMap<String, FakeItem>> {
        if table_name.ends_with("Requirements") {
            &self.requirements_table
        } else if table_name.ends_with("SpatialDistances") {
            &self.spatial_distances_table
        } else if table_name.ends_with("Houses") {
            &self.houses_table
        } else {
            panic!("Unrecognised table {:?}", table_name);
        }
    }

    fn get_primary_key_columns(&self, table_name: &str) -> (&str, Option<&str>) {
        if table_name.ends_with("Requirements") {
            ("RequirementId", None)
        } else if table_name.ends_with("SpatialDistances") {
            ("SourceIndex", Some("DestinationIndex"))
        } else {
            panic!("Unrecognised table {:?}", table_name);
        }
    }

    fn get_primary_key(partition_key: &str, sort_key: Option<&str>) -> String {
        let primary_key = match sort_key {
            None => partition_key,
            Some(sort_key) => &format!("{}{}{}", partition_key, KEY_JOIN_STR, sort_key),
        };
        primary_key.to_string()
    }

    fn write_put(&self, put: Put) -> Result<(), Error> {
        let table = self.get_table(&put.table_name);
        let (partition_column, sort_column) = self.get_primary_key_columns(&put.table_name);
        let partition_key = parse_attribute_value::<String>(put.item.get(partition_column))?;
        let sort_key = match sort_column {
            None => None,
            Some(col) => Some(parse_attribute_value::<String>(put.item.get(col))?),
        };
        let primary_key = Self::get_primary_key(&partition_key, sort_key.as_deref());
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
        let (partition_column, sort_column) = self.get_primary_key_columns(&delete.table_name);
        let partition_key = parse_attribute_value::<String>(delete.key.get(partition_column))?;
        let sort_key = match sort_column {
            None => None,
            Some(col) => Some(parse_attribute_value::<String>(delete.key.get(col))?),
        };
        let primary_key = Self::get_primary_key(&partition_key, sort_key.as_deref());
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
        let hash_map = table.read().unwrap();
        let (partition_column, sort_column) = self.get_primary_key_columns(&get.table_name);
        let partition_key = parse_attribute_value::<String>(get.key.get(partition_column))?;
        let sort_key = match sort_column {
            None => None,
            Some(col) => Some(parse_attribute_value::<String>(get.key.get(col))?),
        };
        let primary_key = Self::get_primary_key(&partition_key, sort_key.as_deref());
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

    async fn query(&self, query: QueryInput) -> Result<QueryOutput, Error> {
        let table_name = query.table_name.ok_or(anyhow::anyhow!("No table name"))?;
        let table = self.get_table(&table_name);
        let table_data = table.read().unwrap();
        let key_condition = query
            .key_condition_expression
            .as_deref()
            .ok_or(anyhow::anyhow!("No key condition expression"))?;
        let attr_names = query
            .expression_attribute_names
            .as_ref()
            .ok_or(anyhow::anyhow!("No expression attribute names"))?;
        let attr_values = query
            .expression_attribute_values
            .as_ref()
            .ok_or(anyhow::anyhow!("No expression attribute values"))?;
        let parts: Vec<&str> = key_condition.split('=').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Unsupported key condition format"));
        }
        let field_key = parts[0].trim();
        let value_key = parts[1].trim();
        let field_name = attr_names.get(field_key).ok_or(anyhow::anyhow!(
            "Field name not found in expression attributes"
        ))?;
        let value = attr_values
            .get(value_key)
            .ok_or(anyhow::anyhow!("Value not found in expression attributes"))?;
        let value = parse_attribute_value::<String>(Some(value))?;
        let mut items = Vec::new();
        for (_key, item) in table_data.iter() {
            if let Some(AttributeValue::S(code)) = item.hash_map.get(field_name) {
                if code == &value {
                    items.push(item.hash_map.clone());
                }
            }
        }
        let query_output = QueryOutput::builder().set_items(Some(items)).build();
        Ok(query_output)
    }
}
