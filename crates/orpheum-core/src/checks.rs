use std::collections::BTreeMap;
use std::fs;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, CheckDef};
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::session::{aggregate_check_status, read_session_files, refresh_state_files};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatusValue {
    Passed,
    Failed,
    Pending,
    Skipped,
}

impl CheckStatusValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Skipped => "skipped",
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
    pub summary: BTreeMap<String, String>,
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

    let mut grouped: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for result in &results {
        grouped
            .entry(result.check_id.clone())
            .or_default()
            .push(result.status.as_str());
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

    if report.summary.values().any(|status| status == "failed") {
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
            status: CheckStatusValue::Skipped,
            message: "check has no applies_to artifact; skipped by v1 runner".into(),
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
        let status = match check.mode.as_str() {
            "presence" => CheckStatus {
                check_id: check.id.clone(),
                artifact_id: Some(artifact_id.clone()),
                status: CheckStatusValue::Passed,
                message: format!("artifact present: {}", target_path),
            },
            "headings" => {
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
            other => CheckStatus {
                check_id: check.id.clone(),
                artifact_id: Some(artifact_id.clone()),
                status: CheckStatusValue::Skipped,
                message: format!("unsupported check mode {other}; skipped"),
            },
        };
        results.push(status);
    }

    Ok(results)
}
