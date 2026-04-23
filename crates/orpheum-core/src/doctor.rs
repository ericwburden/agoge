use camino::Utf8Path;

use crate::catalog::{Catalog, DoctorReport, HealthCounts};
use crate::catalog_runtime::runtime_catalog_resolution;
use crate::error::OrpheumError;
use crate::project_config::{CatalogSource, ProjectState, inspect_local_config};

pub fn run_doctor(
    explicit_catalog_root: Option<&Utf8Path>,
    project_root: &Utf8Path,
) -> Result<DoctorReport, OrpheumError> {
    let local_config = inspect_local_config(project_root);
    let resolution = runtime_catalog_resolution(explicit_catalog_root, project_root).ok();
    let project_state = if project_root.join(".orpheum").exists() {
        ProjectState::Active
    } else if resolution.is_some() || local_config.valid || local_config.exists {
        ProjectState::Initialized
    } else {
        ProjectState::Misconfigured
    };
    let (catalog_root, catalog_source, counts, mut warnings) = if let Some(resolution) = resolution
    {
        let catalog = Catalog::load(&resolution.root)?;
        let health = catalog.health(project_root);
        (
            Some(catalog.paths.root.clone()),
            resolution.source,
            health.counts,
            health.warnings,
        )
    } else {
        (
            None,
            CatalogSource::Unresolved,
            HealthCounts {
                scenarios: 0,
                workflows: 0,
                roles: 0,
                artifacts: 0,
                checks: 0,
                skills: 0,
            },
            Vec::new(),
        )
    };

    if local_config.exists && !local_config.valid {
        warnings.push(crate::catalog::DoctorWarning {
            code: "LOCAL_CONFIG_INVALID".into(),
            message: local_config
                .message
                .clone()
                .unwrap_or_else(|| "local Orpheum config is missing or invalid".into()),
        });
    }

    if matches!(catalog_source, CatalogSource::Unresolved) {
        warnings.push(crate::catalog::DoctorWarning {
            code: "CATALOG_UNRESOLVED".into(),
            message: "no valid Orpheum catalog source was resolved for this project".into(),
        });
    }

    let orpheum_gitignored = !warnings.iter().any(|warning| {
        warning.code == "GITIGNORE_MISSING" || warning.code == "ORPHEUM_NOT_IGNORED"
    });

    Ok(DoctorReport {
        project_state,
        catalog_root: catalog_root.clone(),
        catalog_source,
        project_root: project_root.to_path_buf(),
        local_config,
        counts,
        active_session_present: project_root.join(".orpheum").exists(),
        orpheum_gitignored,
        warnings,
        recovery_commands: recovery_commands(project_root, catalog_root.as_deref()),
    })
}

fn recovery_commands(project_root: &Utf8Path, catalog_root: Option<&Utf8Path>) -> Vec<String> {
    let mut commands = Vec::new();
    if let Some(root) = catalog_root {
        commands.push(format!("orpheum init --catalog {}", root));
    } else {
        commands.push("orpheum init --catalog <path-to-orpheum-catalog>".into());
        commands.push("ORPHEUM_CATALOG=<path-to-orpheum-catalog>".into());
        commands.push(format!(
            "orpheum --catalog <path-to-orpheum-catalog> scenario list --json"
        ));
        commands.push(format!("orpheum doctor --json"));
    }
    if project_root.join(".orpheum").exists() {
        commands.push("orpheum status --json".into());
    }
    commands
}
