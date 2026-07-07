use tauri::Wry;

use crate::application::failure_tracking;
use crate::dto::frame::AlternateTakeResult;
use crate::errors::AppError;
use crate::filesystem;
use crate::providers::{self, ProviderExecution};
use crate::persistence::{
    database, frame_repository, generation_job_repository, review_repository,
    roll_repository,
};

pub fn create_alternate_take(
    app: &tauri::AppHandle<Wry>,
    roll_id: i64,
    frame_id: i64,
) -> Result<AlternateTakeResult, AppError> {
    let db_path = database::database_path(app)?;
    let connection = database::open_connection(&db_path)?;
    let detail = frame_repository::fetch_roll_detail(&connection, roll_id)?;

    if !frame_repository::frame_exists(&connection, frame_id, roll_id)? {
        return Err(AppError::Config {
            context: format!("frame {frame_id} does not belong to roll {roll_id}"),
        });
    }

    roll_repository::select_frame(&connection, roll_id, frame_id)?;
    roll_repository::update_roll_status(&connection, roll_id, "alternate_take_generating")?;
    let alt_job_id = generation_job_repository::create_alternate_take_job(&connection, roll_id)?;
    generation_job_repository::mark_job_running_by_id(&connection, alt_job_id)?;
    let result: Result<AlternateTakeResult, AppError> = (|| {
        let generation_context = roll_repository::fetch_roll_generation_context(&connection, roll_id)?;
        let source_frame = detail
            .frames
            .iter()
            .find(|frame| frame.id == frame_id)
            .ok_or_else(|| AppError::Config {
                context: format!("failed to find source frame {frame_id} for roll {roll_id}"),
            })?;
        let execution = match source_frame.image_path.ends_with(".png") {
            true => {
                let source_image_bytes = filesystem::read_image_bytes(&source_frame.image_path)?;
                providers::generate_alternate_take(app, &generation_context.roll_dna, &source_image_bytes)?
            }
            false => ProviderExecution::Placeholder,
        };

        let (alternate_frame_id, response_payload_json) = match execution {
            ProviderExecution::Placeholder => {
                let (image_path, thumb_path) =
                    filesystem::write_placeholder_alternate_take_svg(app, roll_id, frame_id, &detail.country_code)?;
                let alternate_frame_id = frame_repository::insert_alternate_take_frame(
                    &connection,
                    roll_id,
                    alt_job_id,
                    frame_id,
                    &image_path,
                    &thumb_path,
                    960,
                    960,
                    &serde_json::json!({
                        "mode": "local_placeholder_alternate_take",
                        "parent_frame_id": frame_id
                    })
                    .to_string(),
                )?;
                (
                    alternate_frame_id,
                    serde_json::json!({
                        "mode": "local_placeholder_alternate_take",
                        "parent_frame_id": frame_id,
                        "alternate_frame_id": alternate_frame_id
                    })
                    .to_string(),
                )
            }
            ProviderExecution::Remote(batch) => {
                let image = batch.images.first().ok_or_else(|| AppError::Config {
                    context: format!("OpenAI returned no alternate-take image for roll {roll_id}"),
                })?;
                let (image_path, thumb_path) = filesystem::write_generated_png(
                    app,
                    roll_id,
                    &format!("alternate-take-{frame_id}"),
                    &image.bytes,
                )?;
                let alternate_frame_id = frame_repository::insert_alternate_take_frame(
                    &connection,
                    roll_id,
                    alt_job_id,
                    frame_id,
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
                        "parent_frame_id": frame_id
                    })
                    .to_string(),
                )?;
                (alternate_frame_id, batch.response_payload_json)
            }
        };

        let review = review_repository::insert_review_result(&connection, alternate_frame_id)?;
        frame_repository::update_frame_review_status(&connection, alternate_frame_id, "complete")?;

        generation_job_repository::mark_job_succeeded(&connection, alt_job_id, &response_payload_json)?;
        roll_repository::update_roll_status(&connection, roll_id, "completed")?;

        connection
            .execute(
                "
                INSERT INTO roll_events (roll_id, event_type, payload_json)
                VALUES (?1, 'frame_selected', ?2)
                ",
                rusqlite::params![
                    roll_id,
                    serde_json::json!({ "frame_id": frame_id }).to_string()
                ],
            )
            .map_err(|source| AppError::Sqlite {
                context: format!("failed to insert frame_selected event for roll {roll_id}"),
                source,
            })?;
        connection
            .execute(
                "
                INSERT INTO roll_events (roll_id, event_type, payload_json)
                VALUES (?1, 'alternate_take_completed', ?2)
                ",
                rusqlite::params![
                    roll_id,
                    serde_json::json!({
                        "parent_frame_id": frame_id,
                        "alternate_frame_id": alternate_frame_id,
                        "review_frame_id": alternate_frame_id
                    })
                    .to_string()
                ],
            )
            .map_err(|source| AppError::Sqlite {
                context: format!("failed to insert alternate_take_completed event for roll {roll_id}"),
                source,
            })?;

        let roll = frame_repository::fetch_roll_detail(&connection, roll_id)?;
        Ok(AlternateTakeResult { roll, review })
    })();

    if let Err(error) = result {
        failure_tracking::persist_generation_failure(
            &connection,
            roll_id,
            alt_job_id,
            &error,
            "alternate_take_failed",
            serde_json::json!({
                "generation_job_id": alt_job_id,
                "frame_id": frame_id
            }),
        )?;
        return Err(error);
    }

    result
}
