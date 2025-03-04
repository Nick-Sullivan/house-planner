use super::map;
use super::{house, state::AppState};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(utoipa::OpenApi)]
#[openapi(
    tags(
        (name = house::models::HOUSE_TAG, description = "House endpoints"),
        (name = map::models::MAP_TAG, description = "Map endpoints"),
    ),
    info(
        license(
            name = "MIT",
            identifier = "MIT"
        )
    )
)]
pub struct ApiDoc;

pub fn create_router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/houses", house::endpoints::router())
        .nest("/maps", map::endpoints::router())
}
