// API routes configuration
use actix_web::web;
use crate::api::handlers::hollywood_animal::configure_hollywood_animal_routes;
use crate::api::handlers::project::configure_project_routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    configure_hollywood_animal_routes(cfg);
    configure_project_routes(cfg);
}

pub fn configure() -> impl actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> {
    web::scope("/api/v1/internal").configure(configure_routes)
}