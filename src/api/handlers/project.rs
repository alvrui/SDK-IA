use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::domain::{Project, Narrative, StoryElement, GameEvent, ProjectStatus, NarrativeStatus, StoryElementType, EventType};
use crate::services::persistence::PersistenceService;
use crate::services::narrative::NarrativeService;
use crate::domain::hollywood_animal::CompatibilityMatrix;
use crate::app_data::AppData;

// ==================== REQUEST/RESPONSE TYPES ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProjectRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchProjectsQuery {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub tags: Option<String>, // Comma-separated
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 { 1 }
fn default_page_size() -> u32 { 20 }

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNarrativeRequest {
    pub project_id: String,
    pub title: String,
    pub synopsis: String,
    #[serde(default)]
    pub theme_ids: Vec<String>,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNarrativeRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub synopsis: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub theme_ids: Option<Vec<String>>,
    #[serde(default)]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStoryElementRequest {
    pub narrative_id: String,
    pub element_type: String,
    pub hollywood_element_id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameEventRequest {
    pub narrative_id: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub character_ids: Vec<String>,
    #[serde(default)]
    pub location_ids: Vec<String>,
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(default)]
    pub hollywood_event_id: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub order_index: Option<i32>,
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
}

// ==================== PROJECT ENDPOINTS ====================

/// Create a new project
pub async fn create_project(
    data: web::Data<Arc<AppData>>,
    payload: web::Json<CreateProjectRequest>,
) -> impl Responder {
    let mut project = Project::new(
        payload.name.clone(),
        payload.description.clone(),
        payload.author.clone(),
    );
    project.tags = payload.tags.clone();
    project.metadata = payload.metadata.clone();

    match data.persistence.create_project(&project) {
        Ok(id) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "data": {
                "id": id.to_string(),
                "name": project.name,
                "version": project.version,
                "created_at": project.created_at.to_rfc3339()
            }
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "error": e.to_string()
        })),
    }
}

/// Get a project by ID
pub async fn get_project(
    data: web::Data<Arc<AppData>>,
    project_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&project_id.into_inner()) {
        Ok(id) => {
            match data.persistence.get_project(&id) {
                Ok(Some(project)) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": project
                })),
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Project not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid project ID format"
        })),
    }
}

/// List all projects with pagination
pub async fn list_projects(
    data: web::Data<Arc<AppData>>,
    query: web::Query<SearchProjectsQuery>,
) -> impl Responder {
    let tags: Option<Vec<String>> = query.tags.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect());
    
    let status: Option<ProjectStatus> = query.status.as_ref().and_then(|s| ProjectStatus::from_str(s).ok());

    match data.persistence.search_projects(
        query.name.as_deref(),
        query.author.as_deref(),
        status.as_ref(),
        tags.as_deref(),
        query.page,
        query.page_size,
    ) {
        Ok(projects) => {
            let total = data.persistence.count_projects_search(
                query.name.as_deref(),
                query.author.as_deref(),
                status.as_ref(),
                tags.as_deref(),
            ).unwrap_or(0);
            
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "data": projects,
                "meta": {
                    "page": query.page,
                    "page_size": query.page_size,
                    "total": total,
                    "total_pages": (total as f64 / query.page_size as f64).ceil() as i64
                }
            }))
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "error": e.to_string()
        })),
    }
}

