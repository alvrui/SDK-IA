# Logging Module

This module provides centralized logging for the Rust backend.

## Usage

Add to your main.rs:

mod logging;
use logging::{init_logging, log_request, log_error};

fn main() {
    init_logging();
    log_request("GET", "/api/test", 200, std::time::Duration::from_millis(100));
    log_error("Something went wrong", "test context");
}

## Features

- File logging to data/logs/rust-backend.log with daily rotation
- Console logging
- In-memory log store for API access
- Structured logging with context

## Note

If you have a file named src/logging.rs, it must be deleted as it conflicts with this directory.
Rust does not allow both a file and a directory with the same name.