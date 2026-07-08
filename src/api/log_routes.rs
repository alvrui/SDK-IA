use actix_web::{web, get, HttpResponse, Responder};
use std::sync::Arc;
use serde_json::{json, Value};

use crate::logging;

/// Log entry structure for API
#[derive(serde::Serialize)]
struct LogEntry {
    level: String,
    message: String,
    timestamp: String,
    context: Value,
}

/// Simple in-memory log store for Rust backend
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG_STORE: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());
}

/// Add a log entry to the store
pub fn add_log_entry(level: &str, message: &str, context: Value) {
    let entry = LogEntry {
        level: level.to_string(),
        message: message.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        context,
    };
    let mut store = LOG_STORE.lock().unwrap();
    store.push(entry);
    if store.len() > 1000 {
        store.drain(0..store.len() - 1000);
    }
}

/// Get log entries
#[get("/api/v1/internal/logs")]
async fn get_logs(
    query: web::Query<(Option<String>, Option<usize>)>,
) -> impl Responder {
    let (level, limit) = query.into_inner();
    let limit = limit.unwrap_or(100);
    
    let store = LOG_STORE.lock().unwrap();
    let entries: Vec<LogEntry> = store.iter()
        .filter(|e| level.as_ref().map_or(true, |l| e.level == *l))
        .rev()
        .take(limit)
        .cloned()
        .collect();
    
    HttpResponse::Ok().json(json!({
        "status": "success",
        "data": entries,
        "meta": {
            "total": entries.len(),
            "limit": limit
        }
    }))
}

/// Clear log entries
#[get("/api/v1/internal/logs/clear")]
async fn clear_logs() -> impl Responder {
    let mut store = LOG_STORE.lock().unwrap();
    store.clear();
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Logs cleared"
    }))
}

/// Configure routes
pub fn config_log_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_logs)
       .service(clear_logs);
}