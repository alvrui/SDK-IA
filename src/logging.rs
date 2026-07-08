/// Logging configuration for the application
use tracing_subscriber::{fmt, EnvFilter};
use tracing::{info, error, warn, debug, Level};

pub fn init_logging() {
    // Initialize tracing subscriber with file and console output
    let file_appender = tracing_appender::rolling::daily("data/logs", "rust-backend.log");
    
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .with_ansi(false)
        .finish();
    
    // Add file layer
    let file_layer = fmt::Layer::default()
        .with_writer(file_appender)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .boxed();
    
    // Add console layer
    let console_layer = fmt::Layer::default()
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .boxed();
    
    // Combine layers
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();
    
    info!("Logging initialized - Rust backend");
}

/// Log a request with details
pub fn log_request(method: &str, path: &str, status: u16, duration: std::time::Duration) {
    info!(
        target: "http",
        method = method,
        path = path,
        status = status,
        duration_ms = duration.as_millis(),
        "HTTP Request"
    );
}

/// Log an error
pub fn log_error(error: &str, context: &str) {
    error!(target: "error", error = error, context = context, "Application Error");
}

/// Log database operations
pub fn log_db_operation(operation: &str, table: &str, success: bool, duration: std::time::Duration) {
    if success {
        debug!(target: "database", operation, table, duration_ms = duration.as_millis(), "DB Operation");
    } else {
        warn!(target: "database", operation, table, duration_ms = duration.as_millis(), "DB Operation Failed");
    }
}