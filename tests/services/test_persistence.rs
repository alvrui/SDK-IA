// Unit tests for PersistenceService

use std::path::Path;
use tempfile::NamedTempFile;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use serde_json;

use crate::services::persistence::PersistenceService;
use crate::domain::{Project, Narrative, StoryElement, GameEvent, ProjectStatus, NarrativeStatus, StoryElementType, EventType};

fn create_test_project() -> Project {
    Project {
        id: Uuid::new_v4(),
        name: "Test Project".to_string(),
        description: "Test Description".to_string(),
        author: "Test Author".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: "1.0.0".to_string(),
        status: ProjectStatus::Draft,
        tags: vec!["test".to_string()],
        settings: serde_json::Value::Null,
        metadata: serde_json::Value::Null,
    }
}

fn create_test_narrative(project_id: Uuid) -> Narrative {
    Narrative {
        id: Uuid::new_v4(),
        project_id,
        title: "Test Narrative".to_string(),
        synopsis: "Test Synopsis".to_string(),
        status: NarrativeStatus::Draft,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: "1.0.0".to_string(),
        theme_ids: vec!["THEME_001".to_string()],
        compatibility_score: 0.8,
        context_summary: "Test context".to_string(),
        metadata: serde_json::Value::Null,
    }
}

fn create_test_story_element(narrative_id: Uuid) -> StoryElement {
    StoryElement {
        id: Uuid::new_v4(),
        narrative_id,
        element_type: StoryElementType::Protagonist,
        hollywood_element_id: "HERO_001".to_string(),
        name: "Test Protagonist".to_string(),
        description: "Test Description".to_string(),
        attributes: serde_json::Value::Null,
        created_at: Utc::now(),
        compatibility_score: 0.8,
    }
}

fn create_test_game_event(narrative_id: Uuid) -> GameEvent {
    GameEvent {
        id: Uuid::new_v4(),
        narrative_id,
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
    
    // ==================== DATABASE INITIALIZATION ====================
    
    #[test]
    fn test_new_persistence_service() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        let result = PersistenceService::new(db_path);
        assert!(result.is_ok());
    }
    
    // ==================== PROJECT OPERATIONS ====================
    
    #[test]
    fn test_create_and_get_project() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let retrieved = service.get_project(&project_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, project.name);
    }
    
    #[test]
    fn test_get_nonexistent_project() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let result = service.get_project(&Uuid::new_v4()).unwrap();
        assert!(result.is_none());
    }
    
    #[test]
    fn test_update_project() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let mut project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        project.name = "Updated Project".to_string();
        service.update_project(&project).unwrap();
        
        let retrieved = service.get_project(&project_id).unwrap().unwrap();
        assert_eq!(retrieved.name, "Updated Project");
    }
    
    #[test]
    fn test_list_projects_empty() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let projects = service.list_projects(1, 10).unwrap();
        assert_eq!(projects.len(), 0);
    }
    
    #[test]
    fn test_list_projects_with_data() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        for _ in 0..5 {
            let project = create_test_project();
            service.create_project(&project).unwrap();
        }
        
        let projects = service.list_projects(1, 10).unwrap();
        assert_eq!(projects.len(), 5);
    }
    
    #[test]
    fn test_delete_project() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        service.delete_project(&project_id).unwrap();
        
        let result = service.get_project(&project_id).unwrap();
        assert!(result.is_none());
    }
    
    #[test]
    fn test_count_projects() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        assert_eq!(service.count_projects().unwrap(), 0);
        
        for _ in 0..3 {
            let project = create_test_project();
            service.create_project(&project).unwrap();
        }
        
        assert_eq!(service.count_projects().unwrap(), 3);
    }
    
    // ==================== NARRATIVE OPERATIONS ====================
    
    #[test]
    fn test_create_and_get_narrative() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let retrieved = service.get_narrative(&narrative_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, narrative.title);
    }
    
    #[test]
    fn test_list_narratives_by_project() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        for _ in 0..3 {
            let narrative = create_test_narrative(project_id);
            service.create_narrative(&narrative).unwrap();
        }
        
        let narratives = service.list_narratives_by_project(&project_id).unwrap();
        assert_eq!(narratives.len(), 3);
    }
    
    #[test]
    fn test_update_narrative() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let mut narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        narrative.title = "Updated Narrative".to_string();
        service.update_narrative(&narrative).unwrap();
        
        let retrieved = service.get_narrative(&narrative_id).unwrap().unwrap();
        assert_eq!(retrieved.title, "Updated Narrative");
    }
    
    #[test]
    fn test_delete_narrative() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        service.delete_narrative(&narrative_id).unwrap();
        
        let result = service.get_narrative(&narrative_id).unwrap();
        assert!(result.is_none());
    }
    
    // ==================== STORY ELEMENT OPERATIONS ====================
    
    #[test]
    fn test_create_and_get_story_element() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let element = create_test_story_element(narrative_id);
        let element_id = service.create_story_element(&element).unwrap();
        
        let retrieved = service.get_story_element(&element_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, element.name);
    }
    
    #[test]
    fn test_list_story_elements_by_narrative() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        for _ in 0..3 {
            let element = create_test_story_element(narrative_id);
            service.create_story_element(&element).unwrap();
        }
        
        let elements = service.list_story_elements_by_narrative(&narrative_id).unwrap();
        assert_eq!(elements.len(), 3);
    }
    
    #[test]
    fn test_update_story_element() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let mut element = create_test_story_element(narrative_id);
        let element_id = service.create_story_element(&element).unwrap();
        
        element.name = "Updated Element".to_string();
        service.update_story_element(&element).unwrap();
        
        let retrieved = service.get_story_element(&element_id).unwrap().unwrap();
        assert_eq!(retrieved.name, "Updated Element");
    }
    
    #[test]
    fn test_delete_story_element() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let element = create_test_story_element(narrative_id);
        let element_id = service.create_story_element(&element).unwrap();
        
        service.delete_story_element(&element_id).unwrap();
        
        let result = service.get_story_element(&element_id).unwrap();
        assert!(result.is_none());
    }
    
    // ==================== GAME EVENT OPERATIONS ====================
    
    #[test]
    fn test_create_and_get_game_event() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let event = create_test_game_event(narrative_id);
        let event_id = service.create_game_event(&event).unwrap();
        
        let retrieved = service.get_game_event(&event_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, event.title);
    }
    
    #[test]
    fn test_list_game_events_by_narrative() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        for _ in 0..3 {
            let event = create_test_game_event(narrative_id);
            service.create_game_event(&event).unwrap();
        }
        
        let events = service.list_game_events_by_narrative(&narrative_id).unwrap();
        assert_eq!(events.len(), 3);
    }
    
    #[test]
    fn test_update_game_event() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let mut event = create_test_game_event(narrative_id);
        let event_id = service.create_game_event(&event).unwrap();
        
        event.title = "Updated Event".to_string();
        service.update_game_event(&event).unwrap();
        
        let retrieved = service.get_game_event(&event_id).unwrap().unwrap();
        assert_eq!(retrieved.title, "Updated Event");
    }
    
    #[test]
    fn test_delete_game_event() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let service = PersistenceService::new(db_path).unwrap();
        
        let project = create_test_project();
        let project_id = service.create_project(&project).unwrap();
        
        let narrative = create_test_narrative(project_id);
        let narrative_id = service.create_narrative(&narrative).unwrap();
        
        let event = create_test_game_event(narrative_id);
        let event_id = service.create_game_event(&event).unwrap();
        
        service.delete_game_event(&event_id).unwrap();
        
        let result = service.get_game_event(&event_id).unwrap();
        assert!(result.is_none());
    }
}