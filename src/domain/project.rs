use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::str::FromStr;

/// Status of a project
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Draft,
    InProgress,
    Completed,
    Archived,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Draft
    }
}

impl FromStr for ProjectStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(ProjectStatus::Draft),
            "in_progress" => Ok(ProjectStatus::InProgress),
            "completed" => Ok(ProjectStatus::Completed),
            "archived" => Ok(ProjectStatus::Archived),
            _ => Err(format!("Unknown project status: {}", s)),
        }
    }
}

/// Settings for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    #[serde(default)]
    pub allow_ai_generation: bool,
    #[serde(default)]
    pub ai_model: String,
    #[serde(default)]
    pub max_tokens: u32,
    #[serde(default)]
    pub temperature: f32,
    #[serde(default)]
    pub use_hollywood_animal: bool,
    #[serde(default)]
    pub compatibility_threshold: f32,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            allow_ai_generation: true,
            ai_model: "mistral-large-latest".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            use_hollywood_animal: true,
            compatibility_threshold: 0.6,
        }
    }
}

/// Main project structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: String,
    pub status: ProjectStatus,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub settings: ProjectSettings,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        author: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            author,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: "1.0.0".to_string(),
            status: ProjectStatus::Draft,
            tags: Vec::new(),
            settings: ProjectSettings::default(),
            metadata: HashMap::new(),
        }
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