use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

use crate::catalog::{Catalog, CheckDef, OutputMode};
use crate::error::{OrpheumError, OrpheumErrorCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManifest {
    pub session_id: String,
    pub scenario_id: String,
    pub scenario_version: u32,
    pub applied_at: String,
    pub orpheum_source: Utf8PathBuf,
    pub source_revision: Option<String>,
    pub target_project_root: Utf8PathBuf,
    pub mode: String,
    pub cleanup_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub state: String,
    pub current_phase: String,
    pub completed_workflows: Vec<String>,
    pub pending_workflows: Vec<String>,
    pub artifact_status: BTreeMap<String, String>,
    pub check_status: BTreeMap<String, String>,
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
    pub cleanup_policy: String,
}

pub fn session_files(project_root: &Utf8Path) -> SessionFiles {
    let control_dir = project_root.join(".orpheum");
    SessionFiles {
        active_file: control_dir.join("ACTIVE.md"),
        session_file: control_dir.join("session.json"),
        scenario_file: control_dir.join("scenario.json"),
        state_file: control_dir.join("state.json"),
        prompt_file: control_dir.join("prompts").join("current.md"),
        check_log_file: control_dir.join("logs").join("checks.json"),
        control_dir,
    }
}

pub fn apply_scenario(
    catalog: &Catalog,
    project_root: &Utf8Path,
    scenario_id: &str,
    force: bool,
) -> Result<SessionApplyResult, OrpheumError> {
    let files = session_files(project_root);
    if files.session_file.exists() && !force {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::SessionAlreadyActive,
            format!(
                "an active Orpheum session already exists for this project: {}",
                files.session_file
            ),
        ));
    }

    let resolved = catalog.resolve_scenario(scenario_id)?;

    fs::create_dir_all(files.prompt_file.parent().expect("prompt dir"))?;
    fs::create_dir_all(files.check_log_file.parent().expect("log dir"))?;

    let session_id = format!(
        "sess_{}_{}",
        OffsetDateTime::now_utc()
            .format(&time::macros::format_description!(
                "[year][month][day]_[hour][minute][second]"
            ))
            .unwrap_or_else(|_| "unknown".into()),
        Uuid::new_v4().simple()
    );
    let applied_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|err| OrpheumError::coded(OrpheumErrorCode::Io, err.to_string()))?;
    let current_phase = resolved
        .workflows
        .first()
        .map(|workflow| workflow.id.clone())
        .unwrap_or_else(|| "apply".to_string());

    let manifest = SessionManifest {
        session_id: session_id.clone(),
        scenario_id: resolved.scenario.id.clone(),
        scenario_version: resolved.scenario.version,
        applied_at,
        orpheum_source: catalog.paths.root.clone(),
        source_revision: git_revision(&catalog.paths.root),
        target_project_root: project_root.to_path_buf(),
        mode: "default".into(),
        cleanup_policy: "explicit".into(),
    };
    let state = SessionState {
        state: "active".into(),
        current_phase: current_phase.clone(),
        completed_workflows: Vec::new(),
        pending_workflows: resolved
            .workflows
            .iter()
            .map(|workflow| workflow.id.clone())
            .collect(),
        artifact_status: resolved
            .artifacts
            .iter()
            .map(|artifact| (artifact.id.clone(), "pending".into()))
            .collect(),
        check_status: resolved
            .checks
            .iter()
            .map(|check| (check.id.clone(), "pending".into()))
            .collect(),
        suspended: false,
        resumable: true,
    };
    let snapshot = SessionScenarioSnapshot {
        scenario: resolved.scenario.clone(),
        roles: resolved.roles.clone(),
        workflows: resolved.workflows.clone(),
        artifacts: resolved.artifacts.clone(),
        checks: resolved.checks.clone(),
    };

    fs::write(
        &files.session_file,
        serde_json::to_string_pretty(&manifest)?,
    )?;
    fs::write(
        &files.scenario_file,
        serde_json::to_string_pretty(&snapshot)?,
    )?;
    fs::write(&files.state_file, serde_json::to_string_pretty(&state)?)?;
    fs::write(
        &files.check_log_file,
        "{\n  \"results\": [],\n  \"summary\": {}\n}\n",
    )?;

    let prompt = build_prompt(&snapshot, &state, OutputMode::Human);
    fs::write(&files.prompt_file, &prompt)?;
    fs::write(
        &files.active_file,
        build_active_markdown(&manifest, &snapshot, &state),
    )?;

    Ok(SessionApplyResult {
        session_id,
        scenario_id: resolved.scenario.id,
        project_root: project_root.to_path_buf(),
        control_dir: files.control_dir,
        active_file: files.active_file,
        current_phase,
        next_command: "orpheum prompt current".into(),
        cleanup_policy: "explicit".into(),
    })
}

