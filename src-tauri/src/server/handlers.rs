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
    
    // Extract normalized fields from payload based on source
    let (log_type, command, url, domain, title, file_path) = extract_fields(&body.source, &body.payload);
    
    // Generate or retrieve session ID (simplified: hourly sessions)
    let session_id = format!("session_{}", timestamp.format("%Y%m%d_%H"));
    
    insert_log(
        &state.pool,
        &body.source,
        &payload_text,
        timestamp.to_rfc3339(),
        &log_type,
        &session_id,
        &command,
        &url,
        &domain,
        &title,
        &file_path,
    )
    .await?;
    
    println!("✅ Successfully saved log to database");
    
    Ok(StatusCode::CREATED)
}

/// Extract normalized fields from payload based on source type
fn extract_fields(source: &str, payload: &Value) -> (String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) {
    match source {
        "terminal" => {
            let command = payload.get("command").and_then(|v| v.as_str()).map(|s| s.to_string());
            ("command".to_string(), command, None, None, None, None)
        }
        "browser" => {
            let url = payload.get("url").and_then(|v| v.as_str()).map(|s| s.to_string());
            let title = payload.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());
            let domain = url.as_ref().and_then(|u| {
                url::Url::parse(u).ok().and_then(|parsed| parsed.domain().map(|d| d.to_string()))
            });
            ("browse".to_string(), None, url, domain, title, None)
        }
        "vscode" => {
            let file_path = payload.get("file_path").and_then(|v| v.as_str()).map(|s| s.to_string());
            let language = payload.get("language").and_then(|v| v.as_str()).map(|s| s.to_string());
            ("file_edit".to_string(), None, None, None, language, file_path)
        }
        _ => ("unknown".to_string(), None, None, None, None, None),
    }
}

async fn insert_log(
    pool: &SqlitePool,
    source: &str,
    payload: &str,
    timestamp: String,
    log_type: &str,
    session_id: &str,
    command: &Option<String>,
    url: &Option<String>,
    domain: &Option<String>,
    title: &Option<String>,
    file_path: &Option<String>,
) -> Result<(), (StatusCode, String)> {
    println!("💾 Inserting into database: source={}, timestamp={}, type={}", source, timestamp, log_type);
    
    sqlx::query(
        "INSERT INTO activity_logs (source, payload, timestamp, log_type, session_id, command, url, domain, title, file_path) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
    )
    .bind(source)
    .bind(payload)
    .bind(timestamp)
    .bind(log_type)
    .bind(session_id)
    .bind(command)
    .bind(url)
    .bind(domain)
    .bind(title)
    .bind(file_path)
    .execute(pool)
    .await
    .map(|result| {
        println!("✅ Database insert successful, rows affected: {}", result.rows_affected());
    })
    .map_err(|err| {
        eprintln!("❌ Database error: {}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })
}
