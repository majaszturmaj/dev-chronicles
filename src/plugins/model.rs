use serde::Serialize;

#[derive(Serialize)]
pub struct HistoryEntry {
    pub url: String,
    pub title: Option<String>,
    pub visit_time: i64,
    pub browser: String,
}

#[derive(Serialize)]
pub struct HistoryDump {
    pub browser: String,
    pub entries: Vec<HistoryEntry>,
}
