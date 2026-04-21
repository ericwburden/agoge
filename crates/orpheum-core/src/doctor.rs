use camino::Utf8Path;

use crate::catalog::{Catalog, DoctorReport};
use crate::error::OrpheumError;

pub fn run_doctor(
    catalog: &Catalog,
    project_root: &Utf8Path,
) -> Result<DoctorReport, OrpheumError> {
    let health = catalog.health(project_root);
    Ok(DoctorReport {
        catalog_root: catalog.paths.root.clone(),
        project_root: project_root.to_path_buf(),
        counts: health.counts,
        active_session_present: project_root.join(".orpheum").exists(),
        orpheum_gitignored: !health.warnings.iter().any(|warning| {
            warning.code == "GITIGNORE_MISSING" || warning.code == "ORPHEUM_NOT_IGNORED"
        }),
        warnings: health.warnings,
    })
}
