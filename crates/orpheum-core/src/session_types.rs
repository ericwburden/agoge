use std::collections::BTreeMap;

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use crate::checks::CheckStatusValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionMode {
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupPolicy {
    Explicit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleState {
    Active,
    Suspended,
    Finalized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStatusValue {
    Pending,
    Present,
    Verified,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManifest {
    pub session_id: String,
    pub scenario_id: String,
    pub scenario_version: u32,
    pub applied_at: String,
    pub orpheum_source: Utf8PathBuf,
    pub source_revision: Option<String>,
    pub target_project_root: Utf8PathBuf,
    pub mode: SessionMode,
    pub cleanup_policy: CleanupPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub state: SessionLifecycleState,
    pub current_phase: String,
    pub completed_workflows: Vec<String>,
    pub pending_workflows: Vec<String>,
    pub artifact_status: BTreeMap<String, ArtifactStatusValue>,
    pub check_status: BTreeMap<String, CheckStatusValue>,
    pub suspended: bool,
    pub resumable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionScenarioSnapshot {
    pub scenario: crate::catalog::ScenarioDef,
    pub roles: Vec<crate::catalog::RoleDef>,
    pub workflows: Vec<crate::catalog::WorkflowDef>,
    pub artifacts: Vec<crate::catalog::ArtifactDef>,
    pub checks: Vec<crate::catalog::CheckDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionFiles {
    pub control_dir: Utf8PathBuf,
    pub active_file: Utf8PathBuf,
    pub session_file: Utf8PathBuf,
    pub scenario_file: Utf8PathBuf,
    pub state_file: Utf8PathBuf,
    pub prompt_file: Utf8PathBuf,
    pub check_log_file: Utf8PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionApplyResult {
    pub session_id: String,
    pub scenario_id: String,
    pub project_root: Utf8PathBuf,
    pub control_dir: Utf8PathBuf,
    pub active_file: Utf8PathBuf,
    pub current_phase: String,
    pub next_command: String,
    pub cleanup_policy: CleanupPolicy,
}
