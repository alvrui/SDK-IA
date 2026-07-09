// SDK-IA Rust Backend
// Main entry point for the application

use actix_web::{web as actix_web, App, HttpServer, Responder, HttpResponse};
use std::io;
use std::sync::Arc;

mod app_data;
mod config;
mod domain;
mod services;
mod api;
mod web;

use crate::app_data::AppData;
use crate::services::persistence::PersistenceService;
use crate::services::narrative::NarrativeService;
use crate::services::validation::DomainValidationService;
use crate::services::versioning::VersioningService;
use crate::services::python_client::PythonClient;
use crate::domain::hollywood_animal::CompatibilityMatrix;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "sdk-ia-rust",
        "version": "0.1.0"
    }))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting SDK-IA Rust backend on port 9090");
    
    // Initialize Hollywood Animal compatibility matrix
    let mut compatibility_matrix = CompatibilityMatrix::new();
    match compatibility_matrix.load_from_csv(
        "data/hollywood_animal/elements.csv",
        "data/hollywood_animal/compatibility_rules.csv"
    ) {
        Ok(_) => println!("Hollywood Animal catalog loaded successfully: {} elements, {} rule sets",
                          compatibility_matrix.elements.len(), compatibility_matrix.rules.len()),
        Err(e) => eprintln!("Warning: Failed to load Hollywood Animal catalog: {}", e),
    }
    let compatibility_matrix = Arc::new(compatibility_matrix);
    
    // Initialize persistence service
    let persistence = Arc::new(PersistenceService::new("data/sdk-ia.db").expect("Failed to initialize database"));
    
    // Initialize Python client for agent communication
    let python_client = Arc::new(PythonClient::new("http://127.0.0.1:9000"));
    
    // Initialize narrative service
    let narrative_service = Arc::new(NarrativeService::new(
        persistence.clone(),
        compatibility_matrix.clone()
    ));
    
    // Initialize validation service
    let validation_service = Arc::new(DomainValidationService::new(
        persistence.clone(),
        compatibility_matrix.clone()
    ));
    
    // Initialize versioning service
    let versioning_service = Arc::new(VersioningService);
    
    // Build application data
    let app_data = actix_web::Data::new(AppData {
        persistence: persistence.clone(),
        narrative_service,
        validation_service,
        versioning_service,
        compatibility_matrix,
        python_client,
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/api/v1/internal/health", actix_web::get().to(health_check))
            .configure(api::routes::configure)
            .configure(web::configure)
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}