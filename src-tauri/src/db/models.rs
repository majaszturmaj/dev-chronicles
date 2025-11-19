use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityLog {
    pub id: i64,
    pub source: String,
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct ActivityLogRow {
    pub id: i64,
    pub source: String,
    pub payload: String,
    pub timestamp: String,
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
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiReport {
    pub id: i64,
    pub summary: String,
    pub generated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct AiReportRow {
    pub id: i64,
    pub summary: String,
    pub generated_at: String,
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
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiSettings {
    pub provider_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
}

#[derive(sqlx::FromRow)]
pub struct AiSettingsRow {
    pub provider_url: String,
    pub api_key: Option<String>,
    pub model_name: String,
}

impl From<AiSettingsRow> for AiSettings {
    fn from(row: AiSettingsRow) -> Self {
        Self {
            provider_url: row.provider_url,
            api_key: row.api_key,
            model_name: row.model_name,
        }
    }
}