/// Update a project
pub async fn update_project(
    data: web::Data<Arc<AppData>>,
    project_id: web::Path<String>,
    payload: web::Json<UpdateProjectRequest>,
) -> impl Responder {
    match Uuid::parse_str(&project_id.into_inner()) {
        Ok(id) => {
            match data.persistence.get_project(&id) {
                Ok(Some(mut project)) => {
                    if let Some(name) = &payload.name {
                        project.name = name.clone();
                    }
                    if let Some(description) = &payload.description {
                        project.description = description.clone();
                    }
                    if let Some(author) = &payload.author {
                        project.author = author.clone();
                    }
                    if let Some(status) = &payload.status {
                        if let Ok(status_enum) = ProjectStatus::from_str(status) {
                            project.status = status_enum;
                        }
                    }
                    if let Some(tags) = &payload.tags {
                        project.tags = tags.clone();
                    }
                    if let Some(metadata) = &payload.metadata {
                        project.metadata = metadata.clone();
                    }
                    project.updated_at = Utc::now();
                    project.update_version("patch");

                    match data.persistence.update_project(&project) {
                        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "data": project,
                            "message": "Project updated successfully"
                        })),
                        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "error",
                            "error": e.to_string()
                        })),
                    }
                }
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Project not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid project ID format"
        })),
    }
}

/// Delete a project
pub async fn delete_project(
    data: web::Data<Arc<AppData>>,
    project_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&project_id.into_inner()) {
        Ok(id) => {
            match data.persistence.delete_project(&id) {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "message": "Project and all related data deleted successfully"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid project ID format"
        })),
    }
}

// ==================== NARRATIVE ENDPOINTS ====================

/// Create a new narrative with automatic compatibility calculation
pub async fn create_narrative(
    data: web::Data<Arc<AppData>>,
    project_id: web::Path<String>,
    payload: web::Json<CreateNarrativeRequest>,
) -> impl Responder {
    match Uuid::parse_str(&project_id.into_inner()) {
        Ok(project_id_uuid) => {
            // Validate theme IDs exist in Hollywood Animal catalog
            for theme_id in &payload.theme_ids {
                if !data.compatibility_matrix.elements.contains_key(theme_id) {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "error": format!("Theme '{}' not found in Hollywood Animal catalog", theme_id)
                    }));
                }
            }

            match data.narrative_service.create_narrative_with_compatibility(
                project_id_uuid,
                payload.title.clone(),
                payload.synopsis.clone(),
                payload.theme_ids.clone(),
                payload.metadata.clone(),
            ) {
                Ok(id) => {
                    // Get the created narrative to return full data
                    if let Ok(Some(narrative)) = data.persistence.get_narrative(&id) {
                        HttpResponse::Created().json(serde_json::json!({
                            "status": "success",
                            "data": {
                                "id": id.to_string(),
                                "project_id": narrative.project_id.to_string(),
                                "title": narrative.title,
                                "compatibility_score": narrative.compatibility_score,
                                "version": narrative.version,
                                "created_at": narrative.created_at.to_rfc3339()
                            }
                        }))
                    } else {
                        HttpResponse::Created().json(serde_json::json!({
                            "status": "success",
                            "data": {
                                "id": id.to_string(),
                                "project_id": project_id_uuid.to_string(),
                                "title": payload.title,
                                "version": "1.0.0"
                            }
                        }))
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid project ID format"
        })),
    }
}

/// Get a narrative by ID
pub async fn get_narrative(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(id) => {
            match data.persistence.get_narrative(&id) {
                Ok(Some(narrative)) => {
                    // Also get story elements for this narrative
                    match data.persistence.list_story_elements_by_narrative(&id) {
                        Ok(elements) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "data": {
                                "narrative": narrative,
                                "story_elements": elements
                            }
                        })),
                        Err(_) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "data": narrative
                        })),
                    }
                }
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Narrative not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// List narratives by project ID
pub async fn list_narratives(
    data: web::Data<Arc<AppData>>,
    project_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&project_id.into_inner()) {
        Ok(id) => {
            match data.persistence.list_narratives_by_project(&id) {
                Ok(narratives) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": narratives,
                    "meta": {
                        "count": narratives.len(),
                        "project_id": id.to_string()
                    }
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid project ID format"
        })),
    }
}

