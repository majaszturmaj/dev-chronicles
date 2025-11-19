use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;
use sqlx::SqlitePool;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct IngestRequest {
    pub source: String,
    pub payload: Value,
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
}

pub async fn ingest(
    State(state): State<AppState>,
    Json(body): Json<IngestRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("📥 Received ingestion request from source: {}", body.source);
    
    let timestamp = body.timestamp.unwrap_or_else(Utc::now);
    let payload_text = serde_json::to_string(&body.payload)
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
    
    println!("📝 Payload length: {} bytes", payload_text.len());
    insert_log(&state.pool, &body.source, &payload_text, timestamp.to_rfc3339()).await?;
    
    println!("✅ Successfully saved log to database");
    
    Ok(StatusCode::CREATED)
}

async fn insert_log(
    pool: &SqlitePool,
    source: &str,
    payload: &str,
    timestamp: String,
) -> Result<(), (StatusCode, String)> {

    println!("💾 Inserting into database: source={}, timestamp={}", source, timestamp);
    sqlx::query(
        "INSERT INTO activity_logs (source, payload, timestamp) VALUES (?1, ?2, ?3)",
    )
    .bind(source)
    .bind(payload)
    .bind(timestamp)
    .execute(pool)
    .await
    .map(|result| {
        println!("✅ Database insert successful, rows affected: {}", result.rows_affected());  // ✨ ADD THIS
    })
    .map_err(|err| {
        eprintln!("❌ Database error: {}", err);  // ✨ ADD THIS
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })
}
