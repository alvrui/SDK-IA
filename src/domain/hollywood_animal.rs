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
        let category = row.get("category")
            .ok_or("Missing category")?
            .parse::<ElementCategory>()
            .map_err(|e| format!("Invalid category: {}", e))?;
        
        let parse_float = |field: &str| -> f32 {
            row.get(field)
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0)
        };
        
        let parse_list = |field: &str| -> Vec<String> {
            row.get(field)
                .map(|s| s.split(';').map(|s| s.to_string()).collect())
                .unwrap_or_default()
        };
        
        let parse_flags = |field: &str| -> Vec<String> {
            row.get(field)
                .map(|s| s.split(';').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect())
                .unwrap_or_default()
        };
        
        Ok(Self {
            id: row.get("id").ok_or("Missing id")?.to_string(),
            category,
            subtype: row.get("subtype").ok_or("Missing subtype")?.to_string(),
            narrative_scale: row.get("narrative_scale").ok_or("Missing narrative_scale")?.to_string(),
            tone_profile: ToneProfile {
                serious: parse_float("tone_serious"),
                comic: parse_float("tone_comic"),
                grim: parse_float("tone_grim"),
                adventurous: parse_float("tone_adventurous"),
                melodramatic: parse_float("tone_melodramatic"),
                pulpy: parse_float("tone_pulpy"),
            },
            moral_profile: MoralProfile {
                idealist: parse_float("moral_idealist"),
                cynical: parse_float("moral_cynical"),
                corrupt: parse_float("moral_corrupt"),
                redemptive: parse_float("moral_redemptive"),
                lawful: parse_float("moral_lawful"),
                chaotic: parse_float("moral_chaotic"),
            },
            realism_profile: RealismProfile {
                grounded: parse_float("realism_grounded"),
                heightened: parse_float("realism_heightened"),
                fantastical: parse_float("realism_fantastical"),
                supernatural: parse_float("realism_supernatural"),
                sci_fi: parse_float("realism_sci_fi"),
            },
            agency_type: row.get("agency_type").ok_or("Missing agency_type")?.to_string(),
            core_drives: parse_list("core_drives"),
            content_flags: parse_flags("content_flags"),
            genre_affinity: parse_list("genre_affinity"),
            setting_affinity: parse_list("setting_affinity"),
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
            let pair_type = row.get("pair_type")
                .ok_or("Missing pair_type")?.to_string();
            let axis = row.get("axis")
                .ok_or("Missing axis")?.to_string();
            let weight: f32 = row.get("weight")
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
}