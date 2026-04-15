use axum::Router;
use axum::http::HeaderValue;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

use crate::AppState;

pub mod auth;
pub mod health;
pub mod user;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", user::routes())
        .merge(health::routes())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
}
