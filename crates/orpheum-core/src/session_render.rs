use camino::Utf8Path;

use crate::catalog::{CheckDef, OutputMode};
use crate::checks::CheckStatusValue;
use crate::error::OrpheumError;
use crate::session_fs::read_session_files;
use crate::session_types::{
    ArtifactStatusValue, SessionManifest, SessionScenarioSnapshot, SessionState,
};

pub fn generate_current_prompt(project_root: &Utf8Path) -> Result<String, OrpheumError> {
    let (_, snapshot, state, files) = read_session_files(project_root)?;
    let prompt = build_prompt(&snapshot, &state, OutputMode::Human);
    std::fs::write(&files.prompt_file, &prompt)?;
    Ok(prompt)
}

pub(crate) fn build_prompt(
    snapshot: &SessionScenarioSnapshot,
    state: &SessionState,
    _mode: OutputMode,
) -> String {
    let expected_artifacts = snapshot
        .artifacts
        .iter()
        .map(|artifact| {
            let status = state
                .artifact_status
                .get(&artifact.id)
                .map(|value| match value {
                    ArtifactStatusValue::Pending => "pending",
                    ArtifactStatusValue::Present => "present",
                    ArtifactStatusValue::Verified => "verified",
                    ArtifactStatusValue::Failed => "failed",
                })
                .unwrap_or("pending");
            format!(
                "- `{}` -> `{}` ({status})",
                artifact.id, artifact.default_output_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let failed_checks = state
        .check_status
        .iter()
        .filter(|(_, status)| matches!(status, CheckStatusValue::Failed))
        .map(|(check, _)| format!("- `{check}`"))
        .collect::<Vec<_>>();
    let semantic_review_guidance = semantic_review_guidance(snapshot.scenario.id.as_str());

    format!(
        "# Current Orpheum Prompt\n\nScenario: `{}`\n\nSummary: {}\n\nCurrent phase: `{}`\n\nPending workflows:\n{}\n\nExpected artifacts:\n{}\n\nBlocking checks:\n{}\n{}\n",
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
        },
        semantic_review_guidance,
    )
}

pub(crate) fn build_active_markdown(
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
            .filter(|status| matches!(status, CheckStatusValue::Failed))
            .count(),
        snapshot.scenario.exit_conditions.join(", "),
    )
}

pub(crate) fn aggregate_check_status<'a>(
    check: &CheckDef,
    artifact_results: impl Iterator<Item = &'a CheckStatusValue>,
) -> CheckStatusValue {
    let mut saw_pass = false;
    for status in artifact_results {
        match status {
            CheckStatusValue::Failed => return CheckStatusValue::Failed,
            CheckStatusValue::Passed => saw_pass = true,
            _ => {}
        }
    }
    if check.applies_to.is_empty() {
        CheckStatusValue::NotEvaluableInV1
    } else if saw_pass {
        CheckStatusValue::Passed
    } else {
        CheckStatusValue::Pending
    }
}

fn semantic_review_guidance(scenario_id: &str) -> String {
    if matches!(
        scenario_id,
        "project-discovery" | "project-planning" | "delivery-slice-planning"
    ) {
        "\nSemantic review checkpoint:\n- required before discovery or planning closeout\n- review artifacts one by one with the human for semantic correctness, package boundaries, unwanted architecture drift, and locked decisions\n- use Planning Mode when available, or the host environment's nearest equivalent\n- stay in Planning Mode until questions are resolved, decision changes are captured durably, and cross-artifact reconciliation is complete".to_string()
    } else {
        String::new()
    }
}
