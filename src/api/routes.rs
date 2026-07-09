// API routes configuration
use actix_web::web;
use crate::api::handlers::hollywood_animal::configure_hollywood_animal_routes;
use crate::api::handlers::project::configure_project_routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    configure_hollywood_animal_routes(cfg);
    configure_project_routes(cfg);
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1/internal").configure(configure_routes));
}