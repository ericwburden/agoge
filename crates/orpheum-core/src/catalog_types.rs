use std::collections::{BTreeMap, BTreeSet};

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDef {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub version: u32,
    pub summary: String,
    pub roles: Vec<String>,
    pub workflows: Vec<String>,
    pub artifacts: Vec<String>,
    pub checks: Vec<String>,
    pub entry_conditions: Vec<String>,
    pub exit_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDef {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub version: u32,
    pub summary: String,
    pub role: Option<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub skills: Vec<String>,
    pub checks: Vec<String>,
    pub handoff_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDef {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub version: u32,
    pub summary: String,
    pub default_workflows: Vec<String>,
    pub skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactDef {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub version: u32,
    pub summary: String,
    pub template: bool,
    pub default_output_path: String,
    pub checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckMode {
    Presence,
    Headings,
    #[serde(other)]
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckSeverity {
    Error,
    Warning,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckDef {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub version: u32,
    pub summary: String,
    pub mode: CheckMode,
    pub severity: CheckSeverity,
    pub applies_to: Vec<String>,
    #[serde(default)]
    pub required_headings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEntry {
    pub metadata: ScenarioDef,
    pub path: Utf8PathBuf,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEntry {
    pub metadata: WorkflowDef,
    pub path: Utf8PathBuf,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleEntry {
    pub metadata: RoleDef,
    pub path: Utf8PathBuf,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactEntry {
    pub metadata: ArtifactDef,
    pub path: Utf8PathBuf,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckEntry {
    pub metadata: CheckDef,
    pub path: Utf8PathBuf,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogPaths {
    pub root: Utf8PathBuf,
    pub scenarios: Utf8PathBuf,
    pub workflows: Utf8PathBuf,
    pub roles: Utf8PathBuf,
    pub artifacts: Utf8PathBuf,
    pub checks: Utf8PathBuf,
    pub skills: Utf8PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub paths: CatalogPaths,
    pub scenarios: BTreeMap<String, ScenarioEntry>,
    pub workflows: BTreeMap<String, WorkflowEntry>,
    pub roles: BTreeMap<String, RoleEntry>,
    pub artifacts: BTreeMap<String, ArtifactEntry>,
    pub checks: BTreeMap<String, CheckEntry>,
    pub skills: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub id: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioListItem {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedScenario {
    pub scenario: ScenarioDef,
    pub roles: Vec<RoleDef>,
    pub workflows: Vec<WorkflowDef>,
    pub artifacts: Vec<ArtifactDef>,
    pub checks: Vec<CheckDef>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputMode {
    Human,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCounts {
    pub scenarios: usize,
    pub workflows: usize,
    pub roles: usize,
    pub artifacts: usize,
    pub checks: usize,
    pub skills: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorWarning {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorReport {
    pub catalog_root: Utf8PathBuf,
    pub project_root: Utf8PathBuf,
    pub counts: HealthCounts,
    pub active_session_present: bool,
    pub orpheum_gitignored: bool,
    pub warnings: Vec<DoctorWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogHealth {
    pub counts: HealthCounts,
    pub warnings: Vec<DoctorWarning>,
}
