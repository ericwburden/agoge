pub mod catalog;
pub mod checks;
pub mod doctor;
pub mod error;
pub mod frontmatter;
pub mod init;
pub mod session;

pub use catalog::{
    ArtifactDef, ArtifactEntry, Catalog, CatalogHealth, CatalogPaths, CheckDef, CheckEntry,
    DoctorReport, DoctorWarning, EntityRef, HealthCounts, OutputMode, ResolvedScenario, RoleDef,
    RoleEntry, ScenarioDef, ScenarioEntry, ScenarioListItem, WorkflowDef, WorkflowEntry,
};
pub use checks::run_checks;
pub use checks::{CheckRunReport, CheckStatus, CheckStatusValue};
pub use doctor::run_doctor;
pub use error::{OrpheumError, OrpheumErrorCode};
pub use init::{InitResult, init_project};
pub use session::{
    SessionApplyResult, SessionFiles, SessionManifest, SessionState, apply_scenario,
    generate_current_prompt, read_active_summary, read_session_files,
};
