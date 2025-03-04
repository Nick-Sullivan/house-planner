use super::attribute_value_parser::parse_attribute_value;
use crate::dynamodb_client_trait::IDynamoDbClient;
use anyhow::{Error, Ok};
use aws_sdk_dynamodb::{
    operation::query::QueryInput,
    types::{AttributeValue, Get, Put, TransactGetItem, TransactWriteItem},
};
use std::{collections::HashMap, env};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SpatialDistanceItem {
    pub city_code: String,
    pub source_index: String,
    pub destination_index: String,
    pub duration_walk: i32,
    pub duration_cycle: i32,
    pub duration_drive: i32,
    pub duration_transit: i32,
}

impl SpatialDistanceItem {
    pub async fn from_db(
        requirement_id: &Uuid,
        db: &dyn IDynamoDbClient,
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
        let source_index = parse_attribute_value::<String>(hash_map.get("SourceIndex"))?;
        let destination_index = parse_attribute_value::<String>(hash_map.get("DestinationIndex"))?;
        let duration_walk = parse_attribute_value::<i32>(hash_map.get("DurationWalk"))?;
        let duration_cycle = parse_attribute_value::<i32>(hash_map.get("DurationCycle"))?;
        let duration_drive = parse_attribute_value::<i32>(hash_map.get("DurationDrive"))?;
        let duration_transit = parse_attribute_value::<i32>(hash_map.get("DurationTransit"))?;
        let item = Self {
            city_code,
            source_index,
            destination_index,
            duration_walk,
            duration_cycle,
            duration_drive,
            duration_transit,
        };
        Ok(item)
    }

    pub fn get_table_name() -> Result<String, Error> {
        let name: String = env::var("SPATIAL_DISTANCES_TABLE_NAME")?;
        Ok(name)
    }

    pub fn get(requirement_id: &str) -> Result<TransactGetItem, Error> {
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

    pub async fn list_by_city_from_db(
        city_code: &str,
        db: &dyn IDynamoDbClient,
    ) -> Result<Vec<Self>, Error> {
        let query_input = Self::query_by_city(city_code)?;
        let query_output = db.query(query_input).await?;
        let items = query_output.items.unwrap_or_default();
        let mut results = Vec::new();
        for item in items {
            let spatial_distance_item = Self::from_map(&item)?;
            results.push(spatial_distance_item);
        }
        Ok(results)
    }

    pub async fn list_by_source_from_db(
        source_index: &str,
        db: &dyn IDynamoDbClient,
    ) -> Result<Vec<Self>, Error> {
        let query_input = Self::query_by_source_index(source_index)?;
        let query_output = db.query(query_input).await?;
        let items = query_output.items.unwrap_or_default();
        let mut results = Vec::new();
        for item in items {
            let spatial_distance_item = Self::from_map(&item)?;
            results.push(spatial_distance_item);
        }
        Ok(results)
    }

    pub fn save(&self) -> Result<TransactWriteItem, Error> {
        let put_item = Put::builder()
            .table_name(Self::get_table_name()?)
            .item("CityCode", AttributeValue::S(self.city_code.clone()))
            .item(
                "SourceIndex",
                AttributeValue::S(self.source_index.to_string()),
            )
            .item(
                "DestinationIndex",
                AttributeValue::S(self.destination_index.to_string()),
            )
            .item(
                "DurationWalk",
                AttributeValue::N(self.duration_walk.to_string()),
            )
            .item(
                "DurationCycle",
                AttributeValue::N(self.duration_cycle.to_string()),
            )
            .item(
                "DurationDrive",
                AttributeValue::N(self.duration_drive.to_string()),
            )
            .item(
                "DurationTransit",
                AttributeValue::N(self.duration_transit.to_string()),
            )
            .build()?;
        let transaction_item = TransactWriteItem::builder().put(put_item).build();
        Ok(transaction_item)
    }

    fn query_by_city(city_code: &str) -> Result<QueryInput, Error> {
        let query_input = QueryInput::builder()
            .table_name(Self::get_table_name()?)
            .index_name("CityCodeIndex")
            .key_condition_expression("#city_code = :city_code")
            .expression_attribute_names("#city_code", "CityCode")
            .expression_attribute_values(":city_code", AttributeValue::S(city_code.to_string()))
            .build()?;
        Ok(query_input)
    }

    fn query_by_source_index(source_index: &str) -> Result<QueryInput, Error> {
        let query_input = QueryInput::builder()
            .table_name(Self::get_table_name()?)
            .key_condition_expression("#source_index = :source_index")
            .expression_attribute_names("#source_index", "SourceIndex")
            .expression_attribute_values(
                ":source_index",
                AttributeValue::S(source_index.to_string()),
            )
            .build()?;
        Ok(query_input)
    }
}
