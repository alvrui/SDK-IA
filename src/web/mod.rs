// Web module for serving static files

use actix_web::web;
use actix_files as fs;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        fs::Files::new("./ui/dist", "./").index_file("index.html")
    );
}