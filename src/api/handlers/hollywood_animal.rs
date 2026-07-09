use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use std::str::FromStr as _;

use crate::domain::hollywood_animal::{CompatibilityMatrix, HollywoodElement, CompatibilityResult, CompatibilityCategory};
use crate::app_data::AppData;

/// Configuration function for Hollywood Animal routes
pub fn configure_hollywood_animal_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/hollywood-animal")
            .route("/elements", web::get().to(list_hollywood_elements))
            .route("/elements/{id}", web::get().to(get_hollywood_element))
            .route("/compatibility", web::post().to(calculate_compatibility))
            .route("/categories", web::get().to(list_element_categories))
            .route("/elements/search", web::get().to(search_hollywood_elements))
    );
}

/// List all Hollywood Animal elements
async fn list_hollywood_elements(data: web::Data<AppData>) -> impl Responder {
    let elements: Vec<_> = data.compatibility_matrix.elements.values().cloned().collect();
    HttpResponse::Ok().json(json!({
        "elements": elements,
        "count": elements.len()
    }))
}

/// Get a specific Hollywood Animal element by ID
async fn get_hollywood_element(
    data: web::Data<AppData>,
    id: web::Path<String>,
) -> impl Responder {
    let id_str = id.into_inner();
    match data.compatibility_matrix.elements.get(&id_str) {
        Some(element) => HttpResponse::Ok().json(element),
        None => HttpResponse::NotFound().json(json!({
            "error": format!("Element with ID '{}' not found", id_str)
        })),
    }
}

/// Calculate compatibility between two Hollywood elements
async fn calculate_compatibility(
    data: web::Data<AppData>,
    payload: web::Json<serde_json::Value>,
) -> impl Responder {
    let element_a_id = payload.get("element_a_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| HttpResponse::BadRequest().json(json!({
            "error": "Missing element_a_id"
        })));
    
    let element_b_id = payload.get("element_b_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| HttpResponse::BadRequest().json(json!({
            "error": "Missing element_b_id"
        })));
    
    match (element_a_id, element_b_id) {
        (Ok(a_id), Ok(b_id)) => {
            if let Ok(result) = data.compatibility_matrix.calculate_compatibility(a_id, b_id, None) {
                HttpResponse::Ok().json(result)
            } else {
                HttpResponse::NotFound().json(json!({
                    "error": "Elements not found or compatibility calculation failed"
                }))
            }
        }
        (Err(e), _) | (_, Err(e)) => e,
    }
}

/// List all element categories
async fn list_element_categories() -> impl Responder {
    use crate::domain::hollywood_animal::ElementCategory;
    
    let categories = vec![
        ElementCategory::Protagonist,
        ElementCategory::Antagonist,
        ElementCategory::Supporting,
        ElementCategory::Event,
        ElementCategory::Theme,
        ElementCategory::Finale,
    ];
    
    HttpResponse::Ok().json(json!({
        "categories": categories.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>()
    }))
}

/// Search Hollywood elements by various criteria
async fn search_hollywood_elements(
    data: web::Data<AppData>,
    query: web::Query<serde_json::Value>,
) -> impl Responder {
    let category = query.get("category").and_then(|v| v.as_str());
    let search_term = query.get("q").and_then(|v| v.as_str());
    
    let elements: Vec<_> = data.compatibility_matrix.elements.values().cloned().collect();
    
    let filtered_elements: Vec<_> = elements.into_iter()
        .filter(|e| {
            let matches_category = category.map_or(true, |c| format!("{:?}", e.category).to_lowercase() == c.to_lowercase());
            let matches_search = search_term.map_or(true, |term| {
                e.id.contains(term) || 
                e.name.contains(term) ||
                e.subtype.contains(term)
            });
            matches_category && matches_search
        })
        .collect();
    
    HttpResponse::Ok().json(json!({
        "elements": filtered_elements,
        "count": filtered_elements.len()
    }))
}

// Implement compatibility calculation for CompatibilityMatrix
impl CompatibilityMatrix {
    pub fn calculate_compatibility(
        &self,
        a_id: &str,
        b_id: &str,
        _context: Option<&str>,
    ) -> Result<CompatibilityResult, String> {
        let a = self.elements.get(a_id)
            .ok_or_else(|| format!("Element {} not found", a_id))?;
        let b = self.elements.get(b_id)
            .ok_or_else(|| format!("Element {} not found", b_id))?;
        
        let pair_type = self.get_pair_type(a, b);
        
        // Calculate score based on rules
        let mut total_score = 0.0f32;
        let mut count = 0;
        
        if let Some(rules) = self.rules.get(&pair_type) {
            for (axis, weight) in rules {
                // Here you would calculate the actual compatibility based on the axis
                // For now, return a default score
                total_score += weight;
                count += 1;
            }
        }
        
        let score = if count > 0 {
            total_score / count as f32
        } else {
            0.5 // Default neutral score
        };
        
        Ok(CompatibilityResult {
            score,
            category: CompatibilityCategory::discretize_score(score),
            axis_scores: HashMap::new(),
            penalties: Vec::new(),
            bonuses: Vec::new(),
            explanation: vec!["Compatibility calculated based on Hollywood Animal rules".to_string()],
        })
    }
}