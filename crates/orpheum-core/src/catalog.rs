use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::frontmatter::parse_frontmatter;

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

impl CatalogPaths {
    pub fn discover(root: impl AsRef<Utf8Path>) -> Result<Self, OrpheumError> {
        let root = root.as_ref();
        if !root.exists() {
            return Err(OrpheumError::coded(
                OrpheumErrorCode::CatalogNotFound,
                format!("catalog root does not exist: {root}"),
            ));
        }

        let paths = Self {
            root: root.to_path_buf(),
            scenarios: root.join("scenarios"),
            workflows: root.join("workflows"),
            roles: root.join("roles"),
            artifacts: root.join("artifacts"),
            checks: root.join("checks"),
            skills: root.join("skills"),
        };
        Ok(paths)
    }
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

impl Catalog {
    pub fn load_runtime(
        explicit_root: Option<&Utf8Path>,
        cwd: &Utf8Path,
    ) -> Result<Self, OrpheumError> {
        let root = runtime_catalog_root(explicit_root, cwd)?;
        Self::load(root)
    }

    pub fn load(root: impl AsRef<Utf8Path>) -> Result<Self, OrpheumError> {
        let paths = CatalogPaths::discover(root)?;
        let scenarios = load_map::<ScenarioDef, ScenarioEntry, _, _>(
            &paths.scenarios,
            |path, metadata, body| ScenarioEntry {
                metadata,
                path,
                body,
            },
            |path| {
                path.extension() == Some("md")
                    && path
                        .file_name()
                        .is_some_and(|name| name.ends_with(".definition.md"))
            },
        )?;
        let workflows = load_map::<WorkflowDef, WorkflowEntry, _, _>(
            &paths.workflows,
            |path, metadata, body| WorkflowEntry {
                metadata,
                path,
                body,
            },
            |path| {
                path.extension() == Some("md")
                    && !matches!(path.file_name(), Some("README.md" | "workflow.template.md"))
            },
        )?;
        let roles = load_map::<RoleDef, RoleEntry, _, _>(
            &paths.roles,
            |path, metadata, body| RoleEntry {
                metadata,
                path,
                body,
            },
            |path| {
                path.extension() == Some("md")
                    && !matches!(path.file_name(), Some("README.md" | "role.template.md"))
            },
        )?;
        let artifacts = load_map::<ArtifactDef, ArtifactEntry, _, _>(
            &paths.artifacts,
            |path, metadata, body| ArtifactEntry {
                metadata,
                path,
                body,
            },
            |path| {
                path.extension() == Some("md")
                    && !matches!(path.file_name(), Some("README.md" | "artifact.template.md"))
            },
        )?;
        let checks = load_map::<CheckDef, CheckEntry, _, _>(
            &paths.checks,
            |path, metadata, body| CheckEntry {
                metadata,
                path,
                body,
            },
            |path| {
                path.extension() == Some("md")
                    && path
                        .file_name()
                        .is_some_and(|name| name.ends_with(".check.md"))
            },
        )?;

        let skills = WalkDir::new(&paths.skills)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().is_file() && entry.file_name().to_string_lossy() == "SKILL.md"
            })
            .filter_map(|entry| {
                entry
                    .path()
                    .parent()
                    .and_then(|parent| parent.file_name())
                    .map(|name| name.to_string_lossy().to_string())
            })
            .collect::<BTreeSet<_>>();

        let catalog = Self {
            paths,
            scenarios,
            workflows,
            roles,
            artifacts,
            checks,
            skills,
        };

