use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

/// Categories of Hollywood Animal elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ElementCategory {
    Protagonist,
    Antagonist,
    Supporting,
    Event,
    Theme,
    Finale,
}

impl FromStr for ElementCategory {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "protagonist" => Ok(ElementCategory::Protagonist),
            "antagonist" => Ok(ElementCategory::Antagonist),
            "supporting" => Ok(ElementCategory::Supporting),
            "event" => Ok(ElementCategory::Event),
            "theme" => Ok(ElementCategory::Theme),
            "finale" => Ok(ElementCategory::Finale),
            _ => Err(format!("Unknown category: {}", s)),
        }
    }
}

/// Tone profile for narrative elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneProfile {
    pub serious: f32,
    pub comic: f32,
    pub grim: f32,
    pub adventurous: f32,
    pub melodramatic: f32,
    pub pulpy: f32,
}

/// Moral profile for narrative elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralProfile {
    pub idealist: f32,
    pub cynical: f32,
    pub corrupt: f32,
    pub redemptive: f32,
    pub lawful: f32,
    pub chaotic: f32,
}

/// Realism profile for narrative elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealismProfile {
    pub grounded: f32,
    pub heightened: f32,
    pub fantastical: f32,
    pub supernatural: f32,
    #[serde(rename = "sci_fi")]
    pub sci_fi: f32,
}

/// A Hollywood Animal element with all its metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HollywoodElement {
    pub id: String,
    pub category: ElementCategory,
    pub subtype: String,
    pub narrative_scale: String,
    pub tone_profile: ToneProfile,
    pub moral_profile: MoralProfile,
    pub realism_profile: RealismProfile,
    pub agency_type: String,
    pub core_drives: Vec<String>,
    pub content_flags: Vec<String>,
    pub genre_affinity: Vec<String>,
    pub setting_affinity: Vec<String>,
}

impl HollywoodElement {
    pub fn from_csv_row(row: &csv::StringRecord) -> Result<Self, String> {
        // CSV column indices based on expected order:
        // 0: id, 1: category, 2: subtype, 3: narrative_scale,
        // 4: tone_serious, 5: tone_comic, 6: tone_grim, 7: tone_adventurous, 8: tone_melodramatic, 9: tone_pulpy
        // 10: moral_idealist, 11: moral_cynical, 12: moral_corrupt, 13: moral_redemptive, 14: moral_lawful, 15: moral_chaotic
        // 16: realism_grounded, 17: realism_heightened, 18: realism_fantastical, 19: realism_supernatural, 20: realism_sci_fi
        // 21: agency_type, 22: core_drives, 23: content_flags, 24: genre_affinity, 25: setting_affinity
        
        let parse_float = |index: usize| -> f32 {
            row.get(index)
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0)
        };
        
        let parse_list = |index: usize| -> Vec<String> {
            row.get(index)
                .map(|s| s.split(';').map(|s| s.to_string()).collect())
                .unwrap_or_default()
        };
        
