// Python service client for internal communication
// This module provides HTTP client functionality to call the Python Secretario service

use reqwest::{Client, Response};
use serde_json::Value;
use std::time::Duration;
use thiserror::Error;

use crate::config::AppConfig;

/// Error type for Python service communication
#[derive(Error, Debug)]
pub enum PythonClientError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    
    #[error("Serde JSON error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/// Client for communicating with the Python Secretario service
pub struct PythonClient {
    client: Client,
    base_url: String,
}

impl PythonClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }
    
    pub fn from_config(config: AppConfig) -> Self {
        Self::new(&config.python_service_url)
    }
    
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    
    pub async fn get(&self, endpoint: &str) -> Result<Value, PythonClientError> {
        let url = format!("{}/api/v1/internal/{}", self.base_url, endpoint);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(PythonClientError::ReqwestError)?;
        self.handle_response(response, &url).await
    }
    
    pub async fn post(&self, endpoint: &str, body: Value) -> Result<Value, PythonClientError> {
        let url = format!("{}/api/v1/internal/{}", self.base_url, endpoint);
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(PythonClientError::ReqwestError)?;
        self.handle_response(response, &url).await
    }
    
    pub async fn put(&self, endpoint: &str, body: Value) -> Result<Value, PythonClientError> {
        let url = format!("{}/api/v1/internal/{}", self.base_url, endpoint);
        let response = self.client.put(&url)
            .json(&body)
            .send()
            .await
            .map_err(PythonClientError::ReqwestError)?;
        self.handle_response(response, &url).await
    }
    
    pub async fn delete(&self, endpoint: &str) -> Result<Value, PythonClientError> {
        let url = format!("{}/api/v1/internal/{}", self.base_url, endpoint);
        let response = self.client.delete(&url)
            .send()
            .await
            .map_err(PythonClientError::ReqwestError)?;
        self.handle_response(response, &url).await
    }
    
    async fn handle_response(&self, response: Response, url: &str) -> Result<Value, PythonClientError> {
        let status = response.status();
        if status.is_success() {
            let json: Value = response.json()
                .await
                .map_err(PythonClientError::ReqwestError)?;
            Ok(json)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(PythonClientError::RequestFailed(format!(
                "HTTP {} from {}: {}",
                status, url, error_text
            )))
        }
    }
    
    pub async fn get_agents(&self) -> Result<Value, PythonClientError> {
        self.get("agents").await
    }
    
    pub async fn get_agent(&self, agent_id: &str) -> Result<Value, PythonClientError> {
        self.get(&format!("agents/{}", agent_id)).await
    }
    
    pub async fn create_agent(&self, agent_data: Value) -> Result<Value, PythonClientError> {
        self.post("agents", agent_data).await
    }
    
    pub async fn update_agent(&self, agent_id: &str, agent_data: Value) -> Result<Value, PythonClientError> {
        self.put(&format!("agents/{}", agent_id), agent_data).await
    }
    
    pub async fn delete_agent(&self, agent_id: &str) -> Result<Value, PythonClientError> {
        self.delete(&format!("agents/{}", agent_id)).await
    }
    
    pub async fn send_message(&self, agent_id: &str, message: &str) -> Result<Value, PythonClientError> {
        let body = serde_json::json!({"content": message});
        self.post(&format!("agents/{}/messages", agent_id), body).await
    }
    
    pub async fn send_message_with_conversation(
        &self,
        agent_id: &str,
        conversation_id: Option<&str>,
        message: &str,
    ) -> Result<Value, PythonClientError> {
        let mut body = serde_json::json!({"content": message});
        if let Some(conv_id) = conversation_id {
            body["conversation_id"] = Value::String(conv_id.to_string());
        }
        self.post(&format!("agents/{}/messages", agent_id), body).await
    }
    
    pub async fn health_check(&self) -> Result<Value, PythonClientError> {
        self.get("health").await
    }
    
    pub async fn get_logs(&self) -> Result<Value, PythonClientError> {
        self.get("logs").await
    }
    
    pub async fn get_agentes_legacy(&self) -> Result<Value, PythonClientError> {
        let url = format!("{}/agentes", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(PythonClientError::ReqwestError)?;
        self.handle_response(response, &url).await
    }
    
    pub async fn enviar_mensaje_legacy(
        &self,
        agente: &str,
        mensaje: &str,
    ) -> Result<Value, PythonClientError> {
        let url = format!("{}/enviar_mensaje", self.base_url);
        let body = serde_json::json!({"agente": agente, "mensaje": mensaje});
        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(PythonClientError::ReqwestError)?;
        self.handle_response(response, &url).await
    }
}