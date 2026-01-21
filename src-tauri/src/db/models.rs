use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityLog {
    pub id: i64,
    pub source: String,
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
    pub log_type: Option<String>,
    pub session_id: Option<String>,
    pub command: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub file_path: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct ActivityLogRow {
    pub id: i64,
    pub source: String,
    pub payload: String,
    pub timestamp: String,
    pub log_type: Option<String>,
    pub session_id: Option<String>,
    pub command: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub file_path: Option<String>,
}

#[derive(Debug)]
pub struct ActivityLogConversionError(pub String);

impl TryFrom<ActivityLogRow> for ActivityLog {
    type Error = ActivityLogConversionError;

    fn try_from(row: ActivityLogRow) -> Result<Self, Self::Error> {
        let payload: Value = serde_json::from_str(&row.payload)
            .map_err(|err| ActivityLogConversionError(err.to_string()))?;

        let parsed_timestamp = DateTime::parse_from_rfc3339(&row.timestamp)
            .map_err(|err| ActivityLogConversionError(err.to_string()))?;

        Ok(ActivityLog {
            id: row.id,
            source: row.source,
            payload,
            timestamp: parsed_timestamp.with_timezone(&Utc),
            log_type: row.log_type,
            session_id: row.session_id,
            command: row.command,
            url: row.url,
            domain: row.domain,
            title: row.title,
            file_path: row.file_path,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiReport {
    pub id: i64,
    pub summary: String,
    pub generated_at: DateTime<Utc>,
    pub log_count: Option<i64>,
    pub sources: Option<String>,
    pub session_id: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct AiReportRow {
    pub id: i64,
    pub summary: String,
    pub generated_at: String,
    pub log_count: Option<i64>,
    pub sources: Option<String>,
    pub session_id: Option<String>,
}

impl TryFrom<AiReportRow> for AiReport {
    type Error = ActivityLogConversionError;

    fn try_from(row: AiReportRow) -> Result<Self, Self::Error> {
        let parsed_timestamp = DateTime::parse_from_rfc3339(&row.generated_at)
            .map_err(|err| ActivityLogConversionError(err.to_string()))?;

        Ok(AiReport {
            id: row.id,
            summary: row.summary,
            generated_at: parsed_timestamp.with_timezone(&Utc),
            log_count: row.log_count,
            sources: row.sources,
            session_id: row.session_id,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiSettings {
    pub provider_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub temperature: Option<f32>,
    pub batch_size: Option<i64>,
    pub summary_frequency_min: Option<i64>,
}

#[derive(sqlx::FromRow)]
pub struct AiSettingsRow {
    pub provider_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
    pub temperature: Option<f32>,
    pub batch_size: Option<i64>,
    pub summary_frequency_min: Option<i64>,
}

impl From<AiSettingsRow> for AiSettings {
    fn from(row: AiSettingsRow) -> Self {
        Self {
            provider_url: row.provider_url,
            api_key: row.api_key,
            model_name: row.model_name,
            temperature: row.temperature,
            batch_size: row.batch_size,
            summary_frequency_min: row.summary_frequency_min,
        }
    }
}
