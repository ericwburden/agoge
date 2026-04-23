use std::env;

use camino::{Utf8Path, Utf8PathBuf};

use crate::catalog_loading::load_catalog_for_root;
use crate::error::{OrpheumError, OrpheumErrorCode};
use crate::project_config::{CatalogSource, valid_local_config_catalog_root};

pub(crate) fn runtime_catalog_root(
    explicit_root: Option<&Utf8Path>,
    cwd: &Utf8Path,
) -> Result<Utf8PathBuf, OrpheumError> {
    runtime_catalog_resolution(explicit_root, cwd).map(|resolution| resolution.root)
}

#[derive(Debug, Clone)]
pub(crate) struct CatalogResolution {
    pub root: Utf8PathBuf,
    pub source: CatalogSource,
}

pub(crate) fn runtime_catalog_resolution(
    explicit_root: Option<&Utf8Path>,
    cwd: &Utf8Path,
) -> Result<CatalogResolution, OrpheumError> {
    if let Some(root) = explicit_root {
        ensure_valid_catalog_root(root)?;
        return Ok(CatalogResolution {
            root: root.to_path_buf(),
            source: CatalogSource::Explicit,
        });
    }

    if let Some(root) = valid_local_config_catalog_root(cwd) {
        return Ok(CatalogResolution {
            root,
            source: CatalogSource::LocalConfig,
        });
    }

    if let Ok(root) = env::var("ORPHEUM_CATALOG") {
        let root = Utf8PathBuf::from(root);
        ensure_valid_catalog_root(&root)?;
        return Ok(CatalogResolution {
            root,
            source: CatalogSource::Env,
        });
    }

    for candidate in runtime_catalog_candidates(cwd)? {
        if let Some(root) = find_catalog_root_from(&candidate) {
            ensure_valid_catalog_root(&root)?;
            return Ok(CatalogResolution {
                root,
                source: CatalogSource::RuntimeDiscovery,
            });
        }
    }

    Err(OrpheumError::coded(
        OrpheumErrorCode::CatalogNotFound,
        "unable to locate the Orpheum catalog at runtime; run `orpheum init --catalog <path>` to configure this project, or set ORPHEUM_CATALOG as a compatibility fallback",
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

fn ensure_valid_catalog_root(root: &Utf8Path) -> Result<(), OrpheumError> {
    load_catalog_for_root(root).map(|_| ())
}
