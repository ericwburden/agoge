use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::catalog::{
    ArtifactDef, ArtifactEntry, Catalog, CatalogPaths, CheckDef, CheckEntry, RoleDef, RoleEntry,
    ScenarioDef, ScenarioEntry, WorkflowDef, WorkflowEntry,
};
use crate::catalog_runtime::runtime_catalog_root;
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::frontmatter::parse_frontmatter;

impl CatalogPaths {
    pub fn discover(root: impl AsRef<Utf8Path>) -> Result<Self, OrpheumError> {
        let root = root.as_ref();
        if !root.exists() {
            return Err(OrpheumError::coded(
                OrpheumErrorCode::CatalogNotFound,
                format!("catalog root does not exist: {root}"),
            ));
        }

        Ok(Self {
            root: root.to_path_buf(),
            scenarios: root.join("scenarios"),
            workflows: root.join("workflows"),
            roles: root.join("roles"),
            artifacts: root.join("artifacts"),
            checks: root.join("checks"),
            skills: root.join("skills"),
        })
    }
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