/// Update a narrative
pub async fn update_narrative(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
    payload: web::Json<UpdateNarrativeRequest>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(id) => {
            // Validate theme IDs if provided
            if let Some(ref theme_ids) = payload.theme_ids {
                for theme_id in theme_ids {
                    if !data.compatibility_matrix.elements.contains_key(theme_id) {
                        return HttpResponse::BadRequest().json(serde_json::json!({
                            "status": "error",
                            "error": format!("Theme '{}' not found in Hollywood Animal catalog", theme_id)
                        }));
                    }
                }
            }

            match data.persistence.get_narrative(&id) {
                Ok(Some(mut narrative)) => {
                    if let Some(title) = &payload.title {
                        narrative.title = title.clone();
                    }
                    if let Some(synopsis) = &payload.synopsis {
                        narrative.synopsis = synopsis.clone();
                    }
                    if let Some(status) = &payload.status {
                        if let Ok(status_enum) = NarrativeStatus::from_str(status) {
                            narrative.status = status_enum;
                        }
                    }
                    if let Some(theme_ids) = &payload.theme_ids {
                        narrative.theme_ids = theme_ids.clone();
                    }
                    if let Some(metadata) = &payload.metadata {
                        narrative.metadata = metadata.clone();
                    }
                    narrative.updated_at = Utc::now();
                    narrative.update_version("patch");

                    // Recalculate compatibility score
                    if let Err(e) = data.narrative_service.recalculate_narrative_compatibility(id) {
                        eprintln!("Failed to recalculate compatibility: {}", e);
                    }

                    match data.persistence.update_narrative(&narrative) {
                        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "data": narrative,
                            "message": "Narrative updated successfully"
                        })),
                        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "error",
                            "error": e.to_string()
                        })),
                    }
                }
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Narrative not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// Delete a narrative
pub async fn delete_narrative(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(id) => {
            match data.persistence.delete_narrative(&id) {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "message": "Narrative and all related data deleted successfully"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

// ==================== STORY ELEMENT ENDPOINTS ====================

/// Create a new story element with validation
pub async fn create_story_element(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
    payload: web::Json<CreateStoryElementRequest>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(narrative_id_uuid) => {
            // Validate Hollywood element ID
            if !data.compatibility_matrix.elements.contains_key(&payload.hollywood_element_id) {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "error",
                    "error": format!("Hollywood element '{}' not found in catalog", payload.hollywood_element_id)
                }));
            }

            // Validate element type
            let element_type = match StoryElementType::from_str(&payload.element_type) {
                Ok(et) => et,
                Err(_) => {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "error": format!("Invalid element type: {}", payload.element_type)
                    }));
                }
            };

            let element = StoryElement::new(
                narrative_id_uuid,
                element_type,
                payload.hollywood_element_id.clone(),
                payload.name.clone(),
                payload.description.clone(),
            );

            match data.narrative_service.add_story_element_and_recalculate(narrative_id_uuid, &element) {
                Ok(_) => {
                    // Get the created element to return ID
                    if let Ok(Some(created_element)) = data.persistence.get_story_element(&element.id) {
                        HttpResponse::Created().json(serde_json::json!({
                            "status": "success",
                            "data": {
                                "id": created_element.id.to_string(),
                                "narrative_id": created_element.narrative_id.to_string(),
                                "element_type": format!("{:?}", created_element.element_type),
                                "hollywood_element_id": created_element.hollywood_element_id,
                                "name": created_element.name,
                                "created_at": created_element.created_at.to_rfc3339()
                            },
                            "message": "Story element created and narrative compatibility recalculated"
                        }))
                    } else {
                        HttpResponse::Created().json(serde_json::json!({
                            "status": "success",
                            "data": {
                                "id": element.id.to_string()
                            }
                        }))
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// Get a story element by ID
pub async fn get_story_element(
    data: web::Data<Arc<AppData>>,
    element_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&element_id.into_inner()) {
        Ok(id) => {
            match data.persistence.get_story_element(&id) {
                Ok(Some(element)) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": element
                })),
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Story element not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid element ID format"
        })),
    }
}

