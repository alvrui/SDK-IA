// Routes configuration
use actix_web::web;
use crate::api::handlers::hollywood_animal::configure_hollywood_animal_routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    configure_hollywood_animal_routes(cfg);
}