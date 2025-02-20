use crate::houses::models::House;
use super::response::{ErrorResponse, PaginatedResponse};
use super::request::{PaginationParams, AppState};
use axum::extract::{State, Query, Path};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use uuid::Uuid;

pub const MAP_TAG: &str = "map";

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
    .routes(utoipa_axum::routes!(recommend))
}

#[derive(utoipa::FromSchema, serde::Serialize)]
pub enum RequirementType {
    All,
    OneOf,
}

#[derive(utoipa::FromSchema, serde::Serialize)]
pub enum TravelMode {
    Driving,
    Walking,
    Bicycling,
    PublicTransport,
}

#[derive(utoipa::FromSchema, utoipa::ToSchema, serde::Serialize)]
pub struct Location {
    pub id: i32,
    pub h3_index: String,
}

#[derive(utoipa::FromSchema, serde::Serialize)]
pub struct RequirementRequest {
    pub requirement_id: Uuid,
    pub country_code: String,
    pub city_code: String,
    pub requirement_type: RequirementType,
    pub travel_mode: TravelMode,
    pub locations: Vec<Location>
    pub tolerated_duration: i32,
}




#[derive(utoipa::FromSchema, serde::Serialize)]
pub struct MapRequest {
    pub country_code: String,
    pub city_code: String,
    pub requirement_ids: Vec<Uuid>,
}

// Each POST request has 1 or more requirements, and will return the map with the scores.
// Each requirement has a UUID, and the resulting map score will be saved into the database
// When requesting for houses, the requests will have the list of requirement IDs
// the page can GET requirements so that the user can share a URL

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct RequirementScoreResponse {
    pub requirement_id: Uuid,
    pub selected_location_id: i32,
    pub duration: i32,
    pub score: i32,
}

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct MapTileResponse {
    pub h3_index: i32,
    pub mean_score: i32,
    pub requirement_scores: Vec<RequirementScoreResponse>,
}


#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct MapResponse {
    pub tiles: Vec<MapTileResponse>,
}


#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct RecommendationResponse {
}


#[utoipa::path(
    post, 
    path = "requirement", 
    tag = MAP_TAG,
    request_body = RecommendationRequest,
    responses(
        (status = OK, body = RecommendationResponse), 
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
pub async fn post_requirement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RecommendationRequest>,
) -> Result<Json<RecommendationResponse>, (StatusCode, Json<ErrorResponse>)> {
    
    // load the requirement from the database if it exists
    // let requirement = RequirementItem::from_db(&request.requirement_id, state.db_client).await?;
    // if let Some(requirement) = requirement {
    //     // If the input hasn't changed, return
    // }
    // // load all the tiles for this city
    // let source_index = request.locations.first().unwrap().location.h3_index;
    // let distances = SpatialDistanceItem::list_city_items_from_db(&request.city_code, vec![h3_index], state.db_client).await?;
    // // for each tile i in the city
    // //  for each location j in the requirement
    // for distance in distances {
    //     // load the duration from the database
    //     // calculate the score
    // }
    // // calculate the mean score
    // // save to the database
    return RecommendationResponse{};
}

#[utoipa::path(
    delete, 
    path = "requirement/{requirement_id}", 
    tag = MAP_TAG,
    params(
        ("requirement_id" = Uuid, Path, description = "ID of the requirement to delete")
    ),
    responses(
        (status = OK, description = "Requirement deleted successfully"), 
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
pub async fn delete_requirement(
    State(state): State<Arc<AppState>>,
    Json(recommendation_request): Json<RecommendationRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {

    Ok(StatusCode::OK)
}


#[utoipa::path(
    get, 
    path = "", 
    tag = MAP_TAG,
    request_body = MapRequest,
    responses(
        (status = OK, body = MapResponse), 
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
pub async fn get_map(
    State(state): State<Arc<AppState>>,
    Json(map_request): Json<MapRequest>,
) -> Result<Json<MapResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Load the requirements from the database
    // Verify they belong to the same location
    // for each tile, combine the scores
    // return the map
    MapResponse{tiles: vec![]}
}
