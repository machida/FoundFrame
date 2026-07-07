use tauri::Wry;

use crate::application::failure_tracking;
use crate::errors::AppError;
use crate::filesystem;
use crate::providers::{self, ProviderExecution};
use crate::persistence::{
    database, frame_repository, generation_job_repository, roll_repository,
};

pub fn simulate_contact_sheet(
    app: &tauri::AppHandle<Wry>,
    roll_id: i64,
) -> Result<(), AppError> {
    let db_path = database::database_path(app)?;
    let connection = database::open_connection(&db_path)?;
    let detail = frame_repository::fetch_roll_detail(&connection, roll_id)?;
    let job = generation_job_repository::latest_contact_sheet_job(&connection, roll_id)?
        .ok_or_else(|| AppError::Config {
            context: format!("no contact_sheet job found for roll {roll_id}"),
        })?;

    if !detail.frames.is_empty() {
        return Ok(());
    }

    generation_job_repository::mark_job_running(&connection, job.job_id)?;
    roll_repository::update_roll_status(&connection, roll_id, "generating_contact_sheet")?;

    let result: Result<(), AppError> = (|| {
        let generation_context = roll_repository::fetch_roll_generation_context(&connection, roll_id)?;
        let execution = providers::generate_contact_sheet(app, &generation_context.roll_dna, 8)?;

        let response_payload_json = match execution {
            ProviderExecution::Placeholder => {
                for frame_index in 0..8_i64 {
                    let (image_path, thumb_path) = filesystem::write_placeholder_frame_svg(
                        app,
                        roll_id,
                        frame_index,
                        &detail.country_code,
                        "generating_contact_sheet",
                    )?;

                    frame_repository::insert_contact_sheet_frame(
                        &connection,
                        roll_id,
                        job.job_id,
                        frame_index,
                        &image_path,
                        &thumb_path,
                        960,
                        960,
                        &serde_json::json!({
                            "mode": "local_placeholder",
                            "frame_index": frame_index
                        })
                        .to_string(),
                    )?;
                }

                serde_json::json!({
                    "mode": "local_placeholder_contact_sheet",
                    "frame_count": 8
                })
                .to_string()
            }
            ProviderExecution::Remote(batch) => {
                for (index, image) in batch.images.iter().enumerate() {
                    let frame_index = index as i64;
                    let (image_path, thumb_path) = filesystem::write_generated_png(
                        app,
                        roll_id,
                        &format!("frame-{frame_index}"),
                        &image.bytes,
                    )?;

                    frame_repository::insert_contact_sheet_frame(
                        &connection,
                        roll_id,
                        job.job_id,
                        frame_index,
                        &image_path,
                        &thumb_path,
                        image.width,
                        image.height,
                        &serde_json::json!({
                            "mode": "openai_remote",
                            "provider_key": batch.provider_key,
                            "provider_model": batch.provider_model,
                            "prompt_engine_version": generation_context.prompt_engine_version,
                            "hidden_prompt": batch.prompt,
                            "frame_index": frame_index
                        })
                        .to_string(),
                    )?;
                }

                batch.response_payload_json
            }
        };

        generation_job_repository::mark_job_succeeded(&connection, job.job_id, &response_payload_json)?;
        roll_repository::update_roll_status(&connection, roll_id, "contact_sheet_ready")?;

        connection
            .execute(
                "
                INSERT INTO roll_events (roll_id, event_type, payload_json)
                VALUES (?1, 'contact_sheet_completed', ?2)
                ",
                rusqlite::params![
                    roll_id,
                    serde_json::json!({
                        "generation_job_id": job.job_id,
                        "frame_count": 8,
                        "mode": "completed"
                    })
                    .to_string()
                ],
            )
            .map_err(|source| AppError::Sqlite {
                context: format!("failed to insert contact sheet completed event for roll {roll_id}"),
                source,
            })?;

        Ok(())
    })();

    if let Err(error) = result {
        failure_tracking::persist_generation_failure(
            &connection,
            roll_id,
            job.job_id,
            &error,
            "contact_sheet_failed",
            serde_json::json!({
                "generation_job_id": job.job_id
            }),
        )?;
        return Err(error);
    }

    Ok(())
}