        let parse_flags = |index: usize| -> Vec<String> {
            row.get(index)
                .map(|s| s.split(';').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        };
        
        Ok(Self {
            id: row.get(0).ok_or("Missing id")?.to_string(),
            category: row.get(1)
                .ok_or("Missing category")?
                .parse::<ElementCategory>()
                .map_err(|e| format!("Invalid category: {}", e))?,
            subtype: row.get(2).ok_or("Missing subtype")?.to_string(),
            narrative_scale: row.get(3).ok_or("Missing narrative_scale")?.to_string(),
            tone_profile: ToneProfile {
                serious: parse_float(4),
                comic: parse_float(5),
                grim: parse_float(6),
                adventurous: parse_float(7),
                melodramatic: parse_float(8),
                pulpy: parse_float(9),
            },
            moral_profile: MoralProfile {
                idealist: parse_float(10),
                cynical: parse_float(11),
                corrupt: parse_float(12),
                redemptive: parse_float(13),
                lawful: parse_float(14),
                chaotic: parse_float(15),
            },
            realism_profile: RealismProfile {
                grounded: parse_float(16),
                heightened: parse_float(17),
                fantastical: parse_float(18),
                supernatural: parse_float(19),
                sci_fi: parse_float(20),
            },
            agency_type: row.get(21).ok_or("Missing agency_type")?.to_string(),
            core_drives: parse_list(22),
            content_flags: parse_flags(23),
            genre_affinity: parse_list(24),
            setting_affinity: parse_list(25),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityCategory {
    Strong,
    Good,
    Conditional,
    Weak,
    Incompatible,
}

impl CompatibilityCategory {
    pub fn range(&self) -> (f32, f32) {
        match self {
            CompatibilityCategory::Strong => (0.80, 1.00),
            CompatibilityCategory::Good => (0.60, 0.79),
            CompatibilityCategory::Conditional => (0.40, 0.59),
            CompatibilityCategory::Weak => (0.20, 0.39),
            CompatibilityCategory::Incompatible => (0.00, 0.19),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityResult {
    pub score: f32,
    pub category: CompatibilityCategory,
    pub axis_scores: HashMap<String, f32>,
    pub penalties: Vec<String>,
    pub bonuses: Vec<String>,
    pub explanation: Vec<String>,
}

impl CompatibilityResult {
    pub fn discretize_score(score: f32) -> CompatibilityCategory {
        if score >= 0.80 {
            CompatibilityCategory::Strong
        } else if score >= 0.60 {
            CompatibilityCategory::Good
        } else if score >= 0.40 {
            CompatibilityCategory::Conditional
        } else if score >= 0.20 {
            CompatibilityCategory::Weak
        } else {
            CompatibilityCategory::Incompatible
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityMatrix {
    pub elements: HashMap<String, HollywoodElement>,
    pub rules: HashMap<String, HashMap<String, f32>>,
}

impl Default for CompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatibilityMatrix {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            rules: HashMap::new(),
        }
    }
    
    pub fn load_from_csv(
        &mut self, 
        elements_path: &str, 
        rules_path: &str
    ) -> Result<(), String> {
        let mut rdr = csv::Reader::from_path(elements_path)
            .map_err(|e| format!("Error reading elements.csv: {}", e))?;
        
        for result in rdr.records() {
            let row = result.map_err(|e| format!("CSV error: {}", e))?;
            let elem = HollywoodElement::from_csv_row(&row)?;
            self.elements.insert(elem.id.clone(), elem);
        }
        
        let mut rdr = csv::Reader::from_path(rules_path)
            .map_err(|e| format!("Error reading compatibility_rules.csv: {}", e))?;
        
        for result in rdr.records() {
            let row = result.map_err(|e| format!("CSV error: {}", e))?;
            let pair_type = row.get(0)
                .ok_or("Missing pair_type")?.to_string();
            let axis = row.get(1)
                .ok_or("Missing axis")?.to_string();
            let weight: f32 = row.get(2)
                .ok_or("Missing weight")?.parse()
                .map_err(|e| format!("Invalid weight: {}", e))?;
            
            self.rules.entry(pair_type)
                .or_insert_with(HashMap::new)
                .insert(axis, weight);
        }
        
        Ok(())
    }
    
    pub fn get_pair_type(&self, a: &HollywoodElement, b: &HollywoodElement) -> String {
        let cat_a = format!("{:?}", a.category).to_lowercase();
        let cat_b = format!("{:?}", b.category).to_lowercase();
        let mut types = vec![cat_a, cat_b];
        types.sort();
        format!("{}_{}", types[0], types[1])
    }
    
    pub fn check_compatibility(&self, element1: &str, element2: &str) -> Option<f32> {
        let elem1 = self.elements.get(element1);
        let elem2 = self.elements.get(element2);
        
        if let (Some(e1), Some(e2)) = (elem1, elem2) {
            let pair_type = self.get_pair_type(e1, e2);
            self.rules.get(&pair_type).and_then(|inner_map| inner_map.get(element2).copied())
        } else {
            None
        }
    }
}