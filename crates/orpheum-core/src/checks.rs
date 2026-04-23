use std::collections::BTreeMap;
use std::fs;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, CheckDef, CheckMode};
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::session::{aggregate_check_status, read_session_files, refresh_state_files};
use crate::session_types::ArtifactStatusValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatusValue {
    Passed,
    Failed,
    Pending,
    NotEvaluableInV1,
}

impl CheckStatusValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::NotEvaluableInV1 => "not_evaluable_in_v1",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckStatus {
    pub check_id: String,
    pub artifact_id: Option<String>,
    pub status: CheckStatusValue,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRunReport {
    pub scenario_id: String,
    pub results: Vec<CheckStatus>,
    pub summary: BTreeMap<String, CheckStatusValue>,
}

pub fn run_checks(
    catalog: &Catalog,
    project_root: &Utf8Path,
) -> Result<CheckRunReport, OrpheumError> {
    let (manifest, snapshot, mut state, files) = read_session_files(project_root)?;
    let mut results = Vec::new();

    for check_id in &snapshot.scenario.checks {
        let Some(check) = catalog
            .checks
            .get(check_id)
            .map(|entry| entry.metadata.clone())
        else {
            continue;
        };
        results.extend(run_check(project_root, catalog, &check)?);
    }

    for check in &snapshot.checks {
        if snapshot.scenario.checks.contains(&check.id) {
            continue;
        }
        results.extend(run_check(project_root, catalog, check)?);
    }

    let mut grouped: BTreeMap<String, Vec<&CheckStatusValue>> = BTreeMap::new();
    for result in &results {
        grouped
            .entry(result.check_id.clone())
            .or_default()
            .push(&result.status);
    }

    let summary = snapshot
        .checks
        .iter()
        .map(|check| {
            let aggregated = aggregate_check_status(
                check,
                grouped.get(&check.id).into_iter().flatten().copied(),
            );
            (check.id.clone(), aggregated)
        })
        .collect::<BTreeMap<_, _>>();

    state.check_status = summary.clone();
    state.artifact_status = snapshot
        .artifacts
        .iter()
        .map(|artifact| {
            let target_path = project_root.join(&artifact.default_output_path);
            let directly_applicable = results
                .iter()
                .filter(|result| result.artifact_id.as_deref() == Some(artifact.id.as_str()))
                .collect::<Vec<_>>();
            let status = if !target_path.exists() {
                ArtifactStatusValue::Pending
            } else if directly_applicable
                .iter()
                .any(|result| matches!(result.status, CheckStatusValue::Failed))
            {
                ArtifactStatusValue::Failed
            } else if !directly_applicable.is_empty()
                && directly_applicable
                    .iter()
                    .any(|result| matches!(result.status, CheckStatusValue::Passed))
            {
                ArtifactStatusValue::Verified
            } else {
                ArtifactStatusValue::Present
            };
            (artifact.id.clone(), status)
        })
        .collect();
    refresh_state_files(project_root, &snapshot, &state, &manifest)?;

    let report = CheckRunReport {
        scenario_id: snapshot.scenario.id,
        results,
        summary,
    };
    fs::write(
        &files.check_log_file,
        serde_json::to_string_pretty(&report)?,
    )?;

    if report
        .summary
        .values()
        .any(|status| matches!(status, CheckStatusValue::Failed))
    {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::CheckFailed,
            "one or more checks failed",
        ));
    }

    Ok(report)
}

fn run_check(
    project_root: &Utf8Path,
    catalog: &Catalog,
    check: &CheckDef,
) -> Result<Vec<CheckStatus>, OrpheumError> {
    if check.applies_to.is_empty() {
        return Ok(vec![CheckStatus {
            check_id: check.id.clone(),
            artifact_id: None,
            status: CheckStatusValue::NotEvaluableInV1,
            message: "check has no applies_to artifact; not evaluable in v1".into(),
        }]);
    }

    let mut results = Vec::new();
    for artifact_id in &check.applies_to {
        let artifact = catalog.artifacts.get(artifact_id).ok_or_else(|| {
            OrpheumError::coded(
                OrpheumErrorCode::InvalidMetadata,
                format!(
                    "check {} references unknown artifact {artifact_id}",
                    check.id
                ),
            )
        })?;
        let target_path = project_root.join(&artifact.metadata.default_output_path);
        if !target_path.exists() {
            results.push(CheckStatus {
                check_id: check.id.clone(),
                artifact_id: Some(artifact_id.clone()),
                status: CheckStatusValue::Failed,
                message: format!("artifact file not found: {}", target_path),
            });
            continue;
        }

        let content = fs::read_to_string(&target_path)?;
        let status = match check.mode {
            CheckMode::Presence => CheckStatus {
                check_id: check.id.clone(),
                artifact_id: Some(artifact_id.clone()),
                status: CheckStatusValue::Passed,
                message: format!("artifact present: {}", target_path),
            },
            CheckMode::Headings => {
                let missing = check
                    .required_headings
                    .iter()
                    .filter(|heading| !content.contains(&format!("## {heading}")))
                    .cloned()
                    .collect::<Vec<_>>();
                if missing.is_empty() {
                    CheckStatus {
                        check_id: check.id.clone(),
                        artifact_id: Some(artifact_id.clone()),
                        status: CheckStatusValue::Passed,
                        message: format!("required headings present in {}", target_path),
                    }
                } else {
                    CheckStatus {
                        check_id: check.id.clone(),
                        artifact_id: Some(artifact_id.clone()),
                        status: CheckStatusValue::Failed,
                        message: format!(
                            "missing headings in {}: {}",
                            target_path,
                            missing.join(", ")
                        ),
                    }
                }
            }
            CheckMode::Unsupported => CheckStatus {
                check_id: check.id.clone(),
                artifact_id: Some(artifact_id.clone()),
                status: CheckStatusValue::NotEvaluableInV1,
                message: "unsupported check mode; not evaluable in v1".into(),
            },
        };
        results.push(status);
    }

    Ok(results)
}
