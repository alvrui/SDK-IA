// Validation service for Cadiz12 project

use crate::domain::structures::{Project, StoryElement, Event};
use crate::domain::enums::ValidationStatus;

pub struct ValidationService;

impl ValidationService {
    pub async fn validate_project(project: &Project) -> ValidationStatus {
        ValidationStatus::Valid
    }

    pub async fn validate_story_element(element: &StoryElement) -> ValidationStatus {
        ValidationStatus::Valid
    }

    pub async fn validate_event(event: &Event) -> ValidationStatus {
        ValidationStatus::Valid
    }
}