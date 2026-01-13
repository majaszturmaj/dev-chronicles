use axum::Json;
use crate::plugins::browser_history::{plugin, paths::Browser};

pub async fn ingest() -> Result<Json<serde_json::Value>, String> {
    let data = plugin::collect(Browser::Chrome).await?;
    Ok(Json(serde_json::to_value(data).unwrap()))
}
