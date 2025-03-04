use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
pub const MAP_TAG: &str = "map";

#[derive(ToSchema, Deserialize)]
pub enum TravelMode {
    Driving,
    Walking,
    Bicycling,
    PublicTransport,
}

#[derive(ToSchema, Deserialize)]
pub struct Location {
    pub id: i32,
    pub address: String,
    pub h3_index: String,
    pub lat: f64,
    pub lng: f64,
}

#[derive(ToSchema, Deserialize)]
pub struct RequirementRequest {
    pub requirement_id: Uuid,
    // pub country_code: String,
    pub city_code: String,
    pub travel_mode: TravelMode,
    pub locations: Vec<Location>,
    pub tolerated_duration: i32,
}

#[derive(ToSchema, Deserialize, Debug)]
pub struct MapRequest {
    pub city_code: String,
    pub requirement_ids: Vec<Uuid>,
}

// Each POST request has 1 or more requirements, and will return the map with the scores.
// Each requirement has a UUID, and the resulting map score will be saved into the database
// When requesting for houses, the requests will have the list of requirement IDs
// the page can GET requirements so that the user can share a URL

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct TileScore(i32);

// impl TileScore {
//     pub fn new(value: i32) -> Result<Self, Error> {
//         if value <= 100 {
//             Ok(TileScore(value))
//         } else {
//             Err(anyhow::anyhow!(
//                 "Value {} is out of range. Must be between 0 and 100.",
//                 value
//             ))
//         }
//     }

//     pub fn value(&self) -> i32 {
//         self.0
//     }
// }

#[derive(ToSchema, Serialize)]
pub struct RequirementScoreResponse {
    pub requirement_id: Uuid,
    // pub selected_location_id: i32,
    // pub duration: i32,
    pub score: i32,
}

#[derive(ToSchema, Serialize)]
pub struct MapTileResponse {
    pub h3_index: String,
    pub mean_score: i32,
    pub requirement_scores: Vec<RequirementScoreResponse>,
}

#[derive(ToSchema, Serialize)]
pub struct MapResponse {
    pub tiles: Vec<MapTileResponse>,
}

#[derive(ToSchema, Serialize)]
pub struct RequirementResponse {}
