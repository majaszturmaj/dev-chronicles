#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod commands;
mod db;
mod sanitizer;
mod server;
mod state;

use std::{error::Error, str::FromStr, time::Duration};

use ai::client::AiClient;
use db::init_db;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    SqlitePool,
};
use tauri::{async_runtime, Manager};

const SERVER_PORT: u16 = 3030;
const DB_FILENAME: &str = "activity_logs.db";

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_logs_by_date,
            commands::get_ai_reports,
            commands::trigger_manual_summary,
            commands::fetch_ai_settings,
            commands::save_ai_settings,
            commands::test_ai_connection
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let data_dir = app_handle
                .path()
                .app_data_dir()
                .map_err(|_| "failed to resolve app data directory".to_string())?;

            std::fs::create_dir_all(&data_dir)?;

            let db_path = data_dir.join(DB_FILENAME);
            let db_url = format!("sqlite://{}", db_path.to_string_lossy());

            let pool = create_pool(&db_url)?;
            async_runtime::block_on(async { init_db(&pool).await })
                .map_err(|err| -> Box<dyn Error> { Box::new(err) })?;

            app.manage(pool.clone());
            app.manage(AiClient::new());

            // Build and start the Axum server
            let router = server::build_router(pool.clone());

            println!("🚀 Starting Axum ingestion server on port {}", SERVER_PORT);

            // Spawn server using axum::Server::bind + router.into_make_service()
            async_runtime::spawn(async move {
                if let Err(err) = start_server(router).await {
                    eprintln!("❌ Axum server error: {err}");
                }
            });

            // Start automatic scheduler (every 10 minutes)
            let pool_for_scheduler = pool.clone();
            let app_handle_for_scheduler = app_handle.clone();

            async_runtime::spawn(async move {
                use tokio::time::interval;

                let mut interval_timer = interval(Duration::from_secs(600)); // 10 minutes

                loop {
                    interval_timer.tick().await;

                    match auto_process_logs(&pool_for_scheduler, &app_handle_for_scheduler).await {
                        Ok(summary) => {
                            if !summary.is_empty() {
                                println!("✅ Auto-generated summary at {}", chrono::Utc::now());
                            }
                        }
                        Err(e) => eprintln!("⚠️  Auto-processing error: {}", e),
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_pool(database_url: &str) -> Result<SqlitePool, Box<dyn Error>> {
    let connect_options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));

    async_runtime::block_on(async {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await
    })
    .map_err(|err| -> Box<dyn Error> { Box::new(err) })
}

async fn start_server(router: axum::Router) -> Result<(), Box<dyn Error + Send + Sync>>
{
    let addr = format!("127.0.0.1:{}", SERVER_PORT);
    println!("🎧 Binding Axum server to {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("✅ Axum server successfully started on http://{}", addr);

    // into_make_service() is available on Router without type parameters
    axum::serve(listener, router.into_make_service())
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    Ok(())
}

async fn auto_process_logs(
    pool: &SqlitePool,
    app_handle: &tauri::AppHandle,
) -> Result<String, Box<dyn Error>> {
    use chrono::{Duration, Utc};
    use crate::db::models::{ActivityLog, ActivityLogRow, ActivityLogConversionError};

    let ai_client = app_handle.state::<AiClient>();

    let end = Utc::now();
    let start = end - Duration::minutes(10);

    // Only fetch unprocessed logs
    let rows = sqlx::query_as::<_, ActivityLogRow>(
        "SELECT id, source, payload, timestamp FROM activity_logs \
         WHERE is_processed = 0 AND timestamp >= ?1 AND timestamp <= ?2 \
         ORDER BY timestamp ASC",
    )
    .bind(start.to_rfc3339())
    .bind(end.to_rfc3339())
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        return Ok(String::new());
    }

    let logs: Vec<ActivityLog> = rows
        .into_iter()
        .map(ActivityLog::try_from)
        .collect::<Result<_, ActivityLogConversionError>>()
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.0)) as Box<dyn Error>)?;

    let summary = ai::generate_summary(pool, ai_client.inner(), logs).await?;

    // Save the summary
    sqlx::query("INSERT INTO ai_reports (summary, generated_at) VALUES (?1, ?2)")
        .bind(&summary)
        .bind(end.to_rfc3339())
        .execute(pool)
        .await?;

    // Mark logs as processed
    sqlx::query("UPDATE activity_logs SET is_processed = 1 WHERE timestamp >= ?1 AND timestamp <= ?2")
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .execute(pool)
        .await?;

    Ok(summary)
}

