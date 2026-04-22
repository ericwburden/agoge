pub use crate::catalog_types::{
    ArtifactDef, ArtifactEntry, Catalog, CatalogHealth, CatalogPaths, CheckDef, CheckEntry,
    CheckMode, CheckSeverity, DoctorReport, DoctorWarning, EntityRef, HealthCounts, OutputMode,
    ResolvedScenario, RoleDef, RoleEntry, ScenarioDef, ScenarioEntry, ScenarioListItem,
    WorkflowDef, WorkflowEntry,
};

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;

    use camino::Utf8PathBuf;
    use tempfile::tempdir;

    use crate::catalog::Catalog;
    use crate::error::OrpheumErrorCode;

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
