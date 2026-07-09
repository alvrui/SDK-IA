use std::collections::HashMap;
use std::sync::Arc;
use crate::domain::{Narrative, StoryElement, GameEvent, StoryElementType};
use crate::services::persistence::PersistenceService;
use crate::domain::hollywood_animal::CompatibilityMatrix;
use uuid::Uuid;

/// Validation error type
#[derive(Debug, Clone)]
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
        
        // Get narrative and its elements
        let narrative = self.persistence.get_narrative(&narrative_id)?
            .ok_or("Narrative not found")?;
        let elements = self.persistence.list_story_elements_by_narrative(&narrative_id)?;
        let events = self.persistence.list_game_events_by_narrative(&narrative_id)?;
        
        // Rule 1: Exactly 1 protagonist
        self.validate_exactly_one_element(&mut result, &elements, StoryElementType::Protagonist, "protagonist");
        
        // Rule 2: Exactly 1 antagonist
        self.validate_exactly_one_element(&mut result, &elements, StoryElementType::Antagonist, "antagonist");
        
        // Rule 3: Exactly 1 finale
        self.validate_exactly_one_element(&mut result, &elements, StoryElementType::Finale, "finale");
        
        // Rule 4: At least 1 theme
        self.validate_at_least_one_element(&mut result, &elements, StoryElementType::Theme, "theme", 1);
        
        // Rule 5: At least 1 location
        self.validate_at_least_one_element(&mut result, &elements, StoryElementType::Location, "location", 1);
        
        // Rule 6: Temporal coherence of events
        self.validate_temporal_coherence(&mut result, &events);
        
        // Rule 7: Character consistency in events
        self.validate_character_consistency(&mut result, &elements, &events);
        
        // Rule 8: Location consistency in events
        self.validate_location_consistency(&mut result, &elements, &events);
        
        // Rule 9: Hollywood Animal element validation
        self.validate_hollywood_elements(&mut result, &elements);
        
        // Rule 10: Theme validation in narrative
        self.validate_narrative_themes(&mut result, &narrative);
        
        // Rule 11: Minimum compatibility score
        self.validate_compatibility_score(&mut result, &narrative);
        
        Ok(result)
    }

    /// Validate exactly one element of a specific type
    fn validate_exactly_one_element(
        &self,
        result: &mut ValidationResult,
        elements: &[StoryElement],
        element_type: StoryElementType,
        field_name: &str,
    ) {
        let count = elements.iter().filter(|e| e.element_type == element_type).count();
        
        if count == 0 {
            result.add_error(ValidationError::error(
                field_name,
                &format!("Narrative must have exactly 1 {}, found {}", field_name, count)
            ));
        } else if count > 1 {
            result.add_error(ValidationError::error(
                field_name,
                &format!("Narrative must have exactly 1 {}, found {}", field_name, count)
            ));
        }
    }

    /// Validate at least N elements of a specific type
    fn validate_at_least_one_element(
        &self,
        result: &mut ValidationResult,
        elements: &[StoryElement],
        element_type: StoryElementType,
        field_name: &str,
        min_count: usize,
    ) {
        let count = elements.iter().filter(|e| e.element_type == element_type).count();
        
        if count < min_count {
            result.add_warning(ValidationError::warning(
                field_name,
                &format!("Narrative should have at least {} {}, found {}", min_count, field_name, count)
            ));
        }
    }

    /// Validate temporal coherence of events
    fn validate_temporal_coherence(
        &self,
        result: &mut ValidationResult,
        events: &[GameEvent],
    ) {
        // Only validate if events have timestamps
        let timestamped_events: Vec<_> = events.iter()
            .filter(|e| e.timestamp.is_some())
            .collect();
        
        if timestamped_events.len() < 2 {
            return; // No need to validate with less than 2 timestamped events
        }
        
        // Sort by timestamp
        let mut sorted_events: Vec<_> = timestamped_events.iter().collect();
        sorted_events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        // Check if order_index matches timestamp order
        for i in 0..sorted_events.len() {
            for j in (i + 1)..sorted_events.len() {
                let a = sorted_events[i];
                let b = sorted_events[j];
                
                if let (Some(ts_a), Some(ts_b)) = (a.timestamp, b.timestamp) {
                    if ts_a > ts_b {
                        result.add_error(ValidationError::error(
                            "temporal_coherence",
                            &format!("Event '{}' has later timestamp than '{}' but lower order", a.title, b.title)
                        ));
                    }
                }
            }
        }
        
        // Check order_index consistency
        for i in 0..events.len() {
            for j in (i + 1)..events.len() {
                if events[i].order_index >= events[j].order_index {
                    result.add_warning(ValidationError::warning(
                        "event_order",
                        &format!("Events should have increasing order_index values")
                    ));
                    break;
                }
            }
        }
    }

    /// Validate character consistency in events
    fn validate_character_consistency(
        &self,
        result: &mut ValidationResult,
        elements: &[StoryElement],
        events: &[GameEvent],
    ) {
        // Get all character elements (protagonist, antagonist, supporting)
        let character_elements: Vec<_> = elements.iter()
            .filter(|e| matches!(e.element_type, StoryElementType::Protagonist | StoryElementType::Antagonist))
            .collect();
        
        if character_elements.is_empty() {
            return;
        }
        
        // Check that events reference valid characters
        for event in events {
            for char_id in &event.character_ids {
                if !elements.iter().any(|e| e.id == *char_id) {
                    result.add_error(ValidationError::error(
                        "character_reference",
                        &format!("Event '{}' references non-existent character: {}", event.title, char_id)
                    ));
                }
            }
        }
        
        // Check that protagonist appears in at least one event (optional warning)
        if let Some(protagonist) = elements.iter().find(|e| e.is_protagonist()) {
            let protagonist_in_events = events.iter()
                .any(|e| e.character_ids.contains(&protagonist.id));
            if !protagonist_in_events {
                result.add_warning(ValidationError::warning(
                    "protagonist_usage",
                    &format!("Protagonist '{}' does not appear in any event", protagonist.name)
                ));
            }
        }
        
        // Check that antagonist appears in at least one event (optional warning)
        if let Some(antagonist) = elements.iter().find(|e| e.is_antagonist()) {
            let antagonist_in_events = events.iter()
                .any(|e| e.character_ids.contains(&antagonist.id));
            if !antagonist_in_events {
                result.add_warning(ValidationError::warning(
                    "antagonist_usage",
                    &format!("Antagonist '{}' does not appear in any event", antagonist.name)
                ));
            }
        }
    }

    /// Validate location consistency in events
    fn validate_location_consistency(
        &self,
        result: &mut ValidationResult,
        elements: &[StoryElement],
        events: &[GameEvent],
    ) {
        // Get all location elements
        let location_elements: Vec<_> = elements.iter()
            .filter(|e| e.is_location())
            .collect();
        
        if location_elements.is_empty() {
            return;
        }
        
        // Check that events reference valid locations
        for event in events {
            for loc_id in &event.location_ids {
                if !elements.iter().any(|e| e.id == *loc_id) {
                    result.add_error(ValidationError::error(
                        "location_reference",
                        &format!("Event '{}' references non-existent location: {}", event.title, loc_id)
                    ));
                }
            }
        }
        
        // Check that each event has at least one location (optional warning)
        for event in events {
            if event.location_ids.is_empty() {
                result.add_warning(ValidationError::warning(
                    "event_location",
                    &format!("Event '{}' has no location specified", event.title)
                ));
            }
        }
    }

    /// Validate Hollywood Animal element references
    fn validate_hollywood_elements(
        &self,
        result: &mut ValidationResult,
        elements: &[StoryElement],
    ) {
        for element in elements {
            if !self.compatibility_matrix.elements.contains_key(&element.hollywood_element_id) {
                result.add_error(ValidationError::error(
                    "hollywood_element",
                    &format!("Story element '{}' references unknown Hollywood element: {}", 
                            element.name, element.hollywood_element_id)
                ));
            }
        }
    }

    /// Validate narrative theme references
    fn validate_narrative_themes(
        &self,
        result: &mut ValidationResult,
        narrative: &Narrative,
    ) {
        for theme_id in &narrative.theme_ids {
            if !self.compatibility_matrix.elements.contains_key(theme_id) {
                result.add_error(ValidationError::error(
                    "theme_reference",
                    &format!("Narrative references unknown theme: {}", theme_id)
                ));
            }
        }
    }

    /// Validate compatibility score threshold
    fn validate_compatibility_score(
        &self,
        result: &mut ValidationResult,
        narrative: &Narrative,
    ) {
        // Default threshold is 0.6, but can be configured
        let threshold = 0.6;
        
        if narrative.compatibility_score < threshold {
            result.add_warning(ValidationError::warning(
                "compatibility_score",
                &format!("Narrative compatibility score ({:.2}) is below recommended threshold ({:.2})", 
                        narrative.compatibility_score, threshold)
            ));
        }
    }

    /// Validate a story element
    pub fn validate_story_element(&self, element: &StoryElement) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check Hollywood element reference
        if !self.compatibility_matrix.elements.contains_key(&element.hollywood_element_id) {
            result.add_error(ValidationError::error(
                "hollywood_element",
                &format!("References unknown Hollywood element: {}", element.hollywood_element_id)
            ));
        }
        
        // Check name is not empty
        if element.name.trim().is_empty() {
            result.add_error(ValidationError::error(
                "name",
                "Story element name cannot be empty"
            ));
        }
        
        result
    }

    /// Validate a game event
    pub fn validate_game_event(&self, event: &GameEvent, elements: &[StoryElement]) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Check event type
        if event.title.trim().is_empty() {
            result.add_error(ValidationError::error(
                "title",
                "Game event title cannot be empty"
            ));
        }
        
        // Check character references
        for char_id in &event.character_ids {
            if !elements.iter().any(|e| e.id == *char_id) {
                result.add_error(ValidationError::error(
                    "character_reference",
                    &format!("References non-existent character: {}", char_id)
                ));
            }
        }
        
        // Check location references
        for loc_id in &event.location_ids {
            if !elements.iter().any(|e| e.id == *loc_id) {
                result.add_error(ValidationError::error(
                    "location_reference",
                    &format!("References non-existent location: {}", loc_id)
                ));
            }
        }
        
        // Check Hollywood event reference if provided
        if let Some(ref event_id) = event.hollywood_event_id {
            if !self.compatibility_matrix.elements.contains_key(event_id) {
                result.add_error(ValidationError::error(
                    "hollywood_event",
                    &format!("References unknown Hollywood event: {}", event_id)
                ));
            }
        }
        
        result
    }

    /// Validate project structure
    pub fn validate_project(&self, project_id: Uuid) -> Result<ValidationResult, String> {
        let mut result = ValidationResult::new();
        
        // Get project narratives
        let narratives = self.persistence.list_narratives_by_project(&project_id)?;
        
        if narratives.is_empty() {
            result.add_warning(ValidationError::warning(
                "narratives",
                "Project has no narratives"
            ));
            return Ok(result);
        }
        
        // Validate each narrative
        for narrative in &narratives {
            let narrative_result = self.validate_narrative(narrative.id)?;
            result.merge(narrative_result);
        }
        
        Ok(result)
    }
}