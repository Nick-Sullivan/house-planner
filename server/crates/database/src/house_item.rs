use super::attribute_value_parser::parse_attribute_value;
use super::paginated_models::{PaginatedDbRequest, PaginatedDbResponse};
use crate::dynamodb_client_trait::IDynamoDbClient;
use anyhow::{Error, Ok};
use aws_sdk_dynamodb::{
    operation::query::QueryInput,
    types::{AttributeValue, Put, TransactWriteItem},
};
use chrono::{Duration, Utc};
use std::{collections::HashMap, env};

#[derive(Clone, Debug)]
pub struct HouseItem {
    pub h3_index: String,
    pub address: String,
    pub city_code: String,
    pub url: String,
    pub lat: f64,
    pub lng: f64,
}

impl HouseItem {
    pub fn from_map(hash_map: &HashMap<String, AttributeValue>) -> Result<Self, Error> {
        let h3_index = parse_attribute_value::<String>(hash_map.get("H3Index"))?;
        let address = parse_attribute_value::<String>(hash_map.get("Address"))?;
        let city_code = parse_attribute_value::<String>(hash_map.get("CityCode"))?;
        let url = parse_attribute_value::<String>(hash_map.get("Url"))?;
        let lat = parse_attribute_value::<f64>(hash_map.get("Lat"))?;
        let lng = parse_attribute_value::<f64>(hash_map.get("Lng"))?;
        let item = Self {
            h3_index,
            address,
            city_code,
            url,
            lat,
            lng,
        };
        Ok(item)
    }

    pub fn get_table_name() -> Result<String, Error> {
        let name: String = env::var("HOUSES_TABLE_NAME")?;
        Ok(name)
    }

    pub async fn list_by_h3_index_from_db(
        h3_index: &str,
        db: &dyn IDynamoDbClient,
    ) -> Result<Vec<Self>, Error> {
        let query_input = Self::query_by_h3_index(h3_index)?;
        let query_output = db.query(query_input).await?;
        let items = query_output.items.unwrap_or_default();
        let mut results = Vec::new();
        for item in items {
            let spatial_distance_item = Self::from_map(&item)?;
            results.push(spatial_distance_item);
        }
        Ok(results)
    }

    pub async fn list_by_city_from_db(
        request: PaginatedDbRequest<String>,
        db: &dyn IDynamoDbClient,
    ) -> Result<PaginatedDbResponse<Self>, Error> {
        let query_input = Self::query_by_city(request)?;
        let query_output = db.query(query_input).await?;
        let items = query_output.items.unwrap_or_default();
        let mut results = Vec::new();
        for item in items {
            let spatial_distance_item = Self::from_map(&item)?;
            results.push(spatial_distance_item);
        }
        Ok(PaginatedDbResponse {
            items: results,
            count: query_output.count,
            last_evaluated_key: query_output.last_evaluated_key,
        })
    }

    pub fn save(&self) -> Result<TransactWriteItem, Error> {
        let ttl_timestamp = (Utc::now() + Duration::days(1)).timestamp();
        let put_item = Put::builder()
            .table_name(Self::get_table_name()?)
            .item("H3Index", AttributeValue::S(self.h3_index.to_string()))
            .item("Address", AttributeValue::S(self.address.to_string()))
            .item("CityCode", AttributeValue::S(self.city_code.clone()))
            .item("Url", AttributeValue::S(self.url.to_string()))
            .item("Lat", AttributeValue::N(self.lat.to_string()))
            .item("Lng", AttributeValue::N(self.lng.to_string()))
            .item("TimeToLive", AttributeValue::N(ttl_timestamp.to_string()))
            .build()?;
        let transaction_item = TransactWriteItem::builder().put(put_item).build();
        Ok(transaction_item)
    }

    fn query_by_city(request: PaginatedDbRequest<String>) -> Result<QueryInput, Error> {
        let mut builder = QueryInput::builder()
            .table_name(Self::get_table_name()?)
            .index_name("CityCodeIndex")
            .key_condition_expression("#city_code = :city_code")
            .expression_attribute_names("#city_code", "CityCode")
            .expression_attribute_values(":city_code", AttributeValue::S(request.value.to_string()))
            .set_exclusive_start_key(request.last_evaluated_key);
        if let Some(limit) = request.limit {
            builder = builder.limit(limit);
        }
        let query_input = builder.build()?;
        Ok(query_input)
    }

    fn query_by_h3_index(h3_index: &str) -> Result<QueryInput, Error> {
        let query_input = QueryInput::builder()
            .table_name(Self::get_table_name()?)
            .key_condition_expression("#h3_index = :h3_index")
            .expression_attribute_names("#h3_index", "H3Index")
            .expression_attribute_values(":h3_index", AttributeValue::S(h3_index.to_string()))
            .build()?;
        Ok(query_input)
    }
}