        catalog.validate()?;
        Ok(catalog)
    }

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

    fn validate(&self) -> Result<(), OrpheumError> {
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

fn runtime_catalog_root(
    explicit_root: Option<&Utf8Path>,
    cwd: &Utf8Path,
) -> Result<Utf8PathBuf, OrpheumError> {
    if let Some(root) = explicit_root {
        return Ok(root.to_path_buf());
    }

    if let Ok(root) = env::var("ORPHEUM_CATALOG") {
        let root = Utf8PathBuf::from(root);
        return Ok(root);
    }

    for candidate in runtime_catalog_candidates(cwd)? {
        if let Some(root) = find_catalog_root_from(&candidate) {
            return Ok(root);
        }
    }

    Err(OrpheumError::coded(
        OrpheumErrorCode::CatalogNotFound,
        "unable to locate the Orpheum catalog at runtime; pass --catalog <path> or set ORPHEUM_CATALOG",
    ))
}

fn runtime_catalog_candidates(cwd: &Utf8Path) -> Result<Vec<Utf8PathBuf>, OrpheumError> {
    let mut candidates = vec![cwd.to_path_buf()];
    if let Ok(exe_path) = env::current_exe() {
        let exe_path = Utf8PathBuf::from_path_buf(exe_path).map_err(|_| {
            OrpheumError::coded(
                OrpheumErrorCode::CatalogNotFound,
                "current executable path must be valid UTF-8",
            )
        })?;
        if let Some(parent) = exe_path.parent() {
            candidates.push(parent.to_path_buf());
        }
    }
    Ok(candidates)
}

fn find_catalog_root_from(start: &Utf8Path) -> Option<Utf8PathBuf> {
    for ancestor in start.ancestors() {
        if is_catalog_root(ancestor) {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

fn is_catalog_root(path: &Utf8Path) -> bool {
    path.join("scenarios").is_dir()
        && path.join("workflows").is_dir()
        && path.join("roles").is_dir()
        && path.join("artifacts").is_dir()
        && path.join("checks").is_dir()
        && path.join("skills").is_dir()
}

fn load_map<T, E, F, P>(
    dir: &Utf8Path,
    make_entry: F,
    predicate: P,
) -> Result<BTreeMap<String, E>, OrpheumError>
where
    T: for<'de> Deserialize<'de> + Serialize,
    F: Fn(Utf8PathBuf, T, String) -> E,
    P: Fn(&Utf8Path) -> bool,
{
    if !dir.exists() {
        return Err(OrpheumError::coded(
            OrpheumErrorCode::CatalogNotFound,
            format!("required catalog directory missing: {dir}"),
        ));
    }

    let mut map = BTreeMap::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = Utf8PathBuf::from_path_buf(entry.path()).map_err(|_| {
            OrpheumError::coded(
                OrpheumErrorCode::InvalidMetadata,
                "catalog paths must be valid UTF-8",
            )
        })?;

        if !predicate(&path) {
            continue;
        }

        let text = fs::read_to_string(&path)?;
        let (metadata, body) = parse_frontmatter::<T>(&text)?;
        let id = extract_id(&metadata)?;
        map.insert(id, make_entry(path, metadata, body));
    }

    Ok(map)
}

fn extract_id<T>(metadata: &T) -> Result<String, OrpheumError>
where
    T: Serialize,
{
    let value = serde_json::to_value(metadata)?;
    value
        .get("id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
        .ok_or_else(|| {
            OrpheumError::coded(
                OrpheumErrorCode::InvalidMetadata,
                "metadata missing id field",
            )
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn loads_runtime_catalog_from_workspace_context() {
        let workspace_root = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate dir has parent")
            .parent()
            .expect("workspace root")
            .to_path_buf();
        let catalog =
            Catalog::load_runtime(None, &workspace_root).expect("runtime catalog should load");
        assert!(catalog.scenarios.contains_key("project-planning"));
        assert!(catalog.workflows.contains_key("solution-architect-design"));
        assert!(catalog.artifacts.contains_key("business-objectives"));
        assert!(catalog.checks.contains_key("business-objectives"));
    }

    #[test]
    fn rejects_invalid_frontmatter() {
        let temp = tempdir().expect("tempdir");
        let root = Utf8PathBuf::from_path_buf(temp.path().to_path_buf()).expect("utf8 temp path");
        for dir in [
            "scenarios",
            "workflows",
            "roles",
            "artifacts",
            "checks",
            "skills",
        ] {
            fs::create_dir_all(root.join(dir)).expect("dir created");
        }

        fs::write(
            root.join("scenarios").join("bad.definition.md"),
            "---\nid: bad\nkind: scenario\nthis is not yaml\n---\n\n# Bad\n",
        )
        .expect("write invalid scenario");

        let error = Catalog::load(&root).expect_err("invalid metadata should fail");
        assert_eq!(error.code(), OrpheumErrorCode::InvalidMetadata);
    }
}
