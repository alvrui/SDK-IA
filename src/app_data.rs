use std::sync::Arc;
use crate::services::persistence::PersistenceService;
use crate::services::narrative::NarrativeService;
use crate::domain::hollywood_animal::CompatibilityMatrix;

/// Application state containing all services
pub struct AppData {
    pub persistence: Arc<PersistenceService>,
    pub narrative_service: Arc<NarrativeService>,
    pub compatibility_matrix: Arc<CompatibilityMatrix>,
}
