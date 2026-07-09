// Unit tests for VersioningService

use std::str::FromStr;
use crate::services::versioning::{VersioningService, VersionChangeType};
use crate::domain::{Project, Narrative, ProjectStatus, NarrativeStatus};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use serde_json;

fn create_test_project(version: &str) -> Project {
    Project {
        id: Uuid::new_v4(),
        name: "Test Project".to_string(),
        description: "Test Description".to_string(),
        author: "Test Author".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: version.to_string(),
        status: ProjectStatus::Draft,
        tags: vec![],
        settings: serde_json::Value::Null,
        metadata: serde_json::Value::Null,
    }
}

fn create_test_narrative(version: &str) -> Narrative {
    Narrative {
        id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        title: "Test Narrative".to_string(),
        synopsis: "Test Synopsis".to_string(),
        status: NarrativeStatus::Draft,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: version.to_string(),
        theme_ids: vec![],
        compatibility_score: 0.8,
        context_summary: "".to_string(),
        metadata: serde_json::Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // ==================== VERSION PARSING ====================
    
    #[test]
    fn test_parse_version_valid() {
        assert_eq!(
            VersioningService::parse_version("1.2.3"),
            Some((1, 2, 3))
        );
        assert_eq!(
            VersioningService::parse_version("0.0.0"),
            Some((0, 0, 0))
        );
    }
    
    #[test]
    fn test_parse_version_invalid() {
        assert_eq!(VersioningService::parse_version("1.2"), None);
        assert_eq!(VersioningService::parse_version("invalid"), None);
        assert_eq!(VersioningService::parse_version(""), None);
    }
    
    // ==================== VERSION FORMATTING ====================
    
    #[test]
    fn test_format_version() {
        assert_eq!(
            VersioningService::format_version(1, 2, 3),
            "1.2.3"
        );
    }
    
    // ==================== VERSION INCREMENT ====================
    
    #[test]
    fn test_increment_version_major() {
        assert_eq!(
            VersioningService::increment_version("1.2.3", VersionChangeType::Major),
            "2.0.0"
        );
    }
    
    #[test]
    fn test_increment_version_minor() {
        assert_eq!(
            VersioningService::increment_version("1.2.3", VersionChangeType::Minor),
            "1.3.0"
        );
    }
    
    #[test]
    fn test_increment_version_patch() {
        assert_eq!(
            VersioningService::increment_version("1.2.3", VersionChangeType::Patch),
            "1.2.4"
        );
    }
    
    #[test]
    fn test_increment_version_invalid_defaults() {
        assert_eq!(
            VersioningService::increment_version("invalid", VersionChangeType::Major),
            "2.0.0"
        );
        assert_eq!(
            VersioningService::increment_version("invalid", VersionChangeType::Minor),
            "1.1.0"
        );
        assert_eq!(
            VersioningService::increment_version("invalid", VersionChangeType::Patch),
            "1.0.1"
        );
    }
    
    // ==================== PROJECT CHANGE TYPE DETECTION ====================
    
    #[test]
    fn test_project_change_type_patch_description() {
        let original = create_test_project("1.0.0");
        let mut updated = original.clone();
        updated.description = "New Description".to_string();
        
        let change_type = VersioningService::determine_project_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Patch);
    }
    
    #[test]
    fn test_project_change_type_patch_tags() {
        let original = create_test_project("1.0.0");
        let mut updated = original.clone();
        updated.tags = vec!["new-tag".to_string()];
        
        let change_type = VersioningService::determine_project_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Patch);
    }
    
    #[test]
    fn test_project_change_type_minor_name() {
        let original = create_test_project("1.0.0");
        let mut updated = original.clone();
        updated.name = "New Name".to_string();
        
        let change_type = VersioningService::determine_project_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_project_change_type_minor_status() {
        let original = create_test_project("1.0.0");
        let mut updated = original.clone();
        updated.status = ProjectStatus::Active;
        
        let change_type = VersioningService::determine_project_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_project_change_type_minor_author() {
        let original = create_test_project("1.0.0");
        let mut updated = original.clone();
        updated.author = "New Author".to_string();
        
        let change_type = VersioningService::determine_project_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    // ==================== NARRATIVE CHANGE TYPE DETECTION ====================
    
    #[test]
    fn test_narrative_change_type_patch_synopsis() {
        let original = create_test_narrative("1.0.0");
        let mut updated = original.clone();
        updated.synopsis = "New Synopsis".to_string();
        
        let change_type = VersioningService::determine_narrative_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Patch);
    }
    
    #[test]
    fn test_narrative_change_type_minor_title() {
        let original = create_test_narrative("1.0.0");
        let mut updated = original.clone();
        updated.title = "New Title".to_string();
        
        let change_type = VersioningService::determine_narrative_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_narrative_change_type_minor_status() {
        let original = create_test_narrative("1.0.0");
        let mut updated = original.clone();
        updated.status = NarrativeStatus::Active;
        
        let change_type = VersioningService::determine_narrative_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_narrative_change_type_minor_theme_ids() {
        let original = create_test_narrative("1.0.0");
        let mut updated = original.clone();
        updated.theme_ids = vec!["THEME_001".to_string()];
        
        let change_type = VersioningService::determine_narrative_change_type(&original, &updated);
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    // ==================== CREATION/DELETION CHANGE TYPES ====================
    
    #[test]
    fn test_story_element_creation_is_minor() {
        let change_type = VersioningService::determine_story_element_change_type();
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_story_element_deletion_is_minor() {
        let change_type = VersioningService::determine_story_element_deletion_change_type();
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_game_event_creation_is_minor() {
        let change_type = VersioningService::determine_game_event_change_type();
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    #[test]
    fn test_narrative_creation_is_minor() {
        let change_type = VersioningService::determine_narrative_creation_change_type();
        assert_eq!(change_type, VersionChangeType::Minor);
    }
    
    // ==================== VERSION CHANGE TYPE FROM STRING ====================
    
    #[test]
    fn test_version_change_type_from_str() {
        assert_eq!(
            "patch".parse::<VersionChangeType>().unwrap(),
            VersionChangeType::Patch
        );
        assert_eq!(
            "minor".parse::<VersionChangeType>().unwrap(),
            VersionChangeType::Minor
        );
        assert_eq!(
            "major".parse::<VersionChangeType>().unwrap(),
            VersionChangeType::Major
        );
    }
    
    #[test]
    fn test_version_change_type_from_str_case_insensitive() {
        assert_eq!(
            "PATCH".parse::<VersionChangeType>().unwrap(),
            VersionChangeType::Patch
        );
        assert_eq!(
            "MINOR".parse::<VersionChangeType>().unwrap(),
            VersionChangeType::Minor
        );
    }
    
    #[test]
    fn test_version_change_type_from_str_invalid() {
        assert!("invalid".parse::<VersionChangeType>().is_err());
        assert!("".parse::<VersionChangeType>().is_err());
    }
}