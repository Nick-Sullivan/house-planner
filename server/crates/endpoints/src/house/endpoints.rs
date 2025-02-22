use crate::response::{ErrorResponse, PaginatedResponse};
use crate::request::{PaginationParams, AppState};
use axum::extract::{State, Query, Path};
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use super::models::{HouseResponse, HOUSE_TAG};
use tokio::time::{sleep, Duration};

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
    .routes(utoipa_axum::routes!(get_houses))
    .routes(utoipa_axum::routes!(get_house_by_id))
}

#[utoipa::path(
    get, 
    path = "", 
    tag = HOUSE_TAG,
    params(
        ("page" = Option<usize>, Query, description = "Page number"),
        ("page_size" = Option<usize>, Query, description = "Number of items per page")
    ),
    responses(
        (status = OK, body = PaginatedResponse<HouseResponse>), 
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
pub async fn get_houses(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<HouseResponse>>, (StatusCode, Json<ErrorResponse>)> {
    sleep(Duration::from_secs(10)).await;
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);
    let num_houses = state.house_client.get_num_houses();
    match state.house_client.get_houses(page, page_size) {
        Ok(houses) => {
            let response = PaginatedResponse{
                items: houses.into_iter().map(HouseResponse::from).collect(),
                total_items: num_houses,
                total_pages: (num_houses + page_size - 1) / page_size,
                current_page: page,
                page_size,
            };
            Ok(Json(response))
        }
        Err(e) => {
            let response = ErrorResponse {
                error: e.to_string(),
            };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
        }
    }
}


#[utoipa::path(
    get, 
    path = "/{id}", 
    tag = HOUSE_TAG,
    params(
        ("id" = i32, Path, description = "House ID")
    ),
    responses(
        (status = OK, body = HouseResponse), 
        (status = NOT_FOUND, body = ErrorResponse, description = "House not found"),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
pub async fn get_house_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<HouseResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.house_client.get_house_by_id(id) {
        Ok(house) => Ok(Json(HouseResponse::from(house))),
        Err(e) => {
            let response = ErrorResponse {
                error: e.to_string(),
            };
            if e.to_string().contains("not found") {
                Err((StatusCode::NOT_FOUND, Json(response)))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
            }
        }
    }
}