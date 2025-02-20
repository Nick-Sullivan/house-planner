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

pub struct SpatialDistanceItem {
    pub city_code: String,
    pub source_index: String,
    pub destination_index: String,
}

impl SpatialDistanceItem {
    pub fn new(city_code: &str, source_index: &str, destination_index: &str) -> Self {
        SpatialDistanceItem {
            city_code: city_code.to_string(),
            source_index: source_index.clone(),
            destination_index: destination_index.clone(),
        }
    }

    pub async fn list_city_items_from_db(city_code: &str, source_indices: &[&str], db: &DynamoDbClient) -> Result<Vec<Self>, LogicError> {
        let query = Self::list_city_items_with_index(city_code, source_indices);
        let output = db.query(query).await?;
        let items = output
            .items
            .ok_or(LogicError::GetItemError("Item not found".to_string()))?;
        let mut result = Vec::new();
        for item in items {
            let item = Self::from_map(&item)?;
            result.push(item);
        }
        Ok(result)
    }

    pub fn list_city_items_with_index(city_code: &str, source_indices: &[&str]) -> Query {
        let mut expression_attribute_values = HashMap::new();
        expression_attribute_values.insert(":city_code".to_string(), AttributeValue::S(city_code.to_string()));
        let mut filter_expression = String::new();
        for (i, source_index) in source_indices.iter().enumerate() {
            let placeholder = format!(":source_index{}", i);
            expression_attribute_values.insert(placeholder.clone(), AttributeValue::S(source_index.to_string()));
            if i > 0 {
                filter_expression.push_str(" OR ");
            }
            filter_expression.push_str(&format!("#source_index = {}", placeholder));
        }
        Query::builder()
            .table_name(Self::get_table_name())
            .index_name("CityCodeIndex")
            .key_condition_expression("#city_code = :city_code")
            .filter_expression(filter_expression)
            .expression_attribute_names("#city_code", "CityCode")
            .expression_attribute_names("#source_index", "SourceIndex")
            .expression_attribute_values(expression_attribute_values)
            .select(Select::AllAttributes)
            .build()
    }


    pub fn from_map(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, LogicError> {
        let city_code = parse_attribute_value::<String>(hash_map.get("CityCode"))?;
        let source_index = parse_attribute_value::<String>(requirement_id.get("SourceIndex"))?;
        let destination_index = parse_attribute_value::<String>(requirement_id.get("DestinationIndex"))?;
        let item = Self {
            city_code,
            source_index,
            destination_index,
        };
        Ok(item)
    }

    fn get_table_name() -> String {
        env::var("SPATIAL_DISTANCES_TABLE_NAME").unwrap_or_else(|_| "".to_string())
    }

    pub fn save(&self) -> Result<TransactWriteItem, LogicError> {
        let put_item = Put::builder()
            .table_name(Self::get_table_name())
            .item("CityCode", AttributeValue::S(self.city_code.clone()))
            .item("SourceIndex", AttributeValue::S(self.source_index.clone()))
            .item("DestinationIndex", AttributeValue::S(self.destination_index.clone()))
            .build()
            .map_err(|e| LogicError::UpdateItemError(e.to_string()))?;
        let transaction_item = TransactWriteItem::builder().put(put_item).build();
        Ok(transaction_item)
    }

}