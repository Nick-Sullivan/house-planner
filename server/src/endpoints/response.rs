use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(ToSchema, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total_items: usize,
    pub total_pages: usize,
    pub current_page: usize,
    pub page_size: usize,
}
