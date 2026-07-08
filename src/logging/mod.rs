/// Logging module
pub mod store;

// Re-export logging functions
pub use store::{add_log_entry, get_log_entries, clear_log_entries, LogEntry};

pub use self::{
    init_logging,
    log_request,
    log_error,
    log_db_operation,
    log_agent_operation,
    log_generation,
};

// Include the actual implementation
include!("logging.rs");