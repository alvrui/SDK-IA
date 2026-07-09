use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Status of a narrative
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeStatus {
    Outline,
    InDevelopment,
    Review,
    Completed,
    Archived,
}

impl Default for NarrativeStatus {
    fn default() -> Self {
        NarrativeStatus::Outline
    }
}

/// Narrative structure - belongs to a Project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub synopsis: String,
    pub status: NarrativeStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: String,
    /// Reference to Hollywood Animal theme elements used in this narrative
    #[serde(default)]
    pub theme_ids: Vec<String>,
    /// Overall compatibility score based on all story elements
    #[serde(default)]
    pub compatibility_score: f32,
    /// Context summary for AI generation
    #[serde(default)]
    pub context_summary: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Narrative {
    pub fn new(
        project_id: Uuid,
        title: String,
        synopsis: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id,
            title,
            synopsis,
            status: NarrativeStatus::Outline,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: "1.0.0".to_string(),
            theme_ids: Vec::new(),
            compatibility_score: 0.0,
            context_summary: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn update_compatibility_score(&mut self, score: f32) {
        self.compatibility_score = score.clamp(0.0, 1.0);
        self.updated_at = Utc::now();
    }

    pub fn update_version(&mut self, version_type: &str) {
        let parts: Vec<&str> = self.version.split('.').collect();
        if parts.len() != 3 {
            self.version = "1.0.0".to_string();
            return;
        }

        match version_type {
            "major" => {
                let major = parts[0].parse::<u32>().unwrap_or(1) + 1;
                self.version = format!("{}.0.0", major);
            }
            "minor" => {
                let major = parts[0];
                let minor = parts[1].parse::<u32>().unwrap_or(0) + 1;
                self.version = format!("{}.{}.0", major, minor);
            }
            "patch" => {
                let major = parts[0];
                let minor = parts[1];
                let patch = parts[2].parse::<u32>().unwrap_or(0) + 1;
                self.version = format!("{}.{}.{}", major, minor, patch);
            }
            _ => {}
        }
        self.updated_at = Utc::now();
    }
}
