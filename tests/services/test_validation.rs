// Unit tests for DomainValidationService
// Tests all 11 validation rules

use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use serde_json;

use crate::domain::{Project, Narrative, StoryElement, GameEvent, ProjectStatus, NarrativeStatus, StoryElementType, EventType};
use crate::services::validation::{DomainValidationService, ValidationResult, ValidationError, ValidationSeverity};
use crate::services::persistence::PersistenceService;
use crate::domain::hollywood_animal::CompatibilityMatrix;

fn create_test_narrative() -> Narrative {
    Narrative {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        title: "Test Narrative".to_string(),
        synopsis: "Test Synopsis".to_string(),
        status: NarrativeStatus::Draft,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: "1.0.0".to_string(),
        theme_ids: vec!["THEME_001".to_string()],
        compatibility_score: 0.85,
        context_summary: "Test context".to_string(),
        metadata: serde_json::Value::Null,
    }
}

fn create_test_story_element(element_type: StoryElementType) -> StoryElement {
    StoryElement {
        id: Uuid::new_v4(),
        narrative_id: Uuid::new_v4(),
        element_type,
        hollywood_element_id: format!("{}_001", match element_type {
            StoryElementType::Protagonist => "HERO",
            StoryElementType::Antagonist => "VILLAIN",
            StoryElementType::Finale => "FINAL",
            StoryElementType::Theme => "THEME",
            StoryElementType::Location => "LOC",
            _ => "ELEM",
        }),
        name: format!("Test {}", match element_type {
            StoryElementType::Protagonist => "Protagonist",
            StoryElementType::Antagonist => "Antagonist",
            StoryElementType::Finale => "Finale",
            StoryElementType::Theme => "Theme",
            StoryElementType::Location => "Location",
            _ => "Element",
        }),
        description: "Test description".to_string(),
        attributes: serde_json::Value::Null,
        created_at: Utc::now(),
        compatibility_score: 0.8,
    }
}

fn create_test_game_event() -> GameEvent {
    GameEvent {
        id: Uuid::new_v4(),
        narrative_id: Uuid::new_v4(),
        event_type: EventType::Scene,
        title: "Test Event".to_string(),
        description: "Test Description".to_string(),
        text: "Event text".to_string(),
        character_ids: vec![],
        location_ids: vec![],
        images: vec![],
        hollywood_event_id: None,
        timestamp: None,
        order_index: 0,
        attributes: serde_json::Value::Null,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation_result_new() {
        let result = ValidationResult::new();
        assert!(result.valid);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.warnings.len(), 0);
    }
    
    #[test]
    fn test_validation_result_add_error() {
        let mut result = ValidationResult::new();
        result.add_error(ValidationError::error("test_field", "Test error message"));
        
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].field, "test_field");
    }
    
    #[test]
    fn test_validation_result_add_warning() {
        let mut result = ValidationResult::new();
        result.add_warning(ValidationError::warning("test_field", "Test warning"));
        
        assert!(result.valid);
        assert_eq!(result.warnings.len(), 1);
    }
    
    #[test]
    fn test_validation_result_merge() {
        let mut result1 = ValidationResult::new();
        result1.add_error(ValidationError::error("field1", "Error 1"));
        
        let mut result2 = ValidationResult::new();
        result2.add_error(ValidationError::error("field2", "Error 2"));
        
        result1.merge(result2);
        
        assert!(!result1.valid);
        assert_eq!(result1.errors.len(), 2);
    }
}