/// List story elements by narrative ID
pub async fn list_story_elements(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(id) => {
            match data.persistence.list_story_elements_by_narrative(&id) {
                Ok(elements) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": elements,
                    "meta": {
                        "count": elements.len(),
                        "narrative_id": id.to_string()
                    }
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// List story elements by narrative ID and type
pub async fn list_story_elements_by_type(
    data: web::Data<Arc<AppData>>,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let (narrative_id, element_type) = params.into_inner();
    
    match (Uuid::parse_str(&narrative_id), StoryElementType::from_str(&element_type)) {
        (Ok(narrative_id_uuid), Ok(element_type_enum)) => {
            match data.persistence.list_story_elements_by_type(&narrative_id_uuid, element_type_enum) {
                Ok(elements) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": elements,
                    "meta": {
                        "count": elements.len(),
                        "narrative_id": narrative_id_uuid.to_string(),
                        "element_type": format!("{:?}", element_type_enum)
                    }
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        _ => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID or element type format"
        })),
    }
}

/// Update a story element
pub async fn update_story_element(
    data: web::Data<Arc<AppData>>,
    element_id: web::Path<String>,
    payload: web::Json<CreateStoryElementRequest>,
) -> impl Responder {
    match Uuid::parse_str(&element_id.into_inner()) {
        Ok(id) => {
            // Validate Hollywood element ID
            if !data.compatibility_matrix.elements.contains_key(&payload.hollywood_element_id) {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "status": "error",
                    "error": format!("Hollywood element '{}' not found in catalog", payload.hollywood_element_id)
                }));
            }

            match data.persistence.get_story_element(&id) {
                Ok(Some(mut element)) => {
                    if let Ok(et) = StoryElementType::from_str(&payload.element_type) {
                        element.element_type = et;
                    }
                    element.hollywood_element_id = payload.hollywood_element_id.clone();
                    element.name = payload.name.clone();
                    element.description = payload.description.clone();
                    element.attributes = payload.attributes.clone();

                    match data.persistence.update_story_element(&element) {
                        Ok(_) => {
                            // Recalculate narrative compatibility
                            if let Err(e) = data.narrative_service.recalculate_narrative_compatibility(element.narrative_id) {
                                eprintln!("Failed to recalculate compatibility: {}", e);
                            }
                            
                            HttpResponse::Ok().json(serde_json::json!({
                                "status": "success",
                                "data": element,
                                "message": "Story element updated and narrative compatibility recalculated"
                            }))
                        }
                        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "error",
                            "error": e.to_string()
                        })),
                    }
                }
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Story element not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid element ID format"
        })),
    }
}

/// Delete a story element
pub async fn delete_story_element(
    data: web::Data<Arc<AppData>>,
    element_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&element_id.into_inner()) {
        Ok(id) => {
            // Get the element first to get its narrative_id
            let narrative_id = match data.persistence.get_story_element(&id) {
                Ok(Some(element)) => Some(element.narrative_id),
                _ => None,
            };

            match data.persistence.delete_story_element(&id) {
                Ok(_) => {
                    // Recalculate narrative compatibility if we have the narrative_id
                    if let Some(narrative_id) = narrative_id {
                        if let Err(e) = data.narrative_service.recalculate_narrative_compatibility(narrative_id) {
                            eprintln!("Failed to recalculate compatibility: {}", e);
                        }
                    }
                    
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "success",
                        "message": "Story element deleted and narrative compatibility recalculated"
                    }))
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid element ID format"
        })),
    }
}

// ==================== GAME EVENT ENDPOINTS ====================

