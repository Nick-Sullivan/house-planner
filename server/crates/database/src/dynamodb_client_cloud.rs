use crate::spatial_distance_item::SpatialDistanceItem;

use super::attribute_value_parser;
use super::dynamodb_client_trait::IDynamoDbClient;
use anyhow::Error;
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_config::{self, BehaviorVersion};
use aws_sdk_dynamodb::operation::query::QueryOutput;
use aws_sdk_dynamodb::types::{AttributeValue, ItemResponse, TransactGetItem, TransactWriteItem};
use aws_sdk_dynamodb::{config::Region, Client};
use std::env;

pub struct DynamoDbClient {
    client: Client,
}

impl DynamoDbClient {
    pub async fn new() -> Self {
        let region_name = env::var("AWS_REGION").unwrap_or_else(|_| "".to_string());
        let region_provider =
            RegionProviderChain::first_try(Region::new(region_name)).or_default_provider();
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&config);
        DynamoDbClient { client }
    }
}

#[async_trait]
impl IDynamoDbClient for DynamoDbClient {
    async fn read_single(&self, item: TransactGetItem) -> Result<Option<ItemResponse>, Error> {
        let result = self
            .client
            .transact_get_items()
            .transact_items(item)
            .send()
            .await?;
        let items = result.responses.ok_or(anyhow::anyhow!("No response"))?;
        if items.is_empty() {
            return Ok(None);
        }
        let item = attribute_value_parser::single(items)?;
        Ok(Some(item))
    }

    async fn write(&self, items: Vec<TransactWriteItem>) -> Result<(), Error> {
        self.client
            .transact_write_items()
            .set_transact_items(Some(items))
            .send()
            .await?;
        Ok(())
    }

    async fn write_single(&self, item: TransactWriteItem) -> Result<(), Error> {
        self.write(vec![item]).await
    }

    async fn query_spatial_distance_item(&self, city_code: &str) -> Result<QueryOutput, Error> {
        // I tried to put this in the spatial distance item, but QueryInput can't be applied to
        // a client.
        // TODO, make this better
        let query_output = self
            .client
            .query()
            .table_name(SpatialDistanceItem::get_table_name()?)
            .index_name("CityCodeIndex")
            .key_condition_expression("#city_code = :city_code")
            .expression_attribute_names("#city_code", "CityCode")
            .expression_attribute_values(":city_code", AttributeValue::S(city_code.to_string()))
            .send()
            .await?;
        Ok(query_output)
    }
}
