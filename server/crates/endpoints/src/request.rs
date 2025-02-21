use database::DynamoDbClient;
use houses::house_client::HouseClient;
use serde::Deserialize;

pub struct AppState {
    pub db_client: DynamoDbClient,
    pub house_client: HouseClient,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