/// Create a new game event
pub async fn create_game_event(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
    payload: web::Json<CreateGameEventRequest>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(narrative_id_uuid) => {
            // Validate Hollywood event ID if provided
            if let Some(ref event_id) = payload.hollywood_event_id {
                if !data.compatibility_matrix.elements.contains_key(event_id) {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "error": format!("Hollywood event '{}' not found in catalog", event_id)
                    }));
                }
            }

            let event_type = match EventType::from_str(&payload.event_type) {
                Ok(et) => et,
                Err(_) => {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "error": format!("Invalid event type: {}", payload.event_type)
                    }));
                }
            };

            let mut event = GameEvent::new(
                narrative_id_uuid,
                event_type,
                payload.title.clone(),
                payload.description.clone(),
            );
            event.text = payload.text.clone();
            event.character_ids = payload.character_ids.clone().into_iter()
                .filter_map(|s| Uuid::parse_str(&s).ok())
                .collect();
            event.location_ids = payload.location_ids.clone().into_iter()
                .filter_map(|s| Uuid::parse_str(&s).ok())
                .collect();
            event.images = payload.images.clone();
            event.hollywood_event_id = payload.hollywood_event_id.clone();
            event.order_index = payload.order_index.unwrap_or(0);
            event.attributes = payload.attributes.clone();

            match data.persistence.create_game_event(&event) {
                Ok(id) => HttpResponse::Created().json(serde_json::json!({
                    "status": "success",
                    "data": {
                        "id": id.to_string(),
                        "narrative_id": event.narrative_id.to_string(),
                        "event_type": format!("{:?}", event.event_type),
                        "title": event.title,
                        "order_index": event.order_index,
                        "created_at": event.created_at.to_rfc3339()
                    }
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// Get a game event by ID
pub async fn get_game_event(
    data: web::Data<Arc<AppData>>,
    event_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&event_id.into_inner()) {
        Ok(id) => {
            match data.persistence.get_game_event(&id) {
                Ok(Some(event)) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": event
                })),
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Game event not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid event ID format"
        })),
    }
}

/// List game events by narrative ID
pub async fn list_game_events(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(id) => {
            match data.persistence.list_game_events_by_narrative(&id) {
                Ok(events) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "data": events,
                    "meta": {
                        "count": events.len(),
                        "narrative_id": id.to_string()
                    }
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// Update a game event
pub async fn update_game_event(
    data: web::Data<Arc<AppData>>,
    event_id: web::Path<String>,
    payload: web::Json<CreateGameEventRequest>,
) -> impl Responder {
    match Uuid::parse_str(&event_id.into_inner()) {
        Ok(id) => {
            // Validate Hollywood event ID if provided
            if let Some(ref event_id) = payload.hollywood_event_id {
                if !data.compatibility_matrix.elements.contains_key(event_id) {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "status": "error",
                        "error": format!("Hollywood event '{}' not found in catalog", event_id)
                    }));
                }
            }

            match data.persistence.get_game_event(&id) {
                Ok(Some(mut event)) => {
                    if let Ok(et) = EventType::from_str(&payload.event_type) {
                        event.event_type = et;
                    }
                    event.title = payload.title.clone();
                    event.description = payload.description.clone();
                    event.text = payload.text.clone();
                    event.character_ids = payload.character_ids.clone().into_iter()
                        .filter_map(|s| Uuid::parse_str(&s).ok())
                        .collect();
                    event.location_ids = payload.location_ids.clone().into_iter()
                        .filter_map(|s| Uuid::parse_str(&s).ok())
                        .collect();
                    event.images = payload.images.clone();
                    event.hollywood_event_id = payload.hollywood_event_id.clone();
                    if let Some(order_index) = payload.order_index {
                        event.order_index = order_index;
                    }
                    event.attributes = payload.attributes.clone();
                    event.updated_at = Utc::now();

                    match data.persistence.update_game_event(&event) {
                        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "data": event,
                            "message": "Game event updated successfully"
                        })),
                        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "error",
                            "error": e.to_string()
                        })),
                    }
                }
                Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                    "status": "error",
                    "error": "Game event not found"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid event ID format"
        })),
    }
}

/// Delete a game event
pub async fn delete_game_event(
    data: web::Data<Arc<AppData>>,
    event_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&event_id.into_inner()) {
        Ok(id) => {
            match data.persistence.delete_game_event(&id) {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "message": "Game event deleted successfully"
                })),
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid event ID format"
        })),
    }
}

// ==================== VALIDATION ENDPOINTS ====================

