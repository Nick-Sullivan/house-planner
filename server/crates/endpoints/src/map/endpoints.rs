use super::models::{
    MapRequest, MapResponse, MapTileResponse, RequirementRequest, RequirementResponse,
    RequirementScoreResponse, TravelMode, MAP_TAG,
};
use crate::errors::{map_error_to_response, ErrorResponse};
use crate::state::AppState;
use anyhow::Error;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use database::requirement_item::{MapTile, RequirementItem};
use database::spatial_distance_item::SpatialDistanceItem;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(post_requirement))
        .routes(routes!(get_map))
}

#[utoipa::path(
    post,
    path = "/requirements",
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
    let db = &*state.db_client;
    let h3 = &state.h3_client;
    let requirement = RequirementItem::from_db(&request.requirement_id, db)
        .await
        .map_err(map_error_to_response)?;
    if let Some(_requirement) = requirement {
        // If the input hasn't changed, return
    }
    let unique_sources = request
        .locations
        .iter()
        .map(|location| location.h3_index.clone())
        .collect::<HashSet<String>>();
    let mut tiles_by_index: HashMap<(String, String), SpatialDistanceItem> = HashMap::new();
    for source in &unique_sources {
        let items = SpatialDistanceItem::list_by_source_from_db(&source, db)
            .await
            .map_err(map_error_to_response)?;
        for item in items {
            tiles_by_index.insert((source.clone(), item.destination_index.clone()), item);
        }
    }
    let mut map_tiles = vec![];
    let city_indices = h3
        .get_indices_for_city(&request.city_code)
        .map_err(map_error_to_response)?;
    for destination_index in city_indices {
        let mut duration = i32::MAX;
        for location in &request.locations {
            let source_index = location.h3_index.clone();
            if source_index == destination_index {
                duration = 0;
                break;
            }
            let tile = tiles_by_index.get(&(source_index.clone(), destination_index.clone()));
            // if the distance is not found, it exceeds the tolerated duration
            let tile = match tile {
                Some(tile) => tile,
                None => continue,
            };
            duration = match request.travel_mode {
                TravelMode::Driving => duration.min(tile.duration_drive),
                TravelMode::Bicycling => duration.min(tile.duration_cycle),
                TravelMode::PublicTransport => duration.min(tile.duration_transit),
                TravelMode::Walking => duration.min(tile.duration_walk),
            }
        }
        let score = match duration {
            i32::MAX => 0,
            _ => 100 - 100.min((100 * duration) / request.tolerated_duration),
            // _ => 100 - 100.min((50 * duration) / request.tolerated_duration),
        };
        map_tiles.push(MapTile {
            h3_index: destination_index,
            score: score,
        });
    }
    let requirement = RequirementItem {
        city_code: request.city_code,
        requirement_id: request.requirement_id,
        map_tiles: map_tiles,
    };
    let transaction = requirement.save().map_err(map_error_to_response)?;
    state
        .db_client
        .write_single(transaction)
        .await
        .map_err(map_error_to_response)?;
    Ok(Json(RequirementResponse {}))
}

#[utoipa::path(
    post,
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
    Json(request): Json<MapRequest>,
) -> Result<Json<MapResponse>, (StatusCode, Json<ErrorResponse>)> {
    // If there are no requirements, get all the h3 indices and return
    if request.requirement_ids.is_empty() {
        let indices = state
            .h3_client
            .get_indices_for_city(&request.city_code)
            .map_err(map_error_to_response)?;
        let tile_responses: Vec<MapTileResponse> = indices
            .into_iter()
            .map(|index| {
                Ok(MapTileResponse {
                    h3_index: index.to_string(),
                    mean_score: 0,
                    requirement_scores: vec![],
                })
            })
            .collect::<Result<Vec<_>, Error>>()
            .map_err(map_error_to_response)?;
        return Ok(Json(MapResponse {
            tiles: tile_responses,
        }));
    }

    // Load the requirements from the database
    let mut requirements = vec![];
    for requirement_id in &request.requirement_ids {
        let requirement = RequirementItem::from_db(requirement_id, &*state.db_client)
            .await
            .map_err(map_error_to_response)?;
        if let Some(requirement) = requirement {
            requirements.push(requirement);
        } else {
            return Err(map_error_to_response("Requirement not found"));
        }
    }

    // Aggregate the scores of the tiles from all requirements
    let mut tile_scores: HashMap<String, Vec<RequirementScoreResponse>> = HashMap::new();
    for requirement in &requirements {
        for tile in &requirement.map_tiles {
            tile_scores
                .entry(tile.h3_index.clone())
                .or_insert_with(Vec::new)
                .push(RequirementScoreResponse {
                    requirement_id: requirement.requirement_id,
                    score: tile.score,
                });
        }
    }

    // Calculate the mean score for each tile
    let mut map_tile_response = vec![];
    for (h3_index, scores) in tile_scores {
        // let mean_score = scores.iter().map(|s| s.score).sum::<i32>() as f32 / scores.len() as f32;
        let min_score = scores.iter().map(|s| s.score).min().unwrap_or(0);
        map_tile_response.push(MapTileResponse {
            h3_index,
            mean_score: min_score,
            // mean_score: mean_score as i32,
            requirement_scores: scores,
        });
    }
    Ok(Json(MapResponse {
        tiles: map_tile_response,
    }))
}
