// SDK-IA Rust Backend
// Main entry point for the application

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::io;

mod config;
mod domain;
mod services;
mod api;
mod web;

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
    
    HttpServer::new(|| {
        App::new()
            .route("/api/v1/internal/health", web::get().to(health_check))
            .service(api::routes::configure())
    })
    .bind("127.0.0.1:9090")?
    .run()
    .await
}