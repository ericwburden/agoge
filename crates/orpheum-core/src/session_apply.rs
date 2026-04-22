use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

use camino::Utf8Path;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

use crate::catalog::{Catalog, OutputMode};
use crate::checks::{CheckRunReport, CheckStatusValue};
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::session_fs::session_files;
use crate::session_render::{build_active_markdown, build_prompt};
use crate::session_types::{
    CleanupPolicy, SessionApplyResult, SessionLifecycleState, SessionManifest, SessionMode,
    SessionScenarioSnapshot, SessionState,
};

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
        mode: SessionMode::Default,
        cleanup_policy: CleanupPolicy::Explicit,
    };
    let state = SessionState {
        state: SessionLifecycleState::Active,
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
            .map(|check| (check.id.clone(), CheckStatusValue::Pending))
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
    let check_report = CheckRunReport {
        scenario_id: resolved.scenario.id.clone(),
        results: Vec::new(),
        summary: BTreeMap::new(),
    };
    fs::write(
        &files.check_log_file,
        serde_json::to_string_pretty(&check_report)?,
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
        cleanup_policy: CleanupPolicy::Explicit,
    })
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
