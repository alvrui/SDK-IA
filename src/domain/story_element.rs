use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Type of story element
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum StoryElementType {
    Protagonist,
    Antagonist,
    Finale,
    Location,
    Theme,
}

impl Default for StoryElementType {
    fn default() -> Self {
        StoryElementType::Protagonist
    }
}

/// Story element - part of a Narrative
/// Represents characters, locations, themes, finales that define the narrative context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryElement {
    pub id: Uuid,
    pub narrative_id: Uuid,
    pub element_type: StoryElementType,
    /// Reference to Hollywood Animal catalog ID
    pub hollywood_element_id: String,
    pub name: String,
    pub description: String,
    /// Additional attributes specific to this element instance
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    /// Compatibility score with other elements in the narrative
    #[serde(default)]
    pub compatibility_score: f32,
}

impl StoryElement {
    pub fn new(
        narrative_id: Uuid,
        element_type: StoryElementType,
        hollywood_element_id: String,
        name: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            narrative_id,
            element_type,
            hollywood_element_id,
            name,
            description,
            attributes: HashMap::new(),
            created_at: Utc::now(),
            compatibility_score: 0.0,
        }
    }

    /// Check if this element is a protagonist
    pub fn is_protagonist(&self) -> bool {
        matches!(self.element_type, StoryElementType::Protagonist)
    }

    /// Check if this element is an antagonist
    pub fn is_antagonist(&self) -> bool {
        matches!(self.element_type, StoryElementType::Antagonist)
    }

    /// Check if this element is a finale
    pub fn is_finale(&self) -> bool {
        matches!(self.element_type, StoryElementType::Finale)
    }

    /// Check if this element is a location
    pub fn is_location(&self) -> bool {
        matches!(self.element_type, StoryElementType::Location)
    }

    /// Check if this element is a theme
    pub fn is_theme(&self) -> bool {
        matches!(self.element_type, StoryElementType::Theme)
    }
}
