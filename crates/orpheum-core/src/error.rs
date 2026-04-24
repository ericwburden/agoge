use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrpheumErrorCode {
    CatalogNotFound,
    InvalidMetadata,
    ScenarioNotFound,
    SessionAlreadyActive,
    InvalidSessionState,
    NoActiveSession,
    CheckFailed,
    Io,
}

impl OrpheumErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CatalogNotFound => "CATALOG_NOT_FOUND",
            Self::InvalidMetadata => "INVALID_METADATA",
            Self::ScenarioNotFound => "SCENARIO_NOT_FOUND",
            Self::SessionAlreadyActive => "SESSION_ALREADY_ACTIVE",
            Self::InvalidSessionState => "INVALID_SESSION_STATE",
            Self::NoActiveSession => "NO_ACTIVE_SESSION",
            Self::CheckFailed => "CHECK_FAILED",
            Self::Io => "IO_ERROR",
        }
    }
}

#[derive(Debug, Error)]
pub enum OrpheumError {
    #[error("{message}")]
    Coded {
        code: OrpheumErrorCode,
        message: String,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

impl OrpheumError {
    pub fn code(&self) -> OrpheumErrorCode {
        match self {
            Self::Coded { code, .. } => *code,
            Self::Io(_) | Self::Json(_) | Self::Yaml(_) => OrpheumErrorCode::Io,
        }
    }

    pub fn coded(code: OrpheumErrorCode, message: impl Into<String>) -> Self {
        Self::Coded {
            code,
            message: message.into(),
        }
    }
}
