use std::fs;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use crate::catalog_loading::load_catalog_for_root;
use crate::error::OrpheumError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogSource {
    Explicit,
    LocalConfig,
    Env,
    RuntimeDiscovery,
    Unresolved,
}

impl CatalogSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Explicit => "explicit",
            Self::LocalConfig => "local_config",
            Self::Env => "env",
            Self::RuntimeDiscovery => "runtime_discovery",
            Self::Unresolved => "unresolved",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectState {
    Initialized,
    Active,
    Misconfigured,
}

impl ProjectState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Initialized => "initialized",
            Self::Active => "active",
            Self::Misconfigured => "misconfigured",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalProjectConfig {
    pub catalog_root: Utf8PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfigStatus {
    pub file: Utf8PathBuf,
    pub exists: bool,
    pub valid: bool,
    pub catalog_root: Option<Utf8PathBuf>,
    pub message: Option<String>,
}

pub fn local_config_dir(project_root: &Utf8Path) -> Utf8PathBuf {
    project_root.join(".codex").join("orpheum")
}

pub fn local_config_file(project_root: &Utf8Path) -> Utf8PathBuf {
    local_config_dir(project_root).join("config.json")
}

pub fn write_local_config(
    project_root: &Utf8Path,
    catalog_root: &Utf8Path,
) -> Result<Utf8PathBuf, OrpheumError> {
    let config_dir = local_config_dir(project_root);
    let config_file = local_config_file(project_root);
    fs::create_dir_all(&config_dir)?;
    let config = LocalProjectConfig {
        catalog_root: catalog_root.to_path_buf(),
    };
    fs::write(&config_file, serde_json::to_string_pretty(&config)?)?;
    Ok(config_file)
}

pub fn read_local_config(
    project_root: &Utf8Path,
) -> Result<Option<LocalProjectConfig>, OrpheumError> {
    let config_file = local_config_file(project_root);
    if !config_file.exists() {
        return Ok(None);
    }

    let config = serde_json::from_str::<LocalProjectConfig>(&fs::read_to_string(config_file)?)?;
    Ok(Some(config))
}

pub fn valid_local_config_catalog_root(project_root: &Utf8Path) -> Option<Utf8PathBuf> {
    let status = inspect_local_config(project_root);
    if status.valid {
        status.catalog_root
    } else {
        None
    }
}

pub fn inspect_local_config(project_root: &Utf8Path) -> LocalConfigStatus {
    let file = local_config_file(project_root);
    if !file.exists() {
        return LocalConfigStatus {
            file,
            exists: false,
            valid: false,
            catalog_root: None,
            message: Some("local Orpheum config file not found".into()),
        };
    }

    match fs::read_to_string(&file) {
        Ok(content) => match serde_json::from_str::<LocalProjectConfig>(&content) {
            Ok(config) => {
                let root = config.catalog_root.clone();
                match load_catalog_for_root(&root) {
                    Ok(_) => LocalConfigStatus {
                        file,
                        exists: true,
                        valid: true,
                        catalog_root: Some(root),
                        message: None,
                    },
                    Err(err) => LocalConfigStatus {
                        file,
                        exists: true,
                        valid: false,
                        catalog_root: Some(root),
                        message: Some(err.to_string()),
                    },
                }
            }
            Err(err) => LocalConfigStatus {
                file,
                exists: true,
                valid: false,
                catalog_root: None,
                message: Some(format!("invalid JSON in local Orpheum config: {err}")),
            },
        },
        Err(err) => LocalConfigStatus {
            file,
            exists: true,
            valid: false,
            catalog_root: None,
            message: Some(err.to_string()),
        },
    }
}
