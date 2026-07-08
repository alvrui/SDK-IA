// Domain structures for Cadiz12 project

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryElement {
    pub id: String,
    pub project_id: String,
    pub element_type: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub date: Option<String>,
    pub location: Option<String>,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub story_elements: Vec<String>,
    pub events: Vec<String>,
}