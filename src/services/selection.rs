// Selection service for Cadiz12 project

use crate::domain::structures::{StoryElement, Event};

pub struct SelectionService;

impl SelectionService {
    pub async fn select_story_elements(
        project_id: &str,
        criteria: &str,
    ) -> Vec<StoryElement> {
        vec![]
    }

    pub async fn select_events(project_id: &str, criteria: &str) -> Vec<Event> {
        vec![]
    }
}