use std::fs;

use camino::Utf8Path;

use crate::catalog::{
    Catalog, CatalogHealth, DoctorWarning, HealthCounts, ResolvedScenario, ScenarioListItem,
};
use crate::error::{OrpheumError, OrpheumErrorCode};

impl Catalog {
    pub fn health(&self, project_root: &Utf8Path) -> CatalogHealth {
        let mut warnings = Vec::new();
        let gitignore = project_root.join(".gitignore");
        if !gitignore.exists() {
            warnings.push(DoctorWarning {
                code: "GITIGNORE_MISSING".into(),
                message:
                    "project root has no .gitignore; .orpheum/ would not be ignored by default"
                        .into(),
            });
        } else if let Ok(contents) = fs::read_to_string(&gitignore) {
            if !contents.lines().any(|line| line.trim() == ".orpheum/") {
                warnings.push(DoctorWarning {
                    code: "ORPHEUM_NOT_IGNORED".into(),
                    message: ".gitignore exists but does not contain a .orpheum/ entry".into(),
                });
            }
        }

        CatalogHealth {
            counts: HealthCounts {
                scenarios: self.scenarios.len(),
                workflows: self.workflows.len(),
                roles: self.roles.len(),
                artifacts: self.artifacts.len(),
                checks: self.checks.len(),
                skills: self.skills.len(),
            },
            warnings,
        }
    }

    pub fn list_scenarios(&self) -> Vec<ScenarioListItem> {
        self.scenarios
            .values()
            .map(|entry| ScenarioListItem {
                id: entry.metadata.id.clone(),
                title: entry.metadata.title.clone(),
                summary: entry.metadata.summary.clone(),
                version: entry.metadata.version,
            })
            .collect()
    }

    pub fn resolve_scenario(&self, scenario_id: &str) -> Result<ResolvedScenario, OrpheumError> {
        let scenario = self.scenarios.get(scenario_id).ok_or_else(|| {
            OrpheumError::coded(
                OrpheumErrorCode::ScenarioNotFound,
                format!("scenario not found: {scenario_id}"),
            )
        })?;

        let roles = scenario
            .metadata
            .roles
            .iter()
            .map(|id| self.roles.get(id).map(|entry| entry.metadata.clone()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                OrpheumError::coded(
                    OrpheumErrorCode::InvalidMetadata,
                    format!("scenario {} references missing role", scenario_id),
                )
            })?;

        let workflows = scenario
            .metadata
            .workflows
            .iter()
            .map(|id| self.workflows.get(id).map(|entry| entry.metadata.clone()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                OrpheumError::coded(
                    OrpheumErrorCode::InvalidMetadata,
                    format!("scenario {} references missing workflow", scenario_id),
                )
            })?;

        let artifacts = scenario
            .metadata
            .artifacts
            .iter()
            .map(|id| self.artifacts.get(id).map(|entry| entry.metadata.clone()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                OrpheumError::coded(
                    OrpheumErrorCode::InvalidMetadata,
                    format!("scenario {} references missing artifact", scenario_id),
                )
            })?;

        let checks = scenario
            .metadata
            .checks
            .iter()
            .map(|id| self.checks.get(id).map(|entry| entry.metadata.clone()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                OrpheumError::coded(
                    OrpheumErrorCode::InvalidMetadata,
                    format!("scenario {} references missing check", scenario_id),
                )
            })?;

        Ok(ResolvedScenario {
            scenario: scenario.metadata.clone(),
            roles,
            workflows,
            artifacts,
            checks,
        })
    }

    pub(crate) fn validate(&self) -> Result<(), OrpheumError> {
        for scenario in self.scenarios.values() {
            for role_id in &scenario.metadata.roles {
                if !self.roles.contains_key(role_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "scenario {} references unknown role {role_id}",
                            scenario.metadata.id
                        ),
                    ));
                }
            }
            for workflow_id in &scenario.metadata.workflows {
                if !self.workflows.contains_key(workflow_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "scenario {} references unknown workflow {workflow_id}",
                            scenario.metadata.id
                        ),
                    ));
                }
            }
            for artifact_id in &scenario.metadata.artifacts {
                if !self.artifacts.contains_key(artifact_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "scenario {} references unknown artifact {artifact_id}",
                            scenario.metadata.id
                        ),
                    ));
                }
            }
            for check_id in &scenario.metadata.checks {
                if !self.checks.contains_key(check_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "scenario {} references unknown check {check_id}",
                            scenario.metadata.id
                        ),
                    ));
                }
            }
        }

        for workflow in self.workflows.values() {
            if let Some(role_id) = &workflow.metadata.role {
                if !self.roles.contains_key(role_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "workflow {} references unknown role {role_id}",
                            workflow.metadata.id
                        ),
                    ));
                }
            }
            for artifact_id in workflow
                .metadata
                .inputs
                .iter()
                .chain(workflow.metadata.outputs.iter())
            {
                if !self.artifacts.contains_key(artifact_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "workflow {} references unknown artifact {artifact_id}",
                            workflow.metadata.id
                        ),
                    ));
                }
            }
            for skill_id in &workflow.metadata.skills {
                if !self.skills.contains(skill_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "workflow {} references unknown skill {skill_id}",
                            workflow.metadata.id
                        ),
                    ));
                }
            }
            for check_id in &workflow.metadata.checks {
                if !self.checks.contains_key(check_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "workflow {} references unknown check {check_id}",
                            workflow.metadata.id
                        ),
                    ));
                }
            }
        }

        for role in self.roles.values() {
            for workflow_id in &role.metadata.default_workflows {
                if !self.workflows.contains_key(workflow_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "role {} references unknown workflow {workflow_id}",
                            role.metadata.id
                        ),
                    ));
                }
            }
            for skill_id in &role.metadata.skills {
                if !self.skills.contains(skill_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "role {} references unknown skill {skill_id}",
                            role.metadata.id
                        ),
                    ));
                }
            }
        }

        for artifact in self.artifacts.values() {
            for check_id in &artifact.metadata.checks {
                if !self.checks.contains_key(check_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "artifact {} references unknown check {check_id}",
                            artifact.metadata.id
                        ),
                    ));
                }
            }
        }

        for check in self.checks.values() {
            for artifact_id in &check.metadata.applies_to {
                if !self.artifacts.contains_key(artifact_id) {
                    return Err(OrpheumError::coded(
                        OrpheumErrorCode::InvalidMetadata,
                        format!(
                            "check {} references unknown artifact {artifact_id}",
                            check.metadata.id
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}
