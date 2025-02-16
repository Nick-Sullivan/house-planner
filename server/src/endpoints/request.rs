use crate::houses::house_client::HouseClient;
use serde::Deserialize;

pub struct AppState {
    pub house_client: HouseClient,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}
