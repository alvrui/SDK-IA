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
    
    pub fn calculate_set_compatibility(
        &self, 
        element_ids: &[String], 
        _preset: Option<&str>
    ) -> Result<HashMap<String, f32>, String> {
        let mut scores = HashMap::new();
        
        for i in 0..element_ids.len() {
            for j in (i + 1)..element_ids.len() {
                let a_id = &element_ids[i];
                let b_id = &element_ids[j];
                let result = self.calculate_compatibility(a_id, b_id, None)?;
                let key = format!("{}_{}", a_id, b_id);
                scores.insert(key, result.score);
            }
        }
        
        if !scores.is_empty() {
            let sum: f32 = scores.values().sum();
            let avg = sum / scores.len() as f32;
            scores.insert("internal_coherence".to_string(), avg);
        }
        
        Ok(scores)
    }
    
    fn role_complementarity(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (matches!(a.category, ElementCategory::Protagonist) && matches!(b.category, ElementCategory::Antagonist)) ||
           (matches!(a.category, ElementCategory::Antagonist) && matches!(b.category, ElementCategory::Protagonist)) {
            return 0.95;
        }
        if (a.id == "MENTOR" && b.id == "ACCIDENTAL_HERO") || (a.id == "ACCIDENTAL_HERO" && b.id == "MENTOR") {
            return 0.98;
        }
        if (a.id == "KNIGHT" && b.id == "CORRUPT_OFFICIAL") || (a.id == "CORRUPT_OFFICIAL" && b.id == "KNIGHT") {
            return 0.95;
        }
        if (a.id == "DETECTIVE" && b.id == "CRIMINAL_MASTERMIND") || (a.id == "CRIMINAL_MASTERMIND" && b.id == "DETECTIVE") {
            return 0.95;
        }
        if a.category == b.category {
            return 0.4;
        }
        if (matches!(a.category, ElementCategory::Supporting) && matches!(b.category, ElementCategory::Protagonist)) ||
           (matches!(a.category, ElementCategory::Protagonist) && matches!(b.category, ElementCategory::Supporting)) {
            return 0.7;
        }
        if (matches!(a.category, ElementCategory::Supporting) && matches!(b.category, ElementCategory::Antagonist)) ||
           (matches!(a.category, ElementCategory::Antagonist) && matches!(b.category, ElementCategory::Supporting)) {
            return 0.5;
        }
        0.5
    }
    
    fn moral_tension(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let ma = &a.moral_profile;
        let mb = &b.moral_profile;
        let tension = (ma.idealist - mb.corrupt).abs() +
                      (ma.corrupt - mb.idealist).abs() +
                      (ma.lawful - mb.chaotic).abs() +
                      (ma.chaotic - mb.lawful).abs();
        (tension / 4.0).min(1.0)
    }
    
    fn tone_alignment(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let ta = &a.tone_profile;
        let tb = &b.tone_profile;
        let diff = (ta.serious - tb.serious).abs() +
                   (ta.comic - tb.comic).abs() +
                   (ta.grim - tb.grim).abs() +
                   (ta.adventurous - tb.adventurous).abs() +
                   (ta.melodramatic - tb.melodramatic).abs() +
                   (ta.pulpy - tb.pulpy).abs();
        1.0 - (diff / 6.0)
    }
    
    fn scale_alignment(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let scale_value = |scale: &str| -> f32 {
            for (s, v) in SCALE_VALUES {
                if scale == *s { return *v; }
            }
            0.5
        };
        let sa = scale_value(&a.narrative_scale);
        let sb = scale_value(&b.narrative_scale);
        1.0 - (sa - sb).abs()
    }
    
    fn genre_support(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let a_genres: HashMap<_, _> = a.genre_affinity.iter().map(|s| (s.as_str(), true)).collect();
        let b_genres: HashMap<_, _> = b.genre_affinity.iter().map(|s| (s.as_str(), true)).collect();
        let intersection = a_genres.keys().filter(|k| b_genres.contains_key(*k)).count() as f32;
        let union = (a_genres.len() + b_genres.len()).max(1) as f32;
        (intersection / union).min(1.0)
    }
    
    fn setting_fit(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let a_settings: HashMap<_, _> = a.setting_affinity.iter().map(|s| (s.as_str(), true)).collect();
        let b_settings: HashMap<_, _> = b.setting_affinity.iter().map(|s| (s.as_str(), true)).collect();
        let intersection = a_settings.keys().filter(|k| b_settings.contains_key(*k)).count() as f32;
        let union = (a_settings.len() + b_settings.len()).max(1) as f32;
        (intersection / union).min(1.0)
    }
    
    fn arc_coherence(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (matches!(a.category, ElementCategory::Protagonist) && matches!(b.category, ElementCategory::Theme)) ||
           (matches!(a.category, ElementCategory::Theme) && matches!(b.category, ElementCategory::Protagonist)) {
            let protagonist = if matches!(a.category, ElementCategory::Protagonist) { a } else { b };
            let theme = if matches!(a.category, ElementCategory::Protagonist) { b } else { a };
            let p_drives: HashMap<_, _> = protagonist.core_drives.iter().map(|s| (s.as_str(), true)).collect();
            if theme.id.contains("LOVE") && p_drives.contains_key("love") { return 0.95; }
            if theme.id.contains("JUSTICE") && p_drives.contains_key("justice") { return 0.95; }
            if theme.id.contains("FREEDOM") && p_drives.contains_key("freedom") { return 0.95; }
            if theme.id.contains("WAR") && p_drives.contains_key("survival") { return 0.90; }
            if theme.id.contains("TREASURE") && p_drives.contains_key("greed") { return 0.85; }
            return 0.6;
        }
        if (matches!(a.category, ElementCategory::Theme) && matches!(b.category, ElementCategory::Event)) ||
           (matches!(a.category, ElementCategory::Event) && matches!(b.category, ElementCategory::Theme)) {
            return self.embodiment_strength(a, b);
        }
        0.5
    }
    
    fn embodiment_strength(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (matches!(a.category, ElementCategory::Theme) && matches!(b.category, ElementCategory::Event)) ||
           (matches!(a.category, ElementCategory::Event) && matches!(b.category, ElementCategory::Theme)) {
            let theme = if matches!(a.category, ElementCategory::Theme) { a } else { b };
            let event = if matches!(a.category, ElementCategory::Theme) { b } else { a };
            if theme.id == "THEME_PROTECTING_THE_WITNESS" && event.id == "EVENTS_AMBUSH" { return 0.95; }
            if theme.id == "THEME_LONG_JOURNEY" && event.id == "EVENTS_WORLDWIDE_TRAVELING" { return 0.95; }
            if theme.id == "THEME_FIGHT_FOR_FREEDOM" && event.id == "EVENTS_ESCAPE_CAPTIVITY" { return 0.95; }
            if theme.id == "THEME_AVENGING_LOVED_ONES" && event.id == "EVENTS_FINAL_SHOWDOWN" { return 0.90; }
            if theme.id == "THEME_TREASURE_HUNT" && event.id == "EVENTS_ANCIENT_PUZZLE" { return 0.90; }
            if theme.id == "THEME_SURVIVING_IN_WARTIME" && event.id == "EVENTS_BIG_BATTLE_SCENES" { return 0.90; }
            if theme.id.contains("WAR") && event.id.contains("BATTLE") { return 0.85; }
            if theme.id.contains("LOVE") && event.id.contains("ROMANTIC") { return 0.80; }
            return 0.7;
        }
        0.5
    }
    
    fn arc_payoff(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (matches!(a.category, ElementCategory::Theme) && matches!(b.category, ElementCategory::Finale)) ||
           (matches!(a.category, ElementCategory::Finale) && matches!(b.category, ElementCategory::Theme)) {
            let theme = if matches!(a.category, ElementCategory::Theme) { a } else { b };
            let finale = if matches!(a.category, ElementCategory::Theme) { b } else { a };
            if theme.id == "THEME_UNREQUITED_LOVE" && finale.id == "STARCROSSED_LOVERS" { return 0.95; }
            if theme.id == "THEME_STRUGGLE_FOR_BETTER_LIFE" && finale.id == "PROTAGONISTS_DREAMS_CRUSHED" { return 0.90; }
            if theme.id == "THEME_AVENGING_LOVED_ONES" && finale.id == "ANTAGONIST_GETS_KILLED" { return 0.95; }
            if theme.id == "THEME_FIGHT_FOR_FREEDOM" && finale.id == "PROTAGONIST_FINDS_TREASURE" { return 0.10; }
            if theme.id == "THEME_FIGHT_FOR_FREEDOM" && finale.id == "PROTAGONIST_FINDS_LOVE" { return 0.85; }
            if theme.id == "THEME_LONG_JOURNEY" && finale.id == "PROTAGONIST_RETURNS_HOME" { return 0.95; }
            if theme.id == "THEME_WAR_IS_HELL" && finale.id == "PROTAGONIST_DIES_HEROICALLY" { return 0.95; }
            if theme.id.contains("LOVE") && finale.id.contains("LOVE") { return 0.85; }
            if theme.id.contains("JUSTICE") && finale.id.contains("JUSTICE") { return 0.85; }
            return 0.7;
        }
        0.5
    }
    
    fn core_drives_overlap(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let a_drives: HashMap<_, _> = a.core_drives.iter().map(|s| (s.as_str(), true)).collect();
        let b_drives: HashMap<_, _> = b.core_drives.iter().map(|s| (s.as_str(), true)).collect();
        let intersection = a_drives.keys().filter(|k| b_drives.contains_key(*k)).count() as f32;
        let union = (a_drives.len() + b_drives.len()).max(1) as f32;
        (intersection / union).min(1.0)
    }
    
    fn moral_alignment(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let ma = &a.moral_profile;
        let mb = &b.moral_profile;
        let diff = (ma.idealist - mb.idealist).abs() +
                   (ma.cynical - mb.cynical).abs() +
                   (ma.corrupt - mb.corrupt).abs() +
                   (ma.redemptive - mb.redemptive).abs() +
                   (ma.lawful - mb.lawful).abs() +
                   (ma.chaotic - mb.chaotic).abs();
        1.0 - (diff / 6.0)
    }
    
    fn moral_coherence(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        self.moral_alignment(a, b)
    }
    
    fn natural_fit(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        self.setting_fit(a, b)
    }
    
    fn reinterpretation_fit(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (a.id == "KNIGHT" && b.setting_affinity.iter().any(|s| s.contains("MODERN"))) ||
           (b.id == "KNIGHT" && a.setting_affinity.iter().any(|s| s.contains("MODERN"))) {
            return 0.85;
        }
        if (a.id == "ROBOT" && b.setting_affinity.iter().any(|s| s.contains("MEDIEVAL"))) ||
           (b.id == "ROBOT" && a.setting_affinity.iter().any(|s| s.contains("MEDIEVAL"))) {
            return 0.30;
        }
        if (a.id == "COWBOY" && b.setting_affinity.iter().any(|s| s.contains("SPACE"))) ||
           (b.id == "COWBOY" && a.setting_affinity.iter().any(|s| s.contains("SPACE"))) {
            return 0.60;
        }
        0.5
    }
    
    fn novelty_bonus(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (a.id == "ACCIDENTAL_HERO" && b.id == "ANCIENT_EVIL") ||
           (a.id == "ANCIENT_EVIL" && b.id == "ACCIDENTAL_HERO") {
            return 0.10;
        }
        if (a.id == "ROBOT" && b.setting_affinity.iter().any(|s| s.contains("DYSTOPIAN"))) ||
           (b.id == "ROBOT" && a.setting_affinity.iter().any(|s| s.contains("DYSTOPIAN"))) {
            return 0.15;
        }
        if (a.id == "CLUMSY_OAF" && b.tone_profile.serious > 0.7) ||
           (b.id == "CLUMSY_OAF" && a.tone_profile.serious > 0.7) {
            return 0.05;
        }
        0.0
    }
    
    fn sensitivity_risk(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        if (a.tone_profile.comic > 0.5 && b.content_flags.iter().any(|f| f.contains("extreme_violence"))) ||
           (b.tone_profile.comic > 0.5 && a.content_flags.iter().any(|f| f.contains("extreme_violence"))) {
            return 1.0;
        }
        if (a.tone_profile.serious > 0.7 && b.genre_affinity.iter().any(|g| g.contains("comedy"))) ||
           (b.tone_profile.serious > 0.7 && a.genre_affinity.iter().any(|g| g.contains("comedy"))) {
            return 0.8;
        }
        if a.moral_profile.idealist > 0.7 && b.moral_profile.corrupt > 0.7 {
            return 0.7;
        }
        if (a.tone_profile.grim > 0.7 && b.tone_profile.comic > 0.5) ||
           (b.tone_profile.grim > 0.7 && a.tone_profile.comic > 0.5) {
            return 0.6;
        }
        if (a.tone_profile.melodramatic > 0.7 && b.tone_profile.pulpy > 0.7) ||
           (b.tone_profile.melodramatic > 0.7 && a.tone_profile.pulpy > 0.7) {
            return 0.3;
        }
        0.0
    }
    
    fn calculate_penalties(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let sensitivity = self.sensitivity_risk(a, b);
        if let Some(rules) = self.rules.get("any_any") {
            if let Some(weight) = rules.get("sensitivity_risk") {
                return sensitivity * weight;
            }
        }
        sensitivity * 0.12
    }
    
    fn calculate_bonuses(&self, a: &HollywoodElement, b: &HollywoodElement) -> f32 {
        let novelty = self.novelty_bonus(a, b);
        if let Some(rules) = self.rules.get("any_any") {
            if let Some(weight) = rules.get("novelty_bonus") {
                return novelty * weight;
            }
        }
        novelty * 0.04
    }
    
    fn generate_explanation(
        &self, 
        a: &HollywoodElement, 
        b: &HollywoodElement,
        axis_scores: &HashMap<String, f32>,
        penalties: f32,
        bonuses: f32
    ) -> Vec<String> {
        let mut explanations = Vec::new();
        
        for (axis, score) in axis_scores {
            if *score >= 0.8 {
                match axis.as_str() {
                    "role_complementarity" => explanations.push(format!(
                        "ROLE_COMPLEMENTARITY: {} and {} have complementary narrative roles", a.id, b.id)),
                    "moral_tension" => explanations.push(format!(
                        "MORAL_TENSION: Strong ethical contrast between {} and {}", a.id, b.id)),
                    "tone_alignment" => explanations.push(format!(
                        "TONE_ALIGNMENT: {} and {} share similar tonal profiles", a.id, b.id)),
                    "embodiment_strength" => {
                        let event_id = if matches!(b.category, ElementCategory::Event) { &b.id } else { &a.id };
                        let theme_id = if matches!(a.category, ElementCategory::Theme) { &a.id } else { &b.id };
                        explanations.push(format!("EMBODIMENT: {} strongly embodies the theme of {}", event_id, theme_id));
                    }
                    "arc_payoff" => {
                        let finale_id = if matches!(b.category, ElementCategory::Finale) { &b.id } else { &a.id };
                        let theme_id = if matches!(a.category, ElementCategory::Theme) { &a.id } else { &b.id };
                        explanations.push(format!("ARC_PAYOFF: {} provides strong payoff for theme {}", finale_id, theme_id));
                    }
                    _ => {}
                }
            }
        }
        
        if penalties > 0.0 {
            if (a.tone_profile.comic > 0.5 && b.content_flags.iter().any(|f| f.contains("extreme_violence"))) ||
               (b.tone_profile.comic > 0.5 && a.content_flags.iter().any(|f| f.contains("extreme_violence"))) {
                explanations.push("PENALTY: Comic tone clashes with extreme violence content".to_string());
            }
            if (a.tone_profile.serious > 0.7 && b.genre_affinity.iter().any(|g| g.contains("comedy"))) ||
               (b.tone_profile.serious > 0.7 && a.genre_affinity.iter().any(|g| g.contains("comedy"))) {
                explanations.push("PENALTY: Serious tone clashes with comedy genre".to_string());
            }
            if a.moral_profile.idealist > 0.7 && b.moral_profile.corrupt > 0.7 {
                explanations.push("PENALTY: Idealist and corrupt profiles create moral dissonance".to_string());
            }
        }
        
        if bonuses > 0.0 {
            if (a.id == "ACCIDENTAL_HERO" && b.id == "ANCIENT_EVIL") ||
               (a.id == "ANCIENT_EVIL" && b.id == "ACCIDENTAL_HERO") {
                explanations.push("BONUS: Rare but valid combination of accidental hero vs ancient evil".to_string());
            }
            if (a.id == "ROBOT" && b.setting_affinity.iter().any(|s| s.contains("DYSTOPIAN"))) ||
               (b.id == "ROBOT" && a.setting_affinity.iter().any(|s| s.contains("DYSTOPIAN"))) {
                explanations.push("BONUS: Robot in dystopian setting receives contextual boost".to_string());
            }
        }
        
        explanations
    }
}
