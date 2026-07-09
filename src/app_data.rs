use std::sync::Arc;
use crate::services::persistence::PersistenceService;
use crate::services::narrative::NarrativeService;
use crate::services::validation::DomainValidationService;
use crate::services::versioning::VersioningService;
use crate::services::python_client::PythonClient;
use crate::domain::hollywood_animal::CompatibilityMatrix;

/// Application state containing all services
pub struct AppData {
    pub persistence: Arc<PersistenceService>,
    pub narrative_service: Arc<NarrativeService>,
    pub validation_service: Arc<DomainValidationService>,
    pub versioning_service: Arc<VersioningService>,
    pub compatibility_matrix: Arc<CompatibilityMatrix>,
    pub python_client: Arc<PythonClient>,
}