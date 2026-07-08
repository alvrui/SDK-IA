// Generation service for Cadiz12 project

use crate::domain::structures::{StoryElement, Event, Narrative};

pub struct GenerationService;

impl GenerationService {
    pub async fn generate_story_element(
        project_id: &str,
        element_type: &str,
        prompt: &str,
    ) -> StoryElement {
        StoryElement {
            id: crate::domain::ids::generate_story_element_id(),
            project_id: project_id.to_string(),
            element_type: element_type.to_string(),
            content: format!("Generated content for: {}", prompt),
            metadata: std::collections::HashMap::new(),
            order: 0,
        }
    }

    pub async fn generate_event(project_id: &str, prompt: &str) -> Event {
        Event {
            id: crate::domain::ids::generate_event_id(),
            project_id: project_id.to_string(),
            title: "Generated Event".to_string(),
            description: format!("Generated event for: {}", prompt),
            date: None,
            location: None,
            participants: vec![],
        }
    }

    pub async fn generate_narrative(project_id: &str, prompt: &str) -> Narrative {
        Narrative {
            id: crate::domain::ids::generate_narrative_id(),
            project_id: project_id.to_string(),
            title: "Generated Narrative".to_string(),
            content: format!("Generated narrative for: {}", prompt),
            story_elements: vec![],
            events: vec![],
        }
    }
}