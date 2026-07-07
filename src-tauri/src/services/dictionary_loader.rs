use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use tauri::Wry;

use crate::dto::dictionary::{DictionaryBundleFile, DictionaryCategoriesFile, DictionaryEntriesFile};
use crate::errors::AppError;

fn repo_root(_app: &tauri::AppHandle<Wry>) -> Result<PathBuf, AppError> {
    let current = env::current_dir().map_err(|source| AppError::Io {
        context: "failed to resolve current working directory".to_string(),
        source,
    })?;

    if current.join("dictionaries").exists() {
        return Ok(current);
    }

    if let Some(parent) = current.parent() {
        if parent.join("dictionaries").exists() {
            return Ok(parent.to_path_buf());
        }
    }

    Err(AppError::Config {
        context: "failed to locate repository root containing dictionaries/".to_string(),
    })
}

fn read_yaml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, AppError> {
    let raw = fs::read_to_string(path).map_err(|source| AppError::Io {
        context: format!("failed to read yaml file at {}", path.display()),
        source,
    })?;

    serde_yaml::from_str(&raw).map_err(|source| AppError::Yaml {
        context: format!("failed to parse yaml file at {}", path.display()),
        source,
    })
}

pub fn load_categories(app: &tauri::AppHandle<Wry>) -> Result<DictionaryCategoriesFile, AppError> {
    let path = repo_root(app)?.join("dictionaries/taxonomy/categories.yaml");
    read_yaml(&path)
}

pub fn load_bundle(app: &tauri::AppHandle<Wry>, bundle_name: &str) -> Result<DictionaryBundleFile, AppError> {
    let path = repo_root(app)?.join(format!("dictionaries/bundles/{bundle_name}.yaml"));
    read_yaml(&path)
}

pub fn load_entries_file(app: &tauri::AppHandle<Wry>, relative_path: &str) -> Result<DictionaryEntriesFile, AppError> {
    let path = repo_root(app)?.join("dictionaries").join(relative_path);
    read_yaml(&path)
}
