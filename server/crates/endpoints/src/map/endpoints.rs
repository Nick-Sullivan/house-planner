use super::models::{RequirementRequest, RequirementResponse, MAP_TAG};
use crate::request::AppState;
use crate::response::ErrorResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use database::requirement_item::RequirementItem;
use database::spatial_distance_item::SpatialDistanceItem;
use std::collections::HashMap;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(post_requirement))
}

fn map_error_to_response(error: impl ToString) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: error.to_string(),
        }),
    )
}

#[utoipa::path(
    post,
    path = "/requirement",
    tag = MAP_TAG,
    request_body = RequirementRequest,
    responses(
        (status = OK, body = RequirementResponse),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
#[debug_handler]
pub async fn post_requirement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RequirementRequest>,
) -> Result<Json<RequirementResponse>, (StatusCode, Json<ErrorResponse>)> {
    // load the requirement from the database if it exists
    let db = &*state.db_client;
    let requirement = RequirementItem::from_db(&request.requirement_id, db)
        .await
        .map_err(map_error_to_response)?;
    if let Some(_requirement) = requirement {
        // If the input hasn't changed, return
    }
    // load all the tiles for this city
    let tiles = SpatialDistanceItem::list_from_db(&request.city_code, db)
        .await
        .map_err(map_error_to_response)?;
    let tiles_by_index: HashMap<(String, String), SpatialDistanceItem> = tiles
        .clone()
        .into_iter()
        .map(|item| {
            (
                (item.source_index.clone(), item.destination_index.clone()),
                item,
            )
        })
        .collect();
    for tile in tiles {
        let mut duration = i32::MAX;
        for location in &request.locations {
            let source_index = tile.source_index.clone();
            let destination_index = location.h3_index.clone();
            let tile = tiles_by_index
                .get(&(source_index, destination_index))
                .ok_or_else(|| map_error_to_response("Tile not found"))?;
            duration = duration
                .min(tile.duration_drive)
                .min(tile.duration_cycle)
                .min(tile.duration_transit)
                .min(tile.duration_walk);
        }
        let _cost = 2.max(duration / request.tolerated_duration);
        break;
    }
    // calculate the mean score
    // save to the database
    let requirement = RequirementItem::new(&request.city_code, &request.requirement_id);
    state
        .db_client
        .write_single(requirement.save().map_err(map_error_to_response)?)
        .await
        .map_err(map_error_to_response)?;
    Ok(Json(RequirementResponse {}))
}

// #[utoipa::path(
//     delete,
//     path = "requirement/{requirement_id}",
//     tag = MAP_TAG,
//     params(
//         ("requirement_id" = Uuid, Path, description = "ID of the requirement to delete")
//     ),
//     responses(
//         (status = OK, description = "Requirement deleted successfully"),
//         (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
//     )
// )]
// pub async fn delete_requirement(
//     State(state): State<Arc<AppState>>,
//     Json(recommendation_request): Json<RecommendationRequest>,
// ) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
//     Ok(StatusCode::OK)
// }

// #[utoipa::path(
//     get,
//     path = "",
//     tag = MAP_TAG,
//     request_body = MapRequest,
//     responses(
//         (status = OK, body = MapResponse),
//         (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
//     )
// )]
// pub async fn get_map(
//     State(state): State<Arc<AppState>>,
//     Json(map_request): Json<MapRequest>,
// ) -> Result<Json<MapResponse>, (StatusCode, Json<ErrorResponse>)> {
//     // Load the requirements from the database
//     // Verify they belong to the same location
//     // for each tile, combine the scores
//     // return the map
//     MapResponse { tiles: vec![] }
// }
