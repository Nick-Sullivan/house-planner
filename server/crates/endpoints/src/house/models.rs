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
    pub price_lower: i32,
    pub price_upper: i32,
    pub num_bathrooms: i32,
    pub num_bedrooms: i32,
    pub num_carspaces: i32,
    pub property_type: String,
}

impl From<HouseItem> for HouseResponse {
    fn from(house: HouseItem) -> Self {
        HouseResponse {
            h3_index: house.h3_index,
            address: house.address,
            url: house.url,
            lat: house.lat,
            lon: house.lng,
            price_lower: house.price_lower,
            price_upper: house.price_upper,
            num_bathrooms: house.num_bathrooms,
            num_bedrooms: house.num_bedrooms,
            num_carspaces: house.num_carspaces,
            property_type: house.property_type,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HouseRequestFilter {
    pub city_code: Option<String>,
    pub h3_index: Option<String>,
}
