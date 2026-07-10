use std::collections::HashMap;
use std::sync::Arc;
use crate::domain::{Narrative, StoryElement, GameEvent, StoryElementType};
use crate::services::persistence::PersistenceService;
use crate::domain::hollywood_animal::CompatibilityMatrix;
use uuid::Uuid;

/// Validation error type
#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

impl ValidationError {
    pub fn new(field: &str, message: &str, severity: ValidationSeverity) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            severity,
        }
    }

    pub fn error(field: &str, message: &str) -> Self {
        Self::new(field, message, ValidationSeverity::Error)
    }

    pub fn warning(field: &str, message: &str) -> Self {
        Self::new(field, message, ValidationSeverity::Warning)
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.valid = false;
    }

    pub fn add_warning(&mut self, warning: ValidationError) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.valid = self.valid && other.valid;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

/// Domain validation service
pub struct DomainValidationService {
    persistence: Arc<PersistenceService>,
    compatibility_matrix: Arc<CompatibilityMatrix>,
}

impl DomainValidationService {
    pub fn new(
        persistence: Arc<PersistenceService>,
        compatibility_matrix: Arc<CompatibilityMatrix>,
    ) -> Self {
        Self {
            persistence,
            compatibility_matrix,
        }
    }

    /// Validate a complete narrative structure
    pub fn validate_narrative(&self, narrative_id: Uuid) -> Result<ValidationResult, String> {
        let mut result = ValidationResult::new();
        
        let narrative = self.persistence.get_narrative(&narrative_id)?
            .ok_or("Narrative not found")?;
        let elements = self.persistence.list_story_elements_by_narrative(&narrative_id)?;
        
        for element in &elements {
            let element_result = self.validate_story_element(&element);
            result.merge(element_result);
        }
        
        Ok(result)
    }

    /// Validate a single story element
    pub fn validate_story_element(&self, element: &StoryElement) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        if !self.compatibility_matrix.elements.contains_key(&element.hollywood_element_id) {
            result.add_error(ValidationError::error(
                "hollywood_element_id",
                &format!("Unknown Hollywood element: {}", element.hollywood_element_id)
            ));
        }
        
        result
    }

    /// Validate a game event
    pub fn validate_game_event(&self, event: &GameEvent, elements: &[StoryElement]) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        for element in elements {
            if element.id == event.trigger_element_id {
                let element_result = self.validate_story_element(element);
                result.merge(element_result);
            }
        }
        
        result
    }

    /// Validate a complete project
    pub fn validate_project(&self, project_id: Uuid) -> Result<ValidationResult, String> {
        let mut result = ValidationResult::new();
        
        let narratives = self.persistence.list_narratives_by_project(&project_id)?;
        for narrative in &narratives {
            match self.validate_narrative(narrative.id) {
                Ok(narrative_result) => result.merge(narrative_result),
                Err(e) => result.add_error(ValidationError::error("narrative_validation", &e)),
            }
        }
        
        Ok(result)
    }
}