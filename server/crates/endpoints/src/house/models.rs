use houses::models::House;

pub const HOUSE_TAG: &str = "house";

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct HouseResponse {
    pub id: i32,
    pub address: String,
    pub url: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

impl From<House> for HouseResponse {
    fn from(house: House) -> Self {
        HouseResponse {
            id: house.id,
            address: house.address,
            url: house.url,
            lat: house.lat,
            lon: house.lon,
        }
    }
}
