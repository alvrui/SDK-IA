// SDK-IA Rust Backend
// Main entry point for the application

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
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
    
    // Build application data
    let app_data = web::Data::new(AppData {
        persistence: persistence.clone(),
        narrative_service,
        validation_service,
        compatibility_matrix,
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/api/v1/internal/health", web::get().to(health_check))
            .service(api::routes::configure())
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}
