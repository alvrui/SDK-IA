use std::collections::HashMap;
use std::sync::Arc;
use crate::domain::{Narrative, StoryElement, StoryElementType, Project};
use crate::domain::hollywood_animal::{CompatibilityMatrix, CompatibilityResult};
use crate::services::persistence::PersistenceService;
use uuid::Uuid;

/// Service for narrative business logic
pub struct NarrativeService {
    persistence: Arc<PersistenceService>,
    compatibility_matrix: Arc<CompatibilityMatrix>,
}

impl NarrativeService {
    pub fn new(
        persistence: Arc<PersistenceService>,
        compatibility_matrix: Arc<CompatibilityMatrix>,
    ) -> Self {
        Self {
            persistence,
            compatibility_matrix,
        }
    }

    /// Create a narrative and calculate its compatibility score
    pub fn create_narrative_with_compatibility(
        &self,
        project_id: Uuid,
        title: String,
        synopsis: String,
        theme_ids: Vec<String>,
        metadata: HashMap<String, String>,
    ) -> Result<Uuid, String> {
        let mut narrative = Narrative::new(project_id, title, synopsis);
        narrative.theme_ids = theme_ids;
        narrative.metadata = metadata;
        
        // Create narrative in database first
        let narrative_id = self.persistence.create_narrative(&narrative)?;
        
        // Calculate compatibility score based on themes
        let score = self.calculate_theme_compatibility(&narrative.theme_ids)?;
        narrative.compatibility_score = score;
        narrative.id = narrative_id;
        
        // Update with compatibility score
        self.persistence.update_narrative(&narrative)?;
        
        Ok(narrative_id)
    }

    /// Calculate compatibility score for a set of theme IDs
    fn calculate_theme_compatibility(&self, theme_ids: &[String]) -> Result<f32, String> {
        if theme_ids.len() < 2 {
            return Ok(1.0); // Single theme or none is perfectly compatible
        }
        
        let mut total_score = 0.0;
        let mut count = 0;
        
        for i in 0..theme_ids.len() {
            for j in (i + 1)..theme_ids.len() {
                let a_id = &theme_ids[i];
                let b_id = &theme_ids[j];
                
                // Check if both elements exist in the matrix
                if self.compatibility_matrix.elements.contains_key(a_id) 
                    && self.compatibility_matrix.elements.contains_key(b_id) {
                    if let Ok(result) = self.compatibility_matrix.calculate_compatibility(a_id, b_id, None) {
                        total_score += result.score;
                        count += 1;
                    }
                }
            }
        }
        
        if count > 0 {
            Ok(total_score / count as f32)
        } else {
            Ok(0.5) // Default neutral score
        }
    }

    /// Add a story element to a narrative and recalculate compatibility
    pub fn add_story_element_and_recalculate(
        &self,
        narrative_id: Uuid,
        element: &StoryElement,
    ) -> Result<(), String> {
        // Create the story element
        self.persistence.create_story_element(element)?;
        
        // Recalculate compatibility for the narrative
        self.recalculate_narrative_compatibility(narrative_id)?;
        
        Ok(())
    }

    /// Recalculate compatibility score for a narrative based on all its story elements
    pub fn recalculate_narrative_compatibility(&self, narrative_id: Uuid) -> Result<(), String> {
        // Get all story elements for this narrative
        let elements = self.persistence.list_story_elements_by_narrative(&narrative_id)?;
        
        if elements.is_empty() {
            // If no elements, set to neutral score
            let mut narrative = self.persistence.get_narrative(&narrative_id)?
                .ok_or("Narrative not found")?;
            narrative.compatibility_score = 0.5;
            self.persistence.update_narrative(&narrative)?;
            return Ok(());
        }
        
        // Calculate average compatibility between all pairs
        let mut total_score = 0.0;
        let mut count = 0;
        
        for i in 0..elements.len() {
            for j in (i + 1)..elements.len() {
                let a_id = &elements[i].hollywood_element_id;
                let b_id = &elements[j].hollywood_element_id;
                
                // Check if both elements exist in the matrix
                if self.compatibility_matrix.elements.contains_key(a_id)
                    && self.compatibility_matrix.elements.contains_key(b_id) {
                    if let Ok(result) = self.compatibility_matrix.calculate_compatibility(a_id, b_id, None) {
                        total_score += result.score;
                        count += 1;
                    }
                }
            }
        }
        
        let score = if count > 0 {
            total_score / count as f32
        } else {
            0.5 // Default neutral score
        };
        
        // Update narrative with new compatibility score
        let mut narrative = self.persistence.get_narrative(&narrative_id)?
            .ok_or("Narrative not found")?;
        narrative.compatibility_score = score;
        narrative.context_summary = self.generate_context_summary(&elements);
        self.persistence.update_narrative(&narrative)?;
        
        Ok(())
    }

