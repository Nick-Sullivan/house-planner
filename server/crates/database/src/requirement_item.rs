use super::attribute_value_parser::parse_attribute_value;
use super::{DynamoDbClient, IDynamoDbClient};
use anyhow::Error;
use aws_sdk_dynamodb::types::{AttributeValue, Get, Put, TransactGetItem, TransactWriteItem};
use std::{collections::HashMap, env};
use uuid::Uuid;

pub struct RequirementItem {
    pub city_code: String,
    pub requirement_id: Uuid,
}

impl RequirementItem {
    pub fn new(city_code: &str, requirement_id: &Uuid) -> Self {
        RequirementItem {
            city_code: city_code.to_string(),
            requirement_id: requirement_id.clone(),
        }
    }

    pub async fn from_db(
        requirement_id: &Uuid,
        db: &DynamoDbClient,
    ) -> Result<Option<Self>, Error> {
        let transaction = Self::get(&requirement_id.to_string())?;
        let output = match db.read_single(transaction).await? {
            Some(output) => output,
            None => return Ok(None),
        };
        let attribute = output.item.ok_or(anyhow::anyhow!("No item"))?;
        let item = Self::from_map(&attribute)?;
        Ok(Some(item))
    }

    pub fn from_map(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
        let city_code = parse_attribute_value::<String>(hash_map.get("CityCode"))?;
        let requirement_id = parse_attribute_value::<Uuid>(hash_map.get("RequirementId"))?;
        let item = RequirementItem {
            city_code,
            requirement_id,
        };
        Ok(item)
    }

    fn get_table_name() -> Result<String, Error> {
        let name: String = env::var("REQUIREMENTS_TABLE_NAME")?;
        Ok(name)
    }

    pub fn get(requirement_id: &str) -> Result<TransactGetItem, Error> {
        // TODO handle NULL
        let item = Get::builder()
            .table_name(Self::get_table_name()?)
            .key(
                "RequirementId",
                AttributeValue::S(requirement_id.to_string()),
            )
            .build()?;
        let transaction_item = TransactGetItem::builder().get(item).build();
        Ok(transaction_item)
    }

    pub fn save(&self) -> Result<TransactWriteItem, Error> {
        let put_item = Put::builder()
            .table_name(Self::get_table_name()?)
            .item("CityCode", AttributeValue::S(self.city_code.clone()))
            .item(
                "RequirementId",
                AttributeValue::S(self.requirement_id.to_string()),
            )
            .build()?;
        let transaction_item = TransactWriteItem::builder().put(put_item).build();
        Ok(transaction_item)
    }

    pub fn delete(&self) -> Result<TransactWriteItem, Error> {
        let delete_item = aws_sdk_dynamodb::types::Delete::builder()
            .table_name(Self::get_table_name()?)
            .key(
                "RequirementId",
                AttributeValue::S(self.requirement_id.to_string()),
            )
            .build()?;
        let transaction_item = TransactWriteItem::builder().delete(delete_item).build();
        Ok(transaction_item)
    }
}
