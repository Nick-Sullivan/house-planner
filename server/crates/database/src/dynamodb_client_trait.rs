use anyhow::Error;
use aws_sdk_dynamodb::types::{ItemResponse, TransactGetItem, TransactWriteItem};

#[trait_variant::make(HttpService: Send)]
pub trait IDynamoDbClient {
    async fn read_single(&self, item: TransactGetItem) -> Result<Option<ItemResponse>, Error>;
    async fn write(&self, items: Vec<TransactWriteItem>) -> Result<(), Error>;
    async fn write_single(&self, item: TransactWriteItem) -> Result<(), Error>;
}
