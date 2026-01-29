use serde::Serialize;

#[derive(Serialize)]
pub struct BrowserHistoryEntry {
    pub url: String,
    pub title: Option<String>,
    pub visit_time: i64,
    pub browser: String,
}

#[derive(Serialize)]
pub struct BrowserHistorySnapshot {
    pub browser: String,
    pub entries: Vec<BrowserHistoryEntry>,
}
