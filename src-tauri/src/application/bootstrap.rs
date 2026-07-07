use rusqlite::Connection;
use tauri::Wry;

use crate::dto::dictionary::DictionaryBundleFile;
use crate::errors::AppError;
use crate::persistence::{database, dictionary_repository, migrations};
use crate::services::dictionary_loader;

pub struct BootstrapResult {
    pub database_path: String,
    pub countries_count: i64,
    pub entries_count: i64,
    pub bundle_version: String,
}

fn import_bundle(
    app: &tauri::AppHandle<Wry>,
    connection: &mut Connection,
    bundle: &DictionaryBundleFile,
) -> Result<(), AppError> {
    dictionary_repository::upsert_bundle(connection, bundle)?;

    for include in &bundle.includes {
        let entries_file = dictionary_loader::load_entries_file(app, include)?;
        dictionary_repository::upsert_entries_file(connection, &entries_file)?;
    }

    Ok(())
}

pub fn ensure_bootstrapped(app: &tauri::AppHandle<Wry>) -> Result<BootstrapResult, AppError> {
    let db_path = database::database_path(app)?;
    let mut connection = database::open_connection(&db_path)?;
    migrations::apply_all(&connection)?;

    let categories = dictionary_loader::load_categories(app)?;
    dictionary_repository::upsert_categories(&mut connection, &categories)?;

    let bundle = dictionary_loader::load_bundle(app, "v1-initial")?;
    import_bundle(app, &mut connection, &bundle)?;

    Ok(BootstrapResult {
        database_path: db_path.display().to_string(),
        countries_count: dictionary_repository::count_countries(&connection)?,
        entries_count: dictionary_repository::count_entries(&connection)?,
        bundle_version: bundle.version,
    })
}
