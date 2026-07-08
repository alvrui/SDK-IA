use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::Value;
use chrono::Utc;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub level: String,
    pub message: String,
    pub timestamp: String,
    pub context: Value,
}

lazy_static! {
    pub static ref LOG_STORE: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}

/// Add a log entry to the store
pub fn add_log_entry(level: &str, message: &str, context: Value) {
    let entry = LogEntry {
        level: level.to_string(),
        message: message.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        context,
    };
    let mut store = LOG_STORE.lock().unwrap();
    store.push(entry);
    if store.len() > 1000 {
        store.drain(0..store.len() - 1000);
    }
}

/// Get log entries from the store
pub fn get_log_entries(level: Option<&str>, limit: usize) -> Vec<LogEntry> {
    let store = LOG_STORE.lock().unwrap();
    let mut entries: Vec<LogEntry> = store.iter()
        .filter(|e| level.map_or(true, |l| e.level == l))
        .cloned()
        .collect();
    entries.truncate(limit);
    entries
}

/// Clear all log entries
pub fn clear_log_entries() {
    let mut store = LOG_STORE.lock().unwrap();
    store.clear();
}