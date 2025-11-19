use std::convert::TryFrom;

use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use tauri::State;

use crate::{
    ai::{client::AiClient, generate_summary},
    db::{
        get_ai_settings as load_ai_settings,
        models::{
            ActivityLog, ActivityLogConversionError, ActivityLogRow, AiReport, AiReportRow,
            AiSettings,
        },
        upsert_ai_settings,
    },
};

#[tauri::command]
pub async fn get_logs_by_date(
    date: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ActivityLog>, String> {
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|err| format!("invalid date format: {err}"))?;

    let start = DateTime::<Utc>::from_naive_utc_and_offset(
        parsed_date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| "failed to construct start time".to_string())?,
        Utc,
    );
    let end = start + Duration::days(1);

    let rows = sqlx::query_as::<_, ActivityLogRow>(
        "SELECT id, source, payload, timestamp FROM activity_logs \
         WHERE timestamp >= ?1 AND timestamp < ?2 \
         ORDER BY timestamp DESC",
    )
    .bind(start.to_rfc3339())
    .bind(end.to_rfc3339())
    .fetch_all(pool.inner())
    .await
    .map_err(|err| err.to_string())?;

    rows
        .into_iter()
        .map(ActivityLog::try_from)
        .collect::<Result<Vec<_>, ActivityLogConversionError>>()
        .map_err(|err| err.0)
}

#[tauri::command]
pub async fn get_ai_reports(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AiReport>, String> {
    sqlx::query_as::<_, AiReportRow>(
        "SELECT id, summary, generated_at FROM ai_reports ORDER BY generated_at DESC LIMIT 20",
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|err| err.to_string())?
    .into_iter()
    .map(AiReport::try_from)
    .collect::<Result<Vec<_>, ActivityLogConversionError>>()
    .map_err(|err| err.0)
}

#[tauri::command]
pub async fn trigger_manual_summary(
    pool: State<'_, SqlitePool>,
    ai_client: State<'_, AiClient>,
) -> Result<String, String> {
    let end = Utc::now();
    let start = end - Duration::minutes(15);

    let rows = sqlx::query_as::<_, ActivityLogRow>(
        "SELECT id, source, payload, timestamp FROM activity_logs \
         WHERE timestamp >= ?1 AND timestamp <= ?2 \
         ORDER BY timestamp ASC",
    )
    .bind(start.to_rfc3339())
    .bind(end.to_rfc3339())
    .fetch_all(pool.inner())
    .await
    .map_err(|err| err.to_string())?;

    let logs = rows
        .into_iter()
        .map(ActivityLog::try_from)
        .collect::<Result<Vec<_>, ActivityLogConversionError>>()
        .map_err(|err| err.0)?;

    if logs.is_empty() {
        return Err("No logs found in the last 15 minutes".to_string());
    }

    let summary = generate_summary(pool.inner(), ai_client.inner(), logs)
        .await
        .map_err(|err| err.to_string())?;

    insert_ai_report(pool.inner(), &summary, end)
        .await
        .map_err(|err| err.to_string())?;

    Ok(summary)
}

#[derive(Debug, Deserialize)]
pub struct SaveAiSettingsPayload {
    pub provider_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
}

#[tauri::command]
pub async fn save_ai_settings(
    pool: State<'_, SqlitePool>,
    settings: SaveAiSettingsPayload,
) -> Result<(), String> {
    let provider_url = settings.provider_url.trim();

    if provider_url.is_empty() {
        return Err("Provider URL must not be empty".to_string());
    }

    let api_key = settings
        .api_key
        .and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });

    let model_name = settings.model_name.trim();
    if model_name.is_empty() {
        return Err("Model name must not be empty".to_string());
    }

    upsert_ai_settings(pool.inner(), provider_url, api_key.as_deref(), model_name)
        .await
        .map_err(|err| err.to_string())
}

#[derive(Debug, Serialize)]
pub struct AiSettingsResponse {
    pub provider_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
}

#[tauri::command]
pub async fn fetch_ai_settings(
    pool: State<'_, SqlitePool>,
) -> Result<AiSettingsResponse, String> {
    let settings: AiSettings = load_ai_settings(pool.inner())
        .await
        .map_err(|err| err.to_string())?;

    Ok(AiSettingsResponse {
        provider_url: settings.provider_url,
        api_key: settings.api_key,
        model_name: settings.model_name,
    })
}
#[tauri::command]
pub async fn test_ai_connection(
    pool: State<'_, SqlitePool>,
    ai_client: State<'_, AiClient>,
) -> Result<String, String> {
    use serde_json::json;

    let settings = load_ai_settings(pool.inner())
        .await
        .map_err(|err| format!("Failed to load settings: {err}"))?;

    // Determine model name based on provider URL
    let model_name = if settings.provider_url.contains("openai.com") {
        "gpt-3.5-turbo"
    } else {
        "llama-3-8b-instruct"
    };

    // Send a minimal test request
    let test_payload = json!({
        "model": model_name,
        "messages": [
            {
                "role": "user",
                "content": "Say 'Connection successful' if you can read this."
            }
        ],
        "max_tokens": 10,
        "temperature": 0.1
    });

    let response = ai_client
        .inner()
        .send_chat_completion(&settings, &test_payload)
        .await
        .map_err(|err| format!("Connection failed: {err}"))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!(
            "API returned error status {status}: {error_text}"
        ));
    }

    // Try to parse response to verify it's valid
    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|err| format!("Failed to parse response: {err}"))?;

    if response_json.get("choices").and_then(|c| c.as_array()).is_some() {
        Ok(format!(
            "✓ Connection successful! API responded with status {status}."
        ))
    } else {
        Err("API responded but response format is unexpected".to_string())
    }
}

async fn insert_ai_report(
    pool: &SqlitePool,
    summary: &str,
    generated_at: DateTime<Utc>,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "INSERT INTO ai_reports (summary, generated_at) VALUES (?1, ?2)",
    )
    .bind(summary)
    .bind(generated_at.to_rfc3339())
    .execute(pool)
    .await
}