pub fn read_session_files(
    project_root: &Utf8Path,
) -> Result<
    (
        SessionManifest,
        SessionScenarioSnapshot,
        SessionState,
        SessionFiles,
    ),
    OrpheumError,
> {
    let files = session_files(project_root);
    if !files.session_file.exists() {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::NoActiveSession,
            format!("no active Orpheum session found in {}", files.control_dir),
        ));
    }

    let manifest =
        serde_json::from_str::<SessionManifest>(&fs::read_to_string(&files.session_file)?)?;
    let snapshot = serde_json::from_str::<SessionScenarioSnapshot>(&fs::read_to_string(
        &files.scenario_file,
    )?)?;
    let state = serde_json::from_str::<SessionState>(&fs::read_to_string(&files.state_file)?)?;
    Ok((manifest, snapshot, state, files))
}

pub fn generate_current_prompt(project_root: &Utf8Path) -> Result<String, OrpheumError> {
    let (_, snapshot, state, files) = read_session_files(project_root)?;
    let prompt = build_prompt(&snapshot, &state, OutputMode::Human);
    fs::write(&files.prompt_file, &prompt)?;
    Ok(prompt)
}

pub fn read_active_summary(project_root: &Utf8Path) -> Result<String, OrpheumError> {
    let (_, _, _, files) = read_session_files(project_root)?;
    Ok(fs::read_to_string(files.active_file)?)
}

pub fn refresh_state_files(
    project_root: &Utf8Path,
    snapshot: &SessionScenarioSnapshot,
    state: &SessionState,
    manifest: &SessionManifest,
) -> Result<(), OrpheumError> {
    let files = session_files(project_root);
    fs::write(&files.state_file, serde_json::to_string_pretty(state)?)?;
    let prompt = build_prompt(snapshot, state, OutputMode::Human);
    fs::write(&files.prompt_file, &prompt)?;
    fs::write(
        &files.active_file,
        build_active_markdown(manifest, snapshot, state),
    )?;
    Ok(())
}

pub fn build_prompt(
    snapshot: &SessionScenarioSnapshot,
    state: &SessionState,
    _mode: OutputMode,
) -> String {
    let expected_artifacts = snapshot
        .artifacts
        .iter()
        .map(|artifact| format!("- `{}` -> `{}`", artifact.id, artifact.default_output_path))
        .collect::<Vec<_>>()
        .join("\n");
    let failed_checks = state
        .check_status
        .iter()
        .filter(|(_, status)| *status == "failed")
        .map(|(check, _)| format!("- `{check}`"))
        .collect::<Vec<_>>();

    format!(
        "# Current Orpheum Prompt\n\nScenario: `{}`\n\nSummary: {}\n\nCurrent phase: `{}`\n\nPending workflows:\n{}\n\nExpected artifacts:\n{}\n\nBlocking checks:\n{}\n",
        snapshot.scenario.id,
        snapshot.scenario.summary,
        state.current_phase,
        state
            .pending_workflows
            .iter()
            .map(|workflow| format!("- `{workflow}`"))
            .collect::<Vec<_>>()
            .join("\n"),
        expected_artifacts,
        if failed_checks.is_empty() {
            "- none".to_string()
        } else {
            failed_checks.join("\n")
        }
    )
}

fn build_active_markdown(
    manifest: &SessionManifest,
    snapshot: &SessionScenarioSnapshot,
    state: &SessionState,
) -> String {
    format!(
        "# ACTIVE\n\n- Scenario: `{}`\n- Session ID: `{}`\n- Current phase: `{}`\n- Pending workflows: {}\n- Expected outputs: {}\n- Blocking checks: {}\n- Completion criteria: {}\n- Next recommended action: `orpheum prompt current`\n",
        snapshot.scenario.title,
        manifest.session_id,
        state.current_phase,
        state.pending_workflows.len(),
        snapshot.artifacts.len(),
        state
            .check_status
            .values()
            .filter(|status| status.as_str() == "failed")
            .count(),
        snapshot.scenario.exit_conditions.join(", "),
    )
}

fn git_revision(catalog_root: &Utf8Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(catalog_root.as_str())
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}

pub(crate) fn aggregate_check_status<'a>(
    check: &CheckDef,
    artifact_results: impl Iterator<Item = &'a str>,
) -> String {
    let mut saw_pass = false;
    for status in artifact_results {
        match status {
            "failed" => return "failed".into(),
            "passed" => saw_pass = true,
            _ => {}
        }
    }
    if check.applies_to.is_empty() {
        "skipped".into()
    } else if saw_pass {
        "passed".into()
    } else {
        "pending".into()
    }
}
