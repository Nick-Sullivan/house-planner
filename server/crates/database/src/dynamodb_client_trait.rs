use anyhow::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    operation::query::QueryOutput,
    types::{ItemResponse, TransactGetItem, TransactWriteItem},
};

#[async_trait]
pub trait IDynamoDbClient: Sync + Send {
    async fn read_single(&self, item: TransactGetItem) -> Result<Option<ItemResponse>, Error>;
    async fn write(&self, items: Vec<TransactWriteItem>) -> Result<(), Error>;
    async fn write_single(&self, item: TransactWriteItem) -> Result<(), Error>;
    async fn query_by_city(&self, city_code: &str) -> Result<QueryOutput, Error>;
    async fn query_by_source_index(&self, source_index: &str) -> Result<QueryOutput, Error>;
}
