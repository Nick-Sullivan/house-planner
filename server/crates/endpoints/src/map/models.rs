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
    pub h3_index: String,
}

#[derive(ToSchema, Deserialize)]
pub struct RequirementRequest {
    pub requirement_id: Uuid,
    pub country_code: String,
    pub city_code: String,
    pub travel_mode: TravelMode,
    pub locations: Vec<Location>,
    pub tolerated_duration: i32,
}

#[derive(ToSchema, Deserialize)]
pub struct MapRequest {
    pub country_code: String,
    pub city_code: String,
    pub requirement_ids: Vec<Uuid>,
}

// Each POST request has 1 or more requirements, and will return the map with the scores.
// Each requirement has a UUID, and the resulting map score will be saved into the database
// When requesting for houses, the requests will have the list of requirement IDs
// the page can GET requirements so that the user can share a URL

#[derive(ToSchema, Serialize)]
pub struct RequirementScoreResponse {
    pub requirement_id: Uuid,
    pub selected_location_id: i32,
    pub duration: i32,
    pub score: i32,
}

#[derive(ToSchema, Serialize)]
pub struct MapTileResponse {
    pub h3_index: i32,
    pub mean_score: i32,
    pub requirement_scores: Vec<RequirementScoreResponse>,
}

#[derive(ToSchema, Serialize)]
pub struct MapResponse {
    pub tiles: Vec<MapTileResponse>,
}

#[derive(ToSchema, Serialize)]
pub struct RequirementResponse {}
