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