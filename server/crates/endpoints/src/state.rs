use database::dynamodb_client_trait::IDynamoDbClient;
use h3_mapper::h3_client::H3Client;
use houses::house_client::HouseClient;

pub struct AppState {
    pub db_client: Box<dyn IDynamoDbClient>,
    pub house_client: HouseClient,
    pub h3_client: H3Client,
}
