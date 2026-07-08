/// Logging configuration for the application
use tracing_subscriber::{fmt, EnvFilter, prelude::*, reload};
use std::path::Path;

pub struct LoggingConfig {
    pub level: String,
    pub log_file: Option<String>,
    pub json_format: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_file: None,
            json_format: false,
        }
    }
}

impl LoggingConfig {
    pub fn from_env() -> Self {
        use std::env;
        Self {
            level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            log_file: env::var("RUST_LOG_FILE").ok(),
            json_format: env::var("RUST_LOG_JSON").map(|s| s == "true").unwrap_or(false),
        }
    }

    pub fn init(&self) {
        let filter = EnvFilter::new(self.level.clone());
        
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true);
        
        let subscriber = if self.json_format {
            fmt_layer.json().with_filter(filter)
        } else {
            fmt_layer.pretty().with_filter(filter)
        };
        
        if let Some(log_file) = &self.log_file {
            let file_appender = tracing_appender::rolling::daily(Path::new(log_file), "sdk-ia-rust.log");
            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
            subscriber.with_writer(non_blocking).init();
        } else {
            subscriber.init();
        }
        
        tracing::info!("Rust logging initialized with level: {}", self.level);
    }
}