// Configuration module for SDK-IA

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub python_service_url: String,
    pub database_path: String,
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1".to_string(),
            server_port: 9090,
            python_service_url: "http://127.0.0.1:9000".to_string(),
            database_path: "data/agents.db".to_string(),
            log_level: "info".to_string(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("server_host", "127.0.0.1")?
            .set_default("server_port", 9090)?
            .set_default("python_service_url", "http://127.0.0.1:9000")?
            .set_default("database_path", "data/agents.db")?
            .set_default("log_level", "info")?;

        // Try to load from environment variables with SDK_IA_ prefix
        for (key, value) in env::vars() {
            if key.starts_with("SDK_IA_") {
                let config_key = key.trim_start_matches("SDK_IA_").to_lowercase();
                builder = builder.set_override(config_key, value)?;
            }
        }

        let settings = builder.build()?;
        Ok(settings.try_deserialize()?)
    }
}