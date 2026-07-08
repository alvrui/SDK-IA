use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::domain::hollywood_animal::{CompatibilityMatrix, CompatibilityResult, HollywoodElement, ElementCategory};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilityQuery {
    pub element_a: String,
    pub element_b: String,
    pub preset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCompatibilityRequest {
    pub pairs: Vec<Vec<String>>,
    pub preset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCompatibilityQuery {
    pub elements: Vec<String>,
    pub preset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn get_hollywood_elements(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
) -> impl Responder {
    let elements: Vec<&HollywoodElement> = matrix.elements.values().collect();
    let mut categories: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for elem in matrix.elements.values() {
        let cat = format!("{:?}", elem.category);
        *categories.entry(cat).or_insert(0) += 1;
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": elements,
        "meta": {
            "count": elements.len(),
            "categories": categories
        }
    }))
}

pub async fn get_hollywood_element(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
    id: web::Path<String>,
) -> impl Responder {
    match matrix.elements.get(&id.into_inner()) {
        Some(element) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": element
        })),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "status": "error",
            "error": format!("Element with ID '{}' not found", id.into_inner())
        })),
    }
}

pub async fn get_elements_by_category(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
    category: web::Path<String>,
) -> impl Responder {
    match category.parse::<ElementCategory>() {
        Ok(cat) => {
            let elements: Vec<&HollywoodElement> = matrix.elements.values()
                .filter(|e| e.category == cat)
                .collect();
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "data": elements,
                "meta": {
                    "category": format!("{:?}", cat),
                    "count": elements.len()
                }
            }))
        }
        Err(_) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": format!("Invalid category: {}", category.into_inner())
        })),
    }
}

pub async fn search_elements(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
    query: web::Query<SearchQuery>,
) -> impl Responder {
    let search_term = query.q.to_lowercase();
    let results: Vec<&HollywoodElement> = matrix.elements.values()
        .filter(|e| e.id.to_lowercase().contains(&search_term))
        .collect();
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": results,
        "meta": {
            "search_term": query.q,
            "count": results.len()
        }
    }))
}

pub async fn get_compatibility(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
    query: web::Query<CompatibilityQuery>,
) -> impl Responder {
    match matrix.calculate_compatibility(
        &query.element_a, 
        &query.element_b, 
        query.preset.as_deref()
    ) {
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

pub async fn batch_compatibility(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
    payload: web::Json<BatchCompatibilityRequest>,
) -> impl Responder {
    let mut results: Vec<Result<CompatibilityResult, String>> = Vec::new();
    for pair in &payload.pairs {
        if pair.len() != 2 {
            results.push(Err("Each pair must have exactly 2 elements".to_string()));
            continue;
        }
        match matrix.calculate_compatibility(&pair[0], &pair[1], payload.preset.as_deref()) {
            Ok(result) => results.push(Ok(result)),
            Err(e) => results.push(Err(e)),
        }
    }
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": results,
        "meta": {
            "total_pairs": results.len(),
            "successful": results.iter().filter(|r| r.is_ok()).count(),
            "failed": results.iter().filter(|r| r.is_err()).count()
        }
    }))
}

pub async fn get_set_compatibility(
    matrix: web::Data<Arc<CompatibilityMatrix>>,
    query: web::Query<SetCompatibilityQuery>,
) -> impl Responder {
    match matrix.calculate_set_compatibility(&query.elements, query.preset.as_deref()) {
        Ok(scores) => HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": scores
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "error": e
        })),
    }
}

pub fn configure_hollywood_animal_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/internal/hollywood-animal")
            .route("/elements", web::get().to(get_hollywood_elements))
            .route("/elements/{id}", web::get().to(get_hollywood_element))
            .route("/elements/category/{category}", web::get().to(get_elements_by_category))
            .route("/elements/search", web::get().to(search_elements))
            .route("/compatibility", web::get().to(get_compatibility))
            .route("/compatibility/batch", web::post().to(batch_compatibility))
            .route("/compatibility/set", web::get().to(get_set_compatibility))
    );
}
