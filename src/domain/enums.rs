// Domain enums for Cadiz12 project

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementType {
    Character,
    Location,
    Object,
    Action,
    Dialogue,
    Description,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    Draft,
    InProgress,
    Review,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Pending,
    Valid,
    Invalid,
    NeedsReview,
}

impl std::fmt::Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElementType::Character => write!(f, "character"),
            ElementType::Location => write!(f, "location"),
            ElementType::Object => write!(f, "object"),
            ElementType::Action => write!(f, "action"),
            ElementType::Dialogue => write!(f, "dialogue"),
            ElementType::Description => write!(f, "description"),
        }
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Draft => write!(f, "draft"),
            ProjectStatus::InProgress => write!(f, "in_progress"),
            ProjectStatus::Review => write!(f, "review"),
            ProjectStatus::Completed => write!(f, "completed"),
            ProjectStatus::Archived => write!(f, "archived"),
        }
    }
}