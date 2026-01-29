use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TerminalEvent {
    pub command: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub cwd: String,
    pub shell: String,
}
