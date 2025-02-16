use super::{house_endpoints, request::AppState};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[derive(utoipa::OpenApi)]
#[openapi(
    tags(
        (name = house_endpoints::HOUSE_TAG, description = "House endpoints")
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
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/houses", house_endpoints::router())
}
