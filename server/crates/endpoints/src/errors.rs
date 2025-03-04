use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn map_error_to_response(error: impl ToString) -> (StatusCode, Json<ErrorResponse>) {
    println!("Error: {}", error.to_string());
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: error.to_string(),
        }),
    )
}
