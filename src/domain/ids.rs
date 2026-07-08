// ID generation utilities for Cadiz12 project

use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_project_id() -> String {
    format!("proj_{}", Uuid::new_v4().to_string())
}

pub fn generate_story_element_id() -> String {
    format!("se_{}", Uuid::new_v4().to_string())
}

pub fn generate_event_id() -> String {
    format!("evt_{}", Uuid::new_v4().to_string())
}

pub fn generate_narrative_id() -> String {
    format!("narr_{}", Uuid::new_v4().to_string())
}