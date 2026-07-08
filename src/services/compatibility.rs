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
    
    pub fn calculate_axis_score(&self, axis: &str, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        match axis {
            "role_complementarity" => self.role_complementarity(a, b),
            "moral_tension" => self.moral_tension(a, b),
            "tone_alignment" => self.tone_alignment(a, b),
            "scale_alignment" => self.scale_alignment(a, b),
            "genre_support" => self.genre_support(a, b),
            "setting_fit" => self.setting_fit(a, b),
            "arc_coherence" => self.arc_coherence(a, b),
            "embodiment_strength" => self.embodiment_strength(a, b),
            "arc_payoff" => self.arc_payoff(a, b),
            "core_drives_overlap" => self.core_drives_overlap(a, b),
            "moral_alignment" => self.moral_alignment(a, b),
            "moral_coherence" => self.moral_coherence(a, b),
            "natural_fit" => self.natural_fit(a, b),
            "reinterpretation_fit" => self.reinterpretation_fit(a, b),
            "novelty_bonus" => self.novelty_bonus(a, b),
            "sensitivity_risk" => self.sensitivity_risk(a, b),
            _ => 0.5,
        }
    }