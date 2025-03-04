use database::dynamodb_client_trait::IDynamoDbClient;
use h3_mapper::h3_client::H3Client;
use houses::house_client::HouseClient;
use serde::Deserialize;

pub struct AppState {
    pub db_client: Box<dyn IDynamoDbClient>,
    pub house_client: HouseClient,
    pub h3_client: H3Client,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i32>,
    pub next_token: Option<String>,
}
