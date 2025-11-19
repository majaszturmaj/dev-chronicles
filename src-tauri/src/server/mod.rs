use axum::{routing::post, Router};
use axum::routing::get;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

use crate::state::AppState;

pub mod handlers;

pub fn build_router(pool: sqlx::SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/ingest/terminal", post(handlers::ingest))
        .route("/ingest/vscode", post(handlers::ingest))
        .route("/ingest/browser", post(handlers::ingest))
        .route("/health", get(|| async { "OK" }))  // Health check
        .with_state(AppState { pool })
        .layer(cors)
}
