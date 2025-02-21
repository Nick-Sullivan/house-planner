use super::models::{RequirementRequest, RequirementResponse, MAP_TAG};
use crate::request::AppState;
use crate::response::ErrorResponse;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use database::requirement_item::RequirementItem;
use database::IDynamoDbClient;
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
    // let requirement = RequirementItem::from_db(&request.requirement_id, &state.db_client)
    //     .await
    //     .map_err(map_error_to_response)?;
    // if let Some(requirement) = requirement {
    // If the input hasn't changed, return
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
