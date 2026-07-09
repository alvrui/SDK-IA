// Agent API handlers
// Provides REST endpoints for managing Mistral AI agents
// Acts as a proxy to the Python Secretario service

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::domain::{Agent, AgentCreate, AgentUpdate, AgentMessage, AgentMessageCreate, AgentConversation, AgentListResponse, AgentServiceStatus};
use crate::services::python_client::PythonClient;
use crate::app_data::AppData;

/// Configuration function for agent routes
pub fn configure_agent_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/agents")
            .route("", web::get().to(list_agents))
            .route("", web::post().to(create_agent))
            .route("/{agent_id}", web::get().to(get_agent))
            .route("/{agent_id}", web::put().to(update_agent))
            .route("/{agent_id}", web::delete().to(delete_agent))
            .route("/{agent_id}/messages", web::post().to(send_agent_message))
            .route("/{agent_id}/conversations", web::get().to(list_agent_conversations))
            .route("/status", web::get().to(get_agent_service_status))
    );
}

// ============================================================================
// Agent CRUD Handlers
// ============================================================================

/// List all agents
async fn list_agents(data: web::Data<AppData>) -> impl Responder {
    match data.python_client.get_agents().await {
        Ok(agents_json) => {
            // Parse agents from JSON
            let agents: Vec<Agent> = match serde_json::from_value(agents_json) {
                Ok(a) => a,
                Err(_) => Vec::new(),
            };
            
            HttpResponse::Ok().json(AgentListResponse {
                agents: agents.clone(),
                total: agents.len(),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string(),
            "message": "Failed to list agents"
        })),
    }
}

/// Create a new agent
async fn create_agent(
    data: web::Data<AppData>,
    agent_data: web::Json<AgentCreate>,
) -> impl Responder {
    let agent_json = serde_json::to_value(&agent_data.into_inner()).unwrap();
    
    match data.python_client.create_agent(agent_json).await {
        Ok(agent_json) => {
            let agent: Agent = serde_json::from_value(agent_json).unwrap();
            HttpResponse::Created().json(agent)
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": e.to_string(),
            "message": "Failed to create agent"
        })),
    }
}

/// Get a specific agent by ID
async fn get_agent(
    data: web::Data<AppData>,
    agent_id: web::Path<String>,
) -> impl Responder {
    let agent_id_str = agent_id.into_inner();
    match data.python_client.get_agent(&agent_id_str).await {
        Ok(agent_json) => {
            let agent: Agent = serde_json::from_value(agent_json).unwrap();
            HttpResponse::Ok().json(agent)
        }
        Err(e) => HttpResponse::NotFound().json(json!({
            "error": e.to_string(),
            "message": "Agent not found"
        })),
    }
}

/// Update an existing agent
async fn update_agent(
    data: web::Data<AppData>,
    agent_id: web::Path<String>,
    agent_data: web::Json<AgentUpdate>,
) -> impl Responder {
    let agent_id_str = agent_id.into_inner();
    let agent_json = serde_json::to_value(&agent_data.into_inner()).unwrap();
    
    match data.python_client.update_agent(&agent_id_str, agent_json).await {
        Ok(agent_json) => {
            let agent: Agent = serde_json::from_value(agent_json).unwrap();
            HttpResponse::Ok().json(agent)
        }
        Err(e) => HttpResponse::NotFound().json(json!({
            "error": e.to_string(),
            "message": "Agent not found"
        })),
    }
}

/// Delete an agent
async fn delete_agent(
    data: web::Data<AppData>,
    agent_id: web::Path<String>,
) -> impl Responder {
    let agent_id_str = agent_id.into_inner();
    match data.python_client.delete_agent(&agent_id_str).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::NotFound().json(json!({
            "error": e.to_string(),
            "message": "Agent not found"
        })),
    }
}

// ============================================================================
// Agent Message Handlers
// ============================================================================

/// Send a message to an agent
async fn send_agent_message(
    data: web::Data<AppData>,
    agent_id: web::Path<String>,
    message_data: web::Json<AgentMessageCreate>,
) -> impl Responder {
    let agent_id_str = agent_id.into_inner();
    match data.python_client.send_message_with_conversation(
        &agent_id_str,
        message_data.conversation_id.as_deref(),
        &message_data.content,
    ).await {
        Ok(message_json) => {
            let message: AgentMessage = serde_json::from_value(message_json).unwrap();
            HttpResponse::Created().json(message)
        }
        Err(e) => HttpResponse::BadRequest().json(json!({
            "error": e.to_string(),
            "message": "Failed to send message"
        })),
    }
}

// ============================================================================
// Agent Conversation Handlers
// ============================================================================

/// List conversations for an agent
async fn list_agent_conversations(
    _data: web::Data<AppData>,
    _agent_id: web::Path<String>,
) -> impl Responder {
    // For now, return conversations from storage
    // This would need to be implemented in PythonClient
    HttpResponse::Ok().json(Vec::<AgentConversation>::new())
}

// ============================================================================
// Service Status Handler
// ============================================================================

/// Get agent service status
async fn get_agent_service_status(data: web::Data<AppData>) -> impl Responder {
    match data.python_client.health_check().await {
        Ok(health_json) => {
            let status: AgentServiceStatus = serde_json::from_value(health_json)
                .unwrap_or(AgentServiceStatus {
                    status: "healthy".to_string(),
                    agent_count: 0,
                    active_agents: 0,
                    version: "0.1.0".to_string(),
                });
            HttpResponse::Ok().json(status)
        }
        Err(e) => HttpResponse::ServiceUnavailable().json(json!({
            "error": e.to_string(),
            "status": "unhealthy",
            "message": "Python service unavailable"
        })),
    }
}