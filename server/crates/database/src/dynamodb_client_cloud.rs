use super::attribute_value_parser;
use super::dynamodb_client_trait::IDynamoDbClient;
use anyhow::Error;
use async_trait::async_trait;
use aws_config::meta::region::RegionProviderChain;
use aws_config::{self, BehaviorVersion};
use aws_sdk_dynamodb::operation::query::{QueryInput, QueryOutput};
use aws_sdk_dynamodb::types::{ItemResponse, TransactGetItem, TransactWriteItem};
use aws_sdk_dynamodb::{config::Region, Client};
use std::env;

pub struct DynamoDbClient {
    client: Client,
}

impl DynamoDbClient {
    pub async fn new() -> Result<Self, Error> {
        let region_name = env::var("AWS_REGION")?;
        let region_provider =
            RegionProviderChain::first_try(Region::new(region_name)).or_default_provider();
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&config);
        Ok(DynamoDbClient { client })
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
        if item.item.is_none() {
            return Ok(None);
        }
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

    async fn query(&self, query: QueryInput) -> Result<QueryOutput, Error> {
        let query_output = self
            .client
            .query()
            .set_table_name(query.table_name)
            .set_index_name(query.index_name)
            .set_select(query.select)
            .set_attributes_to_get(query.attributes_to_get)
            .set_limit(query.limit)
            .set_consistent_read(query.consistent_read)
            .set_key_conditions(query.key_conditions)
            .set_query_filter(query.query_filter)
            .set_conditional_operator(query.conditional_operator)
            .set_scan_index_forward(query.scan_index_forward)
            .set_exclusive_start_key(query.exclusive_start_key)
            .set_return_consumed_capacity(query.return_consumed_capacity)
            .set_projection_expression(query.projection_expression)
            .set_filter_expression(query.filter_expression)
            .set_key_condition_expression(query.key_condition_expression)
            .set_expression_attribute_names(query.expression_attribute_names)
            .set_expression_attribute_values(query.expression_attribute_values)
            .send()
            .await?;

        Ok(query_output)
    }
}
