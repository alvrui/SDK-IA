use std::collections::HashMap;
use crate::domain::hollywood_animal::{
    HollywoodElement, CompatibilityMatrix, CompatibilityResult, CompatibilityCategory, ElementCategory
};

const SCALE_VALUES: &[(&str, f32)] = &[
    ("intimate", 0.0),
    ("personal", 0.25),
    ("local", 0.5),
    ("societal", 0.75),
    ("epic", 1.0),
];

impl CompatibilityMatrix {
    pub fn calculate_compatibility(
        &self, 
        a_id: &str, 
        b_id: &str, 
        _preset: Option<&str>
    ) -> Result<CompatibilityResult, String> {
        let a = self.elements.get(a_id)
            .ok_or_else(|| format!("Element {} not found", a_id))?;
        let b = self.elements.get(b_id)
            .ok_or_else(|| format!("Element {} not found", b_id))?;
        
        let pair_type = self.get_pair_type(a, b);
        let rules = self.rules.get(&pair_type)
            .cloned()
            .or_else(|| self.rules.get("any_any").cloned())
            .ok_or("No rules found for pair type")?;
        
        let mut axis_scores = HashMap::new();
        let mut total = 0.0;
        
        for (axis, weight) in &rules {
            let score = self.calculate_axis_score(axis, a, b);
            axis_scores.insert(axis.clone(), score);
            total += weight * score;
        }
        
        let penalties = self.calculate_penalties(a, b);
        let bonuses = self.calculate_bonuses(a, b);
        let mut final_score = total - penalties + bonuses;
        final_score = final_score.clamp(0.0, 1.0);
        
        let category = CompatibilityResult::discretize_score(final_score);
        let explanation = self.generate_explanation(a, b, &axis_scores, penalties, bonuses);
        
        Ok(CompatibilityResult {
            score: final_score,
            category,
            axis_scores,
            penalties: vec![format!("Total penalty: {:.4}", penalties)],
            bonuses: vec![format!("Total bonus: {:.4}", bonuses)],
            explanation,
        })
    }