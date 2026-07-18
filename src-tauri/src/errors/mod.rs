use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{context}")]
    Io {
        context: String,
        #[source]
        source: io::Error,
    },

    #[error("{context}")]
    Sqlite {
        context: String,
        #[source]
        source: rusqlite::Error,
    },

    #[error("{context}")]
    Yaml {
        context: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("{context}")]
    Json {
        context: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("{context}")]
    Keychain { context: String },

    #[error("{context}")]
    Provider { code: String, context: String },

    #[error("failed to resolve application path")]
    PathResolution {
        #[source]
        source: tauri::Error,
    },

    #[error("{context}")]
    Config { context: String },
}

impl AppError {
    pub fn code(&self) -> String {
        match self {
            AppError::Io { .. } => "io_error".to_string(),
            AppError::Sqlite { .. } => "sqlite_error".to_string(),
            AppError::Yaml { .. } => "yaml_error".to_string(),
            AppError::Json { .. } => "json_error".to_string(),
            AppError::Keychain { .. } => "keychain_error".to_string(),
            AppError::Provider { code, .. } => code.clone(),
            AppError::PathResolution { .. } => "path_resolution_error".to_string(),
            AppError::Config { .. } => "config_error".to_string(),
        }
    }
}