/// Validate all elements in a narrative
pub async fn validate_narrative(
    data: web::Data<Arc<AppData>>,
    narrative_id: web::Path<String>,
) -> impl Responder {
    match Uuid::parse_str(&narrative_id.into_inner()) {
        Ok(id) => {
            match data.narrative_service.validate_narrative_elements(id) {
                Ok(errors) => {
                    if errors.is_empty() {
                        HttpResponse::Ok().json(serde_json::json!({
                            "status": "success",
                            "data": {
                                "valid": true,
                                "errors": errors
                            }
                        }))
                    } else {
                        HttpResponse::BadRequest().json(serde_json::json!({
                            "status": "error",
                            "data": {
                                "valid": false,
                                "errors": errors
                            }
                        }))
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "error": e.to_string()
                })),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": "Invalid narrative ID format"
        })),
    }
}

/// Validate a Hollywood element ID
pub async fn validate_hollywood_element(
    data: web::Data<Arc<AppData>>,
    element_id: web::Path<String>,
) -> impl Responder {
    let element_id = element_id.into_inner();
    let valid = data.compatibility_matrix.elements.contains_key(&element_id);
    
    if valid {
        if let Some(element) = data.compatibility_matrix.elements.get(&element_id) {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "data": {
                    "valid": true,
                    "element": element
                }
            }))
        } else {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "data": {
                    "valid": true
                }
            }))
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "data": {
                "valid": false,
                "error": format!("Element '{}' not found in Hollywood Animal catalog", element_id)
            }
        }))
    }
}

/// Get compatibility between two Hollywood elements
pub async fn get_element_compatibility(
    data: web::Data<Arc<AppData>>,
    query: web::Query<(String, String)>,
) -> impl Responder {
    let (element_a, element_b) = query.into_inner();
    
    match data.compatibility_matrix.calculate_compatibility(&element_a, &element_b, None) {
        Ok(result) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": result
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": e
        })),
    }
}

// ==================== ROUTE CONFIGURATION ====================

pub fn configure_project_routes(cfg: &mut web::ServiceConfig) {
    // Project routes
    cfg.service(
        web::scope("/api/v1/internal/projects")
            .route("", web::post().to(create_project))
            .route("", web::get().to(list_projects))
            .route("/search", web::get().to(list_projects))
            .route("/{project_id}", web::get().to(get_project))
            .route("/{project_id}", web::put().to(update_project))
            .route("/{project_id}", web::delete().to(delete_project))
            // Narrative routes
            .route("/{project_id}/narratives", web::post().to(create_narrative))
            .route("/{project_id}/narratives", web::get().to(list_narratives))
            .route("/narratives/{narrative_id}", web::get().to(get_narrative))
            .route("/narratives/{narrative_id}", web::put().to(update_narrative))
            .route("/narratives/{narrative_id}", web::delete().to(delete_narrative))
            .route("/narratives/{narrative_id}/validate", web::get().to(validate_narrative))
            // Story element routes
            .route("/narratives/{narrative_id}/elements", web::post().to(create_story_element))
            .route("/narratives/{narrative_id}/elements", web::get().to(list_story_elements))
            .route("/narratives/{narrative_id}/elements/type/{element_type}", web::get().to(list_story_elements_by_type))
            .route("/elements/{element_id}", web::get().to(get_story_element))
            .route("/elements/{element_id}", web::put().to(update_story_element))
            .route("/elements/{element_id}", web::delete().to(delete_story_element))
            // Game event routes
            .route("/narratives/{narrative_id}/events", web::post().to(create_game_event))
            .route("/narratives/{narrative_id}/events", web::get().to(list_game_events))
            .route("/events/{event_id}", web::get().to(get_game_event))
            .route("/events/{event_id}", web::put().to(update_game_event))
            .route("/events/{event_id}", web::delete().to(delete_game_event))
            // Validation routes
            .route("/hollywood/elements/{element_id}/validate", web::get().to(validate_hollywood_element))
            .route("/hollywood/compatibility", web::get().to(get_element_compatibility))
    );
}
