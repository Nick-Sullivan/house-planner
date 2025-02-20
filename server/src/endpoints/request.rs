use crate::houses::house_client::HouseClient;
use serde::Deserialize;
use crate::database::dynamodb_client::DynamoDbClient;


pub struct AppState {
    pub db_client: DynamoDbClient,
    pub house_client: HouseClient,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
