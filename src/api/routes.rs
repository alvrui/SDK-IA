// API routes for Cadiz12 project

use actix_web::{web, Scope};
use crate::api::handlers::*;

pub fn configure() -> Scope {
    web::scope("/api/v1/internal")
        .route("/health", web::get().to(get_health))
        .route("/projects", web::post().to(create_project))
        .route("/projects", web::get().to(list_projects))
        .route("/projects/{project_id}", web::get().to(get_project))
        .route("/projects/{project_id}/story-elements/{element_type}", web::post().to(generate_story_element))
        .route("/projects/{project_id}/events", web::post().to(generate_event))
        .route("/projects/{project_id}/narratives", web::post().to(generate_narrative))
}