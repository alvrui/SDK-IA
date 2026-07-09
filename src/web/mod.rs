// Web module for serving static files

use actix_web::web;

pub fn configure() -> impl actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> {
    actix_files::Files::new("./ui/dist", "./").index_file("index.html")
}