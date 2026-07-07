use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::Connection;
use tauri::{Manager, Wry};

use crate::errors::AppError;

pub const APP_DB_FILE: &str = "foundframe.sqlite3";

pub fn ensure_app_data_dir(app: &tauri::AppHandle<Wry>) -> Result<PathBuf, AppError> {
    let base_dir = app
        .path()
        .app_data_dir()
        .map_err(|source| AppError::PathResolution { source })?;

    fs::create_dir_all(&base_dir).map_err(|source| AppError::Io {
        context: format!("failed to create app data dir at {}", base_dir.display()),
        source,
    })?;

    Ok(base_dir)
}

pub fn database_path(app: &tauri::AppHandle<Wry>) -> Result<PathBuf, AppError> {
    Ok(ensure_app_data_dir(app)?.join(APP_DB_FILE))
}

pub fn open_connection(path: &Path) -> Result<Connection, AppError> {
    let connection = Connection::open(path).map_err(|source| AppError::Sqlite {
        context: format!("failed to open database at {}", path.display()),
        source,
    })?;

    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|source| AppError::Sqlite {
            context: "failed to enable foreign keys".to_string(),
            source,
        })?;

    Ok(connection)
}
