/// Centralized logging service for Rust backend
use tracing::{info, error, warn, debug, trace, Level};
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt};
use std::path::Path;

const LOG_FILE: &str = "data/logs/rust_backend.log";

pub fn init_logging() {
    // Ensure log directory exists
    if let Some(parent) = Path::new(LOG_FILE).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create log directory");
    }
    
    // Create file appender
    let file_appender = tracing_appender::rolling::daily(parent, "rust_backend.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Create file layer
    let file_layer = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .with_writer(file_writer);
    
    // Create console layer
    let console_layer = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_file(true);
    
    // Create filter layer
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::new("info"))
        .add_directive(Level::INFO.into());
    
    // Initialize subscriber
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(file_layer)
        .with(console_layer)
        .init();
    
    info!("Rust backend logging initialized");
    info!("Log file: {}", LOG_FILE);
}

/// Log a message with context
pub fn log_with_context(
    level: Level,
    message: &str,
    context: &[(&str, String)]
) {
    match level {
        Level::ERROR => {
            let mut err = error!(message);
            for (key, value) in context {
                err = err.field(*key, value);
            }
        }
        Level::WARN => {
            let mut wrn = warn!(message);
            for (key, value) in context {
                wrn = wrn.field(*key, value);
            }
        }
        Level::INFO => {
            let mut inf = info!(message);
            for (key, value) in context {
                inf = inf.field(*key, value);
            }
        }
        Level::DEBUG => {
            let mut dbg = debug!(message);
            for (key, value) in context {
                dbg = dbg.field(*key, value);
            }
        }
        Level::TRACE => {
            let mut trc = trace!(message);
            for (key, value) in context {
                trc = trc.field(*key, value);
            }
        }
    }
}

/// Log service for structured logging
#[derive(Clone)]
pub struct LogService {
    service_name: String,
}

impl LogService {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
        }
    }
    
    pub fn info(&self, message: &str, context: Vec<(&str, String)>) {
        let mut log = info!(service = %self.service_name, message);
        for (key, value) in context {
            log = log.field(*key, value);
        }
    }
    
    pub fn error(&self, message: &str, context: Vec<(&str, String)>) {
        let mut log = error!(service = %self.service_name, message);
        for (key, value) in context {
            log = log.field(*key, value);
        }
    }
    
    pub fn warn(&self, message: &str, context: Vec<(&str, String)>) {
        let mut log = warn!(service = %self.service_name, message);
        for (key, value) in context {
            log = log.field(*key, value);
        }
    }
    
    pub fn debug(&self, message: &str, context: Vec<(&str, String)>) {
        let mut log = debug!(service = %self.service_name, message);
        for (key, value) in context {
            log = log.field(*key, value);
        }
    }
}

// Re-export for convenience
pub use tracing::{info, error, warn, debug, trace, Level};
pub use LogService;