    /// Generate a context summary for AI based on story elements
    fn generate_context_summary(&self, elements: &[StoryElement]) -> String {
        let mut summary = String::new();
        
        // Add protagonist
        if let Some(protagonist) = elements.iter().find(|e| e.is_protagonist()) {
            summary.push_str(&format!("Protagonist: {} ({}). ", protagonist.name, protagonist.hollywood_element_id));
        }
        
        // Add antagonist
        if let Some(antagonist) = elements.iter().find(|e| e.is_antagonist()) {
            summary.push_str(&format!("Antagonist: {} ({}). ", antagonist.name, antagonist.hollywood_element_id));
        }
        
        // Add finale
        if let Some(finale) = elements.iter().find(|e| e.is_finale()) {
            summary.push_str(&format!("Finale: {} ({}). ", finale.name, finale.hollywood_element_id));
        }
        
        // Add locations
        let locations: Vec<_> = elements.iter().filter(|e| e.is_location()).collect();
        if !locations.is_empty() {
            summary.push_str("Locations: ");
            for loc in &locations {
                summary.push_str(&format!("{} ({}), ", loc.name, loc.hollywood_element_id));
            }
            summary.pop(); // Remove trailing comma and space
            summary.push_str(". ");
        }
        
        // Add themes
        let themes: Vec<_> = elements.iter().filter(|e| e.is_theme()).collect();
        if !themes.is_empty() {
            summary.push_str("Themes: ");
            for theme in &themes {
                summary.push_str(&format!("{} ({}), ", theme.name, theme.hollywood_element_id));
            }
            summary.pop();
            summary.push_str(".");
        }
        
        summary
    }

    /// Validate that a Hollywood element ID exists in the catalog
    pub fn validate_hollywood_element(&self, element_id: &str) -> bool {
        self.compatibility_matrix.elements.contains_key(element_id)
    }

    /// Validate all story elements in a narrative
    pub fn validate_narrative_elements(&self, narrative_id: Uuid) -> Result<Vec<String>, String> {
        let elements = self.persistence.list_story_elements_by_narrative(&narrative_id)?;
        let mut errors = Vec::new();
        
        for element in &elements {
            if !self.validate_hollywood_element(&element.hollywood_element_id) {
                errors.push(format!(
                    "Story element '{}' references unknown Hollywood element: {}",
                    element.name, element.hollywood_element_id
                ));
            }
        }
        
        Ok(errors)
    }

    /// Get narrative with all its story elements
    pub fn get_narrative_with_elements(&self, narrative_id: Uuid) -> Result<(Narrative, Vec<StoryElement>), String> {
        let narrative = self.persistence.get_narrative(&narrative_id)?
            .ok_or("Narrative not found")?;
        let elements = self.persistence.list_story_elements_by_narrative(&narrative_id)?;
        Ok((narrative, elements))
    }

    /// Get all narratives for a project with their elements
    pub fn get_project_narratives_with_elements(&self, project_id: Uuid) -> Result<Vec<(Narrative, Vec<StoryElement>)>, String> {
        let narratives = self.persistence.list_narratives_by_project(&project_id)?;
        let mut results = Vec::new();
        
        for narrative in &narratives {
            let elements = self.persistence.list_story_elements_by_narrative(&narrative.id)?;
            results.push((narrative.clone(), elements));
        }
        
        Ok(results)
    }
}
