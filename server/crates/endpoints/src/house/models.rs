use database::house_item::HouseItem;
use serde::Deserialize;

pub const HOUSE_TAG: &str = "house";

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct HouseResponse {
    pub h3_index: String,
    pub address: String,
    pub url: String,
    pub lat: f64,
    pub lon: f64,
}

impl From<HouseItem> for HouseResponse {
    fn from(house: HouseItem) -> Self {
        HouseResponse {
            h3_index: house.h3_index,
            address: house.address,
            url: house.url,
            lat: house.lat,
            lon: house.lng,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HouseRequestFilter {
    pub city_code: Option<String>,
    pub h3_index: Option<String>,
}
