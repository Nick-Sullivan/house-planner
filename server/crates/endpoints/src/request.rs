use database::dynamodb_client_trait::IDynamoDbClient;
use houses::house_client::HouseClient;
use serde::Deserialize;

pub struct AppState {
    pub db_client: Box<dyn IDynamoDbClient>,
    pub house_client: HouseClient,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
