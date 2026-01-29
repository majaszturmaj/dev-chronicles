pub mod model;
pub mod paths;
pub mod utils;
pub mod plugin;

use axum::{routing::post, Json, Router};
use crate::plugins::browser_history::paths::Browser;

async fn handler() -> Result<Json<model::BrowserHistorySnapshot>, String> {
    let data = plugin::collect(Browser::Chrome).await?;
    Ok(Json(data))
}

pub fn routes() -> Router {
    Router::new()
        .route("/ingest/browser/snapshot", post(handler))
}
