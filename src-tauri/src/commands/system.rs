use serde::Serialize;
use tauri::Wry;

use crate::application::alternate_take;
use crate::application::bootstrap;
use crate::application::contact_sheet;
use crate::application::rolls;
use crate::dto::frame::{AlternateTakeResult, ArchiveQueryRequest, ArchiveRollSummary, RollDetail};
use crate::dto::roll::{CreateRollRequest, CreatedRollSummary};
use crate::dto::setup::{
    CountryOption, DeletePresetRequest, PresetSummary, ProviderCredentialStatusDto, RenamePresetRequest,
    SavePresetRequest, SaveProviderApiKeyRequest, SettingsSnapshot, SetupBootstrapData,
};
use crate::errors::AppError;
use crate::keychain;
use crate::persistence::{
    database, dictionary_repository, favorites_repository, frame_repository, presets_repository,
    provider_health_repository,
};
use crate::providers;
use crate::services::dictionary_loader;
use crate::services::setup_resolver;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppBootstrapStatus {
    pub app_name: &'static str,
    pub dictionary_source: &'static str,
    pub initial_provider: &'static str,
    pub review_engine: &'static str,
    pub database_path: String,
    pub countries_count: i64,
    pub entries_count: i64,
    pub bundle_version: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DictionaryDebugStatus {
    pub categories_count: usize,
    pub bundle_version: String,
    pub bundle_includes: usize,
    pub first_include_country: String,
    pub first_include_entries: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConnectionStatusDto {
    pub provider_key: String,
    pub checked_model: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSetupPreviewDto {
    pub country_code: String,
    pub moment: String,
    pub place: String,
    pub time: String,
    pub season: String,
    pub weather: String,
    pub tiny_detail: String,
}

fn to_string_error(error: AppError) -> String {
    error.to_string()
}

#[tauri::command]
pub fn app_bootstrap_status(app: tauri::AppHandle<Wry>) -> Result<AppBootstrapStatus, String> {
    let bootstrap = bootstrap::ensure_bootstrapped(&app).map_err(to_string_error)?;

    Ok(AppBootstrapStatus {
        app_name: "FoundFrame",
        dictionary_source: "yaml_to_sqlite",
        initial_provider: "openai",
        review_engine: "rule_based_v1",
        database_path: bootstrap.database_path,
        countries_count: bootstrap.countries_count,
        entries_count: bootstrap.entries_count,
        bundle_version: bootstrap.bundle_version,
    })
}

#[tauri::command]
pub fn dictionary_debug_status(app: tauri::AppHandle<Wry>) -> Result<DictionaryDebugStatus, String> {
    let categories = dictionary_loader::load_categories(&app).map_err(to_string_error)?;
    let bundle = dictionary_loader::load_bundle(&app, "v1-initial").map_err(to_string_error)?;
    let first_include = bundle
        .includes
        .first()
        .ok_or_else(|| "bundle contains no include files".to_string())?;
    let first_entries = dictionary_loader::load_entries_file(&app, first_include).map_err(to_string_error)?;

    Ok(DictionaryDebugStatus {
        categories_count: categories.categories.len(),
        bundle_version: bundle.version,
        bundle_includes: bundle.includes.len(),
        first_include_country: first_entries.country,
        first_include_entries: first_entries.entries.len(),
    })
}

#[tauri::command]
pub fn setup_bootstrap_data(app: tauri::AppHandle<Wry>) -> Result<SetupBootstrapData, String> {
    let bootstrap = bootstrap::ensure_bootstrapped(&app).map_err(to_string_error)?;
    let db_path = std::path::PathBuf::from(bootstrap.database_path);
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    let countries = dictionary_repository::list_countries(&connection).map_err(to_string_error)?;
    let default_country_code =
        dictionary_repository::default_country_code(&connection).map_err(to_string_error)?;

    Ok(SetupBootstrapData {
        default_country_code,
        countries: countries
            .into_iter()
            .map(|(code, display_name, is_default)| CountryOption {
                code,
                display_name,
                is_default,
            })
            .collect(),
        suggested_times: vec![
            "early_morning",
            "morning",
            "noon",
            "afternoon",
            "late_afternoon",
            "evening",
            "night",
        ],
        suggested_seasons: vec!["spring", "summer", "autumn", "winter"],
        suggested_weather: vec!["clear", "cloudy", "rain", "drizzle", "humid", "snow"],
    })
}

#[tauri::command]
pub fn create_roll(
    app: tauri::AppHandle<Wry>,
    request: CreateRollRequest,
) -> Result<CreatedRollSummary, String> {
    rolls::create_roll(&app, request).map_err(to_string_error)
}

#[tauri::command]
pub fn resolve_setup_preview(
    app: tauri::AppHandle<Wry>,
    request: CreateRollRequest,
) -> Result<ResolvedSetupPreviewDto, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    let fallback_default = dictionary_repository::default_country_code(&connection).map_err(to_string_error)?;
    let country_code =
        setup_resolver::resolved_country_code(&connection, &request, fallback_default).map_err(to_string_error)?;
    let resolved = setup_resolver::resolve_setup_values(&connection, &request, &country_code)
        .map_err(to_string_error)?;

    Ok(ResolvedSetupPreviewDto {
        country_code: country_code,
        moment: resolved.moment,
        place: resolved.place,
        time: resolved.time,
        season: resolved.season,
        weather: resolved.weather,
        tiny_detail: resolved.tiny_detail,
    })
}

#[tauri::command]
pub fn process_contact_sheet_roll(app: tauri::AppHandle<Wry>, roll_id: i64) -> Result<RollDetail, String> {
    contact_sheet::simulate_contact_sheet(&app, roll_id).map_err(to_string_error)?;

    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    frame_repository::fetch_roll_detail(&connection, roll_id).map_err(to_string_error)
}

#[tauri::command]
pub fn roll_detail(app: tauri::AppHandle<Wry>, roll_id: i64) -> Result<RollDetail, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    frame_repository::fetch_roll_detail(&connection, roll_id).map_err(to_string_error)
}

#[tauri::command]
pub fn select_frame_and_generate_alternate_take(
    app: tauri::AppHandle<Wry>,
    roll_id: i64,
    frame_id: i64,
) -> Result<AlternateTakeResult, String> {
    alternate_take::create_alternate_take(&app, roll_id, frame_id).map_err(to_string_error)
}

#[tauri::command]
pub fn settings_snapshot(app: tauri::AppHandle<Wry>) -> Result<SettingsSnapshot, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    let provider_credentials = keychain::provider_credential_statuses()
        .map_err(to_string_error)?
        .into_iter()
        .map(|status| {
            let health = provider_health_repository::fetch_provider_health(&connection, &status.provider_key)
                .map_err(to_string_error)?;
            let health_status = if !status.has_api_key {
                "unconfigured".to_string()
            } else {
                health
                    .as_ref()
                    .map(|record| record.status.clone())
                    .unwrap_or_else(|| "saved_unverified".to_string())
            };

            Ok(ProviderCredentialStatusDto {
                provider_key: status.provider_key,
                has_api_key: status.has_api_key,
                account_label: status.account_label,
                health_status,
                last_check_message: health.as_ref().and_then(|record| record.last_check_message.clone()),
                last_check_at: health.and_then(|record| record.last_check_at),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(SettingsSnapshot {
        provider_credentials,
    })
}

#[tauri::command]
pub fn save_provider_api_key(
    app: tauri::AppHandle<Wry>,
    request: SaveProviderApiKeyRequest,
) -> Result<SettingsSnapshot, String> {
    match request.provider_key.as_str() {
        "openai" => {
            keychain::store_openai_api_key(&request.api_key).map_err(to_string_error)?;
            let db_path = database::database_path(&app).map_err(to_string_error)?;
            let connection = database::open_connection(&db_path).map_err(to_string_error)?;
            provider_health_repository::upsert_provider_health(
                &connection,
                "openai",
                "saved_unverified",
                Some("Key saved. Connection has not been checked yet."),
                None,
            )
            .map_err(to_string_error)?;
            settings_snapshot(app)
        }
        _ => Err("unsupported provider".to_string()),
    }
}

#[tauri::command]
pub fn clear_provider_api_key(
    app: tauri::AppHandle<Wry>,
    provider_key: String,
) -> Result<SettingsSnapshot, String> {
    match provider_key.as_str() {
        "openai" => {
            keychain::clear_openai_api_key().map_err(to_string_error)?;
            let db_path = database::database_path(&app).map_err(to_string_error)?;
            let connection = database::open_connection(&db_path).map_err(to_string_error)?;
            provider_health_repository::upsert_provider_health(
                &connection,
                "openai",
                "unconfigured",
                Some("No API key is saved."),
                None,
            )
            .map_err(to_string_error)?;
            settings_snapshot(app)
        }
        _ => Err("unsupported provider".to_string()),
    }
}

#[tauri::command]
pub fn test_provider_connection(
    app: tauri::AppHandle<Wry>,
    provider_key: String,
) -> Result<ProviderConnectionStatusDto, String> {
    match provider_key.as_str() {
        "openai" => {
            let checked_at = chrono::Utc::now().to_rfc3339();
            let db_path = database::database_path(&app).map_err(to_string_error)?;
            let connection = database::open_connection(&db_path).map_err(to_string_error)?;
            match providers::test_openai_connection() {
                Ok(status) => {
                    provider_health_repository::upsert_provider_health(
                        &connection,
                        "openai",
                        "ready",
                        Some(&status.message),
                        Some(&checked_at),
                    )
                    .map_err(to_string_error)?;
                    Ok(ProviderConnectionStatusDto {
                        provider_key: status.provider_key,
                        checked_model: status.checked_model,
                        message: status.message,
                    })
                }
                Err(error) => {
                    provider_health_repository::upsert_provider_health(
                        &connection,
                        "openai",
                        "degraded",
                        Some(&error.to_string()),
                        Some(&checked_at),
                    )
                    .map_err(to_string_error)?;
                    Err(to_string_error(error))
                }
            }
        }
        _ => Err("unsupported provider".to_string()),
    }
}

#[tauri::command]
pub fn recent_rolls(
    app: tauri::AppHandle<Wry>,
    request: Option<ArchiveQueryRequest>,
) -> Result<Vec<ArchiveRollSummary>, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    let request = request.unwrap_or(ArchiveQueryRequest {
        query: None,
        status: None,
        sort: None,
        limit: Some(24),
    });
    frame_repository::list_recent_rolls(&connection, &request).map_err(to_string_error)
}

#[tauri::command]
pub fn set_frame_favorite(
    app: tauri::AppHandle<Wry>,
    roll_id: i64,
    frame_id: i64,
    is_favorite: bool,
) -> Result<RollDetail, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;

    if !frame_repository::frame_exists(&connection, frame_id, roll_id).map_err(to_string_error)? {
        return Err("frame does not belong to roll".to_string());
    }

    favorites_repository::set_favorite(&connection, frame_id, is_favorite).map_err(to_string_error)?;
    frame_repository::fetch_roll_detail(&connection, roll_id).map_err(to_string_error)
}

#[tauri::command]
pub fn presets(app: tauri::AppHandle<Wry>) -> Result<Vec<PresetSummary>, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    presets_repository::list_presets(&connection).map_err(to_string_error)
}

#[tauri::command]
pub fn save_preset(
    app: tauri::AppHandle<Wry>,
    request: SavePresetRequest,
) -> Result<Vec<PresetSummary>, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    let country_code = request
        .input_snapshot
        .country
        .value
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "jp".to_string());
    let country_id = dictionary_repository::country_id_by_code(&connection, &country_code)
        .map_err(to_string_error)?
        .ok_or_else(|| "country for preset was not found".to_string())?;

    presets_repository::save_preset(
        &connection,
        &request.name,
        country_id,
        &request.input_snapshot,
        request.is_locked_random_template,
    )
    .map_err(to_string_error)?;

    presets_repository::list_presets(&connection).map_err(to_string_error)
}

#[tauri::command]
pub fn delete_preset(
    app: tauri::AppHandle<Wry>,
    request: DeletePresetRequest,
) -> Result<Vec<PresetSummary>, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    presets_repository::delete_preset(&connection, request.preset_id).map_err(to_string_error)?;
    presets_repository::list_presets(&connection).map_err(to_string_error)
}

#[tauri::command]
pub fn rename_preset(
    app: tauri::AppHandle<Wry>,
    request: RenamePresetRequest,
) -> Result<Vec<PresetSummary>, String> {
    let db_path = database::database_path(&app).map_err(to_string_error)?;
    let connection = database::open_connection(&db_path).map_err(to_string_error)?;
    presets_repository::rename_preset(&connection, request.preset_id, &request.name).map_err(to_string_error)?;
    presets_repository::list_presets(&connection).map_err(to_string_error)
}
