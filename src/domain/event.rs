use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::str::FromStr;

/// Type of game event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Scene,
    Dialogue,
    Action,
    Decision,
    Transition,
    Combat,
    Puzzle,
    Exploration,
}

impl Default for EventType {
    fn default() -> Self {
        EventType::Scene
    }
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "scene" => Ok(EventType::Scene),
            "dialogue" => Ok(EventType::Dialogue),
            "action" => Ok(EventType::Action),
            "decision" => Ok(EventType::Decision),
            "transition" => Ok(EventType::Transition),
            "combat" => Ok(EventType::Combat),
            "puzzle" => Ok(EventType::Puzzle),
            "exploration" => Ok(EventType::Exploration),
            _ => Err(format!("Unknown event type: {}", s)),
        }
    }
}

/// Game event - generated based on narrative context
/// Contains rich content: texts, characters, images, location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub id: Uuid,
    pub narrative_id: Uuid,
    pub event_type: EventType,
    pub title: String,
    pub description: String,
    /// Main text content of the event
    #[serde(default)]
    pub text: String,
    /// List of character IDs involved in this event
    #[serde(default)]
    pub character_ids: Vec<Uuid>,
    /// List of location IDs where this event takes place
    #[serde(default)]
    pub location_ids: Vec<Uuid>,
    /// URLs or base64 encoded images
    #[serde(default)]
    pub images: Vec<String>,
    /// Reference to Hollywood Animal event catalog ID
    #[serde(default)]
    pub hollywood_event_id: Option<String>,
    /// Timestamp within the narrative (can be relative or absolute)
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
    /// Order index for sequencing
    #[serde(default)]
    pub order_index: i32,
    /// Additional metadata and attributes
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GameEvent {
    pub fn new(
        narrative_id: Uuid,
        event_type: EventType,
        title: String,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            narrative_id,
            event_type,
            title,
            description,
            text: String::new(),
            character_ids: Vec::new(),
            location_ids: Vec::new(),
            images: Vec::new(),
            hollywood_event_id: None,
            timestamp: None,
            order_index: 0,
            attributes: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add a character to this event
    pub fn add_character(&mut self, character_id: Uuid) {
        if !self.character_ids.contains(&character_id) {
            self.character_ids.push(character_id);
            self.updated_at = Utc::now();
        }
    }

    /// Add a location to this event
    pub fn add_location(&mut self, location_id: Uuid) {
        if !self.location_ids.contains(&location_id) {
            self.location_ids.push(location_id);
            self.updated_at = Utc::now();
        }
    }

    /// Add an image to this event
    pub fn add_image(&mut self, image_url: String) {
        self.images.push(image_url);
        self.updated_at = Utc::now();
    }
}