use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestEvent<T> {
    pub source: String,
    pub timestamp: i64,
    pub payload: T,
}
