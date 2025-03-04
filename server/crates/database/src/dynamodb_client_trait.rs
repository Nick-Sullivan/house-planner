use anyhow::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    operation::query::{QueryInput, QueryOutput},
    types::{ItemResponse, TransactGetItem, TransactWriteItem},
};

#[async_trait]
pub trait IDynamoDbClient: Sync + Send {
    async fn read_single(&self, item: TransactGetItem) -> Result<Option<ItemResponse>, Error>;
    async fn write(&self, items: Vec<TransactWriteItem>) -> Result<(), Error>;
    async fn write_single(&self, item: TransactWriteItem) -> Result<(), Error>;
    async fn query(&self, query: QueryInput) -> Result<QueryOutput, Error>;
}
