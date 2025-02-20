use crate::attribute_value_parser::{parse_attribute_value, DATETIME_FORMAT};
use crate::{DynamoDbClient, IDynamoDbClient};
use aws_sdk_dynamodb::types::{AttributeValue, Get, Put, TransactGetItem, TransactWriteItem};
use chrono::{DateTime, Utc};
use domain::errors::LogicError;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, env};

pub struct RequirementItem {
    pub city_code: String,
    pub requirement_id: Uuid,
}

impl RequirementItem {
    pub fn new(city_code: &str, requirement_id: &Uuid, ) -> Self {
        RequirementItem {
            city_code: city_code.to_string(),
            requirement_id: requirement_id.clone(),
        }
    }

    pub async fn from_db(requirement_id: &str, db: &DynamoDbClient) -> Result<Option<Self>> {
        let transaction = Self::get(requirement_id).ok()?;
        let output = db.read_single(transaction).await.ok()?;
        if output.is_none() {
            return Ok(None);
        }
        let attribute = output.item?;
        let item = Self::from_map(&attribute).ok()?;
        Some(item)
    }

    pub fn from_map(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, LogicError> {
        let city_code = parse_attribute_value::<String>(hash_map.get("CityCode"))?;
        let requirement_id = parse_attribute_value::<Uuid>(requirement_id.get("RequirementId"))?;
        let item = RequirementItem {
            city_code,
            requirement_id,
        };
        Ok(item)
    }

    fn get_table_name() -> String {
        env::var("REQUIREMENTS_TABLE_NAME").unwrap_or_else(|_| "".to_string())
    }

    pub fn get(requirement_id: &str) -> Result<Option<TransactGetItem>> {
        // TODO handle NULL
        let item = Get::builder()
            .table_name(Self::get_table_name())
            .key("RequirementId", AttributeValue::S(requirement_id.to_string()))
            .build()?
        let transaction_item = TransactGetItem::builder().get(item).build();
        Ok(Some(transaction_item))
    }

    pub fn save(&self) -> Result<TransactWriteItem, LogicError> {
        let put_item = Put::builder()
            .table_name(Self::get_table_name())
            .item("CityCode", AttributeValue::S(self.city_code.clone()))
            .item("RequirementId", AttributeValue::S(self.requirement_id.clone()))
            .build()
            .map_err(|e| LogicError::UpdateItemError(e.to_string()))?;
        let transaction_item = TransactWriteItem::builder().put(put_item).build();
        Ok(transaction_item)
    }

    pub fn delete(&self) -> Result<TransactWriteItem, LogicError> {
        let delete_item = aws_sdk_dynamodb::types::Delete::builder()
            .table_name(Self::get_table_name())
            .key("RequirementId", AttributeValue::S(self.requirement_id.to_string()))
            .build()
            .map_err(|e| LogicError::DeleteItemError(e.to_string()))?;
        let transaction_item = TransactWriteItem::builder().delete(delete_item).build();
        Ok(transaction_item)
    }
}