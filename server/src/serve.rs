#![allow(dead_code)]
mod endpoints;
mod houses;
mod maps;
mod planner;

use axum::body::Body;
use axum::Router;
use endpoints::request::AppState;
use houses::house_client::HouseClient;
use hyper::Request;
use std::error::Error;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut house_client = HouseClient::new();
    house_client.load_data()?;
    let app_state = Arc::new(AppState { house_client });

    let trace_layer =
        TraceLayer::new_for_http().on_request(|request: &Request<Body>, _: &tracing::Span| {
            tracing::info!(
                method = %request.method(),
                uri = %request.uri(),
                headers = ?request.headers(),
                message = "begin request!"
            )
        });

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let (router, api) = endpoints::router::create_router().split_for_parts();
    let router = router.with_state(app_state.clone());

    let app = Router::new()
        .with_state(app_state)
        .merge(router)
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", api))
        .layer(trace_layer)
        .layer(cors_layer);

    // One-shot when invoked from API Gateway
    #[cfg(feature = "api_gateway_trigger")]
    {
        let lambda_app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);
        lambda_http::run(lambda_app).await?;
    }

    // Run a server that listens for requests for local development
    #[cfg(feature = "local_trigger")]
    {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    Ok(())
}

// async fn handle_hello() -> impl IntoResponse {
//     let data = "Hello!";
//     (StatusCode::OK, data).into_response()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::body::to_bytes;

//     #[tokio::test]
//     async fn test_handle_hello_responds() {
//         let response = handle_hello().await.into_response();
//         assert_eq!(response.status(), StatusCode::OK);
//         let body = response.into_body();
//         let body_bytes = to_bytes(body, usize::MAX).await.unwrap();
//         let body_str = std::str::from_utf8(&body_bytes).unwrap();
//         assert_eq!(body_str, "Hello!");
//     }
// }
