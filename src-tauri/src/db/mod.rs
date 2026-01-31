use sqlx::SqlitePool;
use crate::db::models::{AiSettings, AiSettingsRow};
use std::fmt;
use std::error::Error;

pub mod models;

const SCHEMA: &str = include_str!("schema.sql");

#[derive(Debug)]
struct SimpleError(String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SimpleError {}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Remove semicolons that appear inside line comments to avoid accidental splits
    let sanitized: String = SCHEMA
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("--") {
                line.replace(";", "")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Split schema into individual statements and execute them separately
    for statement in sanitized.split(';') {
        let trimmed = statement.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Log the statement being executed for debugging if it fails
        if trimmed.len() > 120 {
            eprintln!("Executing schema statement (truncated): {}...", &trimmed[..120]);
        } else {
            eprintln!("Executing schema statement: {}", trimmed);
        }

        match sqlx::query(trimmed).execute(pool).await {
            Ok(_) => {}
            Err(err) => {
                let msg = err.to_string();
                eprintln!("Schema exec error: {}", msg);
                if msg.contains("near \"use\"") || msg.contains("syntax error") {
                    eprintln!("⚠️  Skipping schema statement due to parse error: {}", msg);
                    continue;
                } else {
                    return Err(err);
                }
            }
        }
    }

    // Migration: Add extension_send_interval_sec column if it's missing (for existing databases)
    let add_column_sql = "ALTER TABLE ai_settings ADD COLUMN extension_send_interval_sec REAL DEFAULT 5.0";
    match sqlx::query(add_column_sql).execute(pool).await {
        Ok(_) => eprintln!("✓ Added extension_send_interval_sec column to ai_settings"),
        Err(err) => {
            let msg = err.to_string();
            if msg.contains("duplicate column") || msg.contains("already exists") {
                eprintln!("✓ extension_send_interval_sec column already exists");
            } else {
                eprintln!("⚠️  Could not add extension_send_interval_sec column: {}", msg);
            }
        }
    }

    Ok(())
}

pub async fn get_ai_settings(pool: &SqlitePool) -> Result<AiSettings, sqlx::Error> {
    // Try to fetch with the new extension_send_interval_sec column; if it fails, fall back to without it
    let result = sqlx::query_as::<_, AiSettingsRow>(
        "SELECT provider_url, api_key, model_name, temperature, batch_size, summary_frequency_min, extension_send_interval_sec FROM ai_settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await;

    let row = match result {
        Ok(row) => row,
        Err(sqlx::Error::RowNotFound) => return Err(sqlx::Error::RowNotFound),
        Err(_) => {
            // Fallback: older database without extension_send_interval_sec; query without it
            eprintln!("⚠️  extension_send_interval_sec column not found, using fallback query");
            let fallback = sqlx::query_as::<_, AiSettingsRow>(
                "SELECT provider_url, api_key, model_name, temperature, batch_size, summary_frequency_min, CAST(5.0 AS REAL) as extension_send_interval_sec FROM ai_settings WHERE id = 1"
            )
            .fetch_one(pool)
            .await?;
            fallback
        }
    };

    let settings = AiSettings::from(row);
    Ok(settings)
}

pub async fn upsert_ai_settings(
    pool: &SqlitePool,
    provider_url: &str,
    api_key: Option<&str>,
    model_name: &str,
    extension_send_interval_sec: Option<f32>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO ai_settings (id, provider_url, api_key, model_name, extension_send_interval_sec) 
         VALUES (1, ?1, ?2, ?3, ?4) 
         ON CONFLICT(id) DO UPDATE SET 
            provider_url = excluded.provider_url, 
            api_key = excluded.api_key,
            model_name = excluded.model_name,
            extension_send_interval_sec = excluded.extension_send_interval_sec" 
    )
    .bind(provider_url)
    .bind(api_key)
    .bind(model_name)
    .bind(extension_send_interval_sec)
    .execute(pool)
    .await?;

    Ok(())
}

