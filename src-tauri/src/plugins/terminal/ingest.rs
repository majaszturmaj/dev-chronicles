use axum::Json;
use crate::ingestion::models::IngestEvent;
use crate::plugins::terminal::model::TerminalEvent;

pub async fn ingest_terminal(
    Json(event): Json<IngestEvent<TerminalEvent>>
) -> Result<(), String> {

    // tu: zapis do SQLite / log / pipeline
    println!(
        "[TERMINAL] {} | {}",
        event.payload.shell,
        event.payload.command
    );

    Ok(())
}
