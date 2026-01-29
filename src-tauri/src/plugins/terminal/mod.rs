pub mod model;
pub mod ingest;

use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/ingest/terminal", post(ingest::ingest_terminal))
}
