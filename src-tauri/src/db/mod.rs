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
    // Split schema into individual statements and execute them separately
    for statement in SCHEMA.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sqlx::query(trimmed)
                .execute(pool)
                .await?;
        }
    }

    Ok(())
}

pub async fn get_ai_settings(pool: &SqlitePool) -> Result<AiSettings, sqlx::Error> {
    let row = sqlx::query_as::<_, AiSettingsRow>(
        "SELECT provider_url, api_key, model_name, temperature, batch_size, summary_frequency_min FROM ai_settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let settings = AiSettings::from(row);

    Ok(settings)
}

pub async fn upsert_ai_settings(
    pool: &SqlitePool,
    provider_url: &str,
    api_key: Option<&str>,
    model_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO ai_settings (id, provider_url, api_key, model_name) 
         VALUES (1, ?1, ?2, ?3) 
         ON CONFLICT(id) DO UPDATE SET 
            provider_url = excluded.provider_url, 
            api_key = excluded.api_key,
            model_name = excluded.model_name" 
    )
    .bind(provider_url)
    .bind(api_key)
    .bind(model_name)  
    .execute(pool)
    .await?;

    Ok(())
}

