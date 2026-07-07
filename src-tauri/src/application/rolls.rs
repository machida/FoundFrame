use tauri::Wry;

use crate::application::bootstrap;
use crate::dto::roll::{CreateRollRequest, CreatedRollSummary};
use crate::errors::AppError;
use crate::persistence::{database, dictionary_repository, roll_repository};
use crate::services::setup_resolver;

pub fn create_roll(
    app: &tauri::AppHandle<Wry>,
    request: CreateRollRequest,
) -> Result<CreatedRollSummary, AppError> {
    let bootstrap = bootstrap::ensure_bootstrapped(app)?;
    let db_path = std::path::PathBuf::from(bootstrap.database_path);
    let mut connection = database::open_connection(&db_path)?;

    let fallback_default = dictionary_repository::default_country_code(&connection)?;
    let country_code = setup_resolver::resolved_country_code(&connection, &request, fallback_default)?;
    let country_id = dictionary_repository::country_id_by_code(&connection, &country_code)?
        .ok_or_else(|| AppError::Config {
            context: format!("country code {country_code} was not found in imported dictionary data"),
        })?;
    let resolved_values = setup_resolver::resolve_setup_values(&connection, &request, &country_code)?;

    let created = roll_repository::create_roll(
        &mut connection,
        country_id,
        &country_code,
        &request,
        &resolved_values,
    )?;

    Ok(CreatedRollSummary {
        roll_id: created.roll_id,
        status: created.status,
        country_code: created.country_code,
        prompt_engine_version: created.prompt_engine_version,
        provider_key: created.provider_key,
        provider_model: created.provider_model,
        contact_sheet_frame_count: created.contact_sheet_frame_count,
        created_at: created.created_at,
        generation_job_id: created.generation_job_id,
        generation_job_status: created.generation_job_status,
    })
}
