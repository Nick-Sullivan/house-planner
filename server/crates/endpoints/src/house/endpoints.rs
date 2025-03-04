use super::models::{HouseRequestFilter, HouseResponse, HOUSE_TAG};
use crate::errors::{map_error_to_response, ErrorResponse};
use crate::pagination::{PaginatedResponse, PaginationParams};
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use database::house_item::HouseItem;
use database::paginated_models::serialise_db_key;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(utoipa_axum::routes!(get_houses))
    // .routes(utoipa_axum::routes!(get_house_by_id))
}

// #[utoipa::path(
//     get,
//     path = "",
//     tag = HOUSE_TAG,
//     params(
//         ("page" = Option<usize>, Query, description = "Page number"),
//         ("page_size" = Option<usize>, Query, description = "Number of items per page")
//     ),
//     responses(
//         (status = OK, body = PaginatedResponse<HouseResponse>),
//         (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
//     )
// )]
// pub async fn get_houses(
//     State(state): State<Arc<AppState>>,
//     Query(pagination): Query<PaginationParams>,
// ) -> Result<Json<PaginatedResponse<HouseResponse>>, (StatusCode, Json<ErrorResponse>)> {
//     let page = pagination.page.unwrap_or(1);
//     let page_size = pagination.page_size.unwrap_or(10);
//     let num_houses = state.house_client.get_num_houses();
//     match state.house_client.get_houses(page, page_size) {
//         Ok(houses) => {
//             let response = PaginatedResponse{
//                 items: houses.into_iter().map(HouseResponse::from).collect(),
//                 total_items: num_houses,
//                 total_pages: (num_houses + page_size - 1) / page_size,
//                 current_page: page,
//                 page_size,
//             };
//             Ok(Json(response))
//         }
//         Err(e) => {
//             let response = ErrorResponse {
//                 error: e.to_string(),
//             };
//             Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
//         }
//     }
// }

#[utoipa::path(
    get,
    path = "",
    tag = HOUSE_TAG,
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of items to return"),
        ("last_evaluated_key" = Option<String>, Query, description = "Last evaluated key from previous response"),
        ("city_code" = Option<String>, Query, description = "City code to filter houses"),
        ("h3_index" = Option<String>, Query, description = "H3 geospatial index to filter houses")
    ),
    responses(
        (status = OK, body = PaginatedResponse<HouseResponse>),
        (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
    )
)]
pub async fn get_houses(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<PaginationParams>,
    Query(filter): Query<HouseRequestFilter>,
) -> Result<Json<PaginatedResponse<HouseResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let db = &*state.db_client;
    if filter.h3_index.is_some() && filter.city_code.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "h3_index and city_code can't both be provided".to_string(),
            }),
        ));
    }
    let db_response = if let Some(h3_index) = &filter.h3_index {
        HouseItem::list_by_h3_index_from_db(
            h3_index.as_str(),
            pagination.limit,
            pagination
                .decode_last_evaluated_key()
                .map_err(map_error_to_response)?,
            db,
        )
        .await
        .map_err(map_error_to_response)?
    } else if let Some(city_code) = &filter.city_code {
        HouseItem::list_by_city_from_db(
            city_code.as_str(),
            pagination.limit,
            pagination
                .decode_last_evaluated_key()
                .map_err(map_error_to_response)?,
            db,
        )
        .await
        .map_err(map_error_to_response)?
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Either h3_index or city_code must be provided".to_string(),
            }),
        ));
    };
    let response = PaginatedResponse {
        items: db_response
            .items
            .into_iter()
            .map(HouseResponse::from)
            .collect(),
        last_evaluated_key: match db_response.last_evaluated_key {
            Some(key) => Some(serialise_db_key(key).map_err(map_error_to_response)?),
            None => None,
        },
    };
    Ok(Json(response))
}

// #[utoipa::path(
//     get,
//     path = "/{id}",
//     tag = HOUSE_TAG,
//     params(
//         ("id" = i32, Path, description = "House ID")
//     ),
//     responses(
//         (status = OK, body = HouseResponse),
//         (status = NOT_FOUND, body = ErrorResponse, description = "House not found"),
//         (status = INTERNAL_SERVER_ERROR, body = ErrorResponse, description = "Internal server error")
//     )
// )]
// pub async fn get_house_by_id(
//     State(state): State<Arc<AppState>>,
//     Path(id): Path<i32>,
// ) -> Result<Json<HouseResponse>, (StatusCode, Json<ErrorResponse>)> {
//     match state.house_client.get_house_by_id(id) {
//         Ok(house) => Ok(Json(HouseResponse::from(house))),
//         Err(e) => {
//             let response = ErrorResponse {
//                 error: e.to_string(),
//             };
//             if e.to_string().contains("not found") {
//                 Err((StatusCode::NOT_FOUND, Json(response)))
//             } else {
//                 Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)))
//             }
//         }
//     }
// }
