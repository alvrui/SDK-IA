use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Agent status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Active,
    Inactive,
    Maintenance,
    Disabled,
}

impl Default for AgentStatus {
    fn default() -> Self {
        AgentStatus::Active
    }
}

/// Agent model (Mistral models)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentModel {
    MistralTiny,
    MistralSmall,
    MistralMedium,
    MistralLarge,
    Codestral,
    Mixtral8x7b,
    Mixtral8x22b,
    Custom(String),
}

impl Default for AgentModel {
    fn default() -> Self {
        AgentModel::MistralSmall
    }
}

/// Agent entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub model: AgentModel,
    pub system_prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub status: AgentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl Agent {
    pub fn new(
        id: String,
        name: String,
        description: Option<String>,
        model: AgentModel,
        system_prompt: String,
        temperature: f32,
        max_tokens: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            model,
            system_prompt,
            temperature,
            max_tokens,
            status: AgentStatus::Active,
            created_at: now,
            updated_at: now,
            metadata: serde_json::Value::Null,
        }
    }
}

/// Agent creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCreate {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub model: AgentModel,
    pub system_prompt: String,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    4096
}

impl Default for AgentCreate {
    fn default() -> Self {
        Self {
            name: "New Agent".to_string(),
            description: None,
            model: AgentModel::default(),
            system_prompt: "You are a helpful AI assistant.".to_string(),
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
        }
    }
}

/// Agent update request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub model: Option<AgentModel>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub status: Option<AgentStatus>,
    pub metadata: Option<serde_json::Value>,
}

/// Message entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: String,
    pub agent_id: String,
    pub conversation_id: Option<String>,
    pub content: String,
    pub role: String, // "user" or "assistant"
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Message creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessageCreate {
    pub content: String,
    pub conversation_id: Option<String>,
}

/// Conversation entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConversation {
    pub id: String,
    pub agent_id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: u32,
    pub metadata: serde_json::Value,
}

/// List response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentListResponse {
    pub agents: Vec<Agent>,
    pub total: usize,
}

/// Health status for agent service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentServiceStatus {
    pub status: String,
    pub agent_count: usize,
    pub active_agents: usize,
    pub version: String,
}