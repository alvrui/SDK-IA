use std::str::FromStr;
use crate::domain::{Project, Narrative, StoryElement, GameEvent};

/// Change type for determining version bump
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionChangeType {
    /// Minor changes: metadata, tags, description
    Patch,
    /// Significant changes: adding/removing elements, changing status
    Minor,
    /// Structural changes: major reorganization (manual only)
    Major,
}

impl FromStr for VersionChangeType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "patch" => Ok(VersionChangeType::Patch),
            "minor" => Ok(VersionChangeType::Minor),
            "major" => Ok(VersionChangeType::Major),
            _ => Err(format!("Unknown version change type: {}", s)),
        }
    }
}

/// Versioning service for automatic version management
pub struct VersioningService;

impl VersioningService {
    /// Determine version change type based on what fields are being updated
    pub fn determine_project_change_type(
        original: &Project,
        updated: &Project,
    ) -> VersionChangeType {
        // Check for structural changes (major)
        // For now, we don't auto-detect major changes
        
        // Check for significant changes (minor)
        if original.status != updated.status {
            return VersionChangeType::Minor;
        }
        if original.author != updated.author {
            return VersionChangeType::Minor;
        }
        
        // Check for minor changes (patch)
        if original.name != updated.name {
            return VersionChangeType::Minor; // Name change is significant
        }
        if original.description != updated.description {
            return VersionChangeType::Patch;
        }
        if original.tags != updated.tags {
            return VersionChangeType::Patch;
        }
        if original.metadata != updated.metadata {
            return VersionChangeType::Patch;
        }
        
        // Default to patch if we can't determine
        VersionChangeType::Patch
    }

    /// Determine version change type based on narrative updates
    pub fn determine_narrative_change_type(
        original: &Narrative,
        updated: &Narrative,
    ) -> VersionChangeType {
        // Check for significant changes (minor)
        if original.status != updated.status {
            return VersionChangeType::Minor;
        }
        if original.theme_ids != updated.theme_ids {
            return VersionChangeType::Minor;
        }
        
        // Check for minor changes (patch)
        if original.title != updated.title {
            return VersionChangeType::Minor; // Title change is significant
        }
        if original.synopsis != updated.synopsis {
            return VersionChangeType::Patch;
        }
        if original.metadata != updated.metadata {
            return VersionChangeType::Patch;
        }
        
        // Default to patch
        VersionChangeType::Patch
    }

    /// Determine version change type for story element creation
    pub fn determine_story_element_change_type() -> VersionChangeType {
        // Adding a story element is a significant change
        VersionChangeType::Minor
    }

    /// Determine version change type for story element deletion
    pub fn determine_story_element_deletion_change_type() -> VersionChangeType {
        // Removing a story element is a significant change
        VersionChangeType::Minor
    }

    /// Determine version change type for game event creation
    pub fn determine_game_event_change_type() -> VersionChangeType {
        // Adding a game event is a minor change
        VersionChangeType::Minor
    }

    /// Determine version change type for game event deletion
    pub fn determine_game_event_deletion_change_type() -> VersionChangeType {
        // Removing a game event is a minor change
        VersionChangeType::Minor
    }

    /// Determine version change type for narrative creation
    pub fn determine_narrative_creation_change_type() -> VersionChangeType {
        // Adding a narrative is a significant change
        VersionChangeType::Minor
    }

    /// Determine version change type for narrative deletion
    pub fn determine_narrative_deletion_change_type() -> VersionChangeType {
        // Removing a narrative is a significant change
        VersionChangeType::Minor
    }

    /// Apply version bump to a project
    pub fn apply_project_version_bump(project: &mut Project, change_type: VersionChangeType) {
        match change_type {
            VersionChangeType::Major => project.update_version("major"),
            VersionChangeType::Minor => project.update_version("minor"),
            VersionChangeType::Patch => project.update_version("patch"),
        }
    }

    /// Apply version bump to a narrative
    pub fn apply_narrative_version_bump(narrative: &mut Narrative, change_type: VersionChangeType) {
        match change_type {
            VersionChangeType::Major => narrative.update_version("major"),
            VersionChangeType::Minor => narrative.update_version("minor"),
            VersionChangeType::Patch => narrative.update_version("patch"),
        }
    }

    /// Parse a version string into components
    pub fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() == 3 {
            match (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<u32>()
            ) {
                (Ok(major), Ok(minor), Ok(patch)) => Some((major, minor, patch)),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Format version from components
    pub fn format_version(major: u32, minor: u32, patch: u32) -> String {
        format!("{}.{}.{}", major, minor, patch)
    }

    /// Increment version based on change type
    pub fn increment_version(version: &str, change_type: VersionChangeType) -> String {
        if let Some((major, minor, patch)) = Self::parse_version(version) {
            match change_type {
                VersionChangeType::Major => Self::format_version(major + 1, 0, 0),
                VersionChangeType::Minor => Self::format_version(major, minor + 1, 0),
                VersionChangeType::Patch => Self::format_version(major, minor, patch + 1),
            }
        } else {
            // Default to 1.0.0 if version is invalid
            match change_type {
                VersionChangeType::Major => "2.0.0".to_string(),
                VersionChangeType::Minor => "1.1.0".to_string(),
                VersionChangeType::Patch => "1.0.1".to_string(),
            }
        }
    }
}
