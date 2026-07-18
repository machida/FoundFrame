use rusqlite::{params, Connection, OptionalExtension};

use crate::errors::AppError;

pub struct ContactSheetJob {
    pub job_id: i64,
}

pub fn latest_contact_sheet_job(
    connection: &Connection,
    roll_id: i64,
) -> Result<Option<ContactSheetJob>, AppError> {
    connection
        .query_row(
            "
            SELECT id, roll_id, status
            FROM generation_jobs
            WHERE roll_id = ?1 AND job_type = 'contact_sheet'
            ORDER BY id DESC
            LIMIT 1
            ",
            [roll_id],
            |row| {
                Ok(ContactSheetJob {
                    job_id: row.get(0)?,
                })
            },
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch latest contact sheet job for roll {roll_id}"),
            source,
        })
}

pub fn mark_job_running(connection: &Connection, job_id: i64) -> Result<(), AppError> {
    connection
        .execute(
            "
            UPDATE generation_jobs
            SET status = 'running',
                started_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            ",
            [job_id],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to mark generation job {job_id} running"),
            source,
        })?;
    Ok(())
}

pub fn mark_job_succeeded(
    connection: &Connection,
    job_id: i64,
    response_payload_json: &str,
) -> Result<(), AppError> {
    connection
        .execute(
            "
            UPDATE generation_jobs
            SET status = 'succeeded',
                response_payload_json = ?2,
                completed_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            ",
            params![job_id, response_payload_json],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to mark generation job {job_id} succeeded"),
            source,
        })?;
    Ok(())
}

pub fn mark_job_failed(
    connection: &Connection,
    job_id: i64,
    error_code: &str,
    error_message: &str,
) -> Result<(), AppError> {
    connection
        .execute(
            "
            UPDATE generation_jobs
            SET status = 'failed',
                error_code = ?2,
                error_message = ?3,
                completed_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            ",
            params![job_id, error_code, error_message],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to mark generation job {job_id} failed"),
            source,
        })?;
    Ok(())
}

pub fn create_alternate_take_job(connection: &Connection, roll_id: i64) -> Result<i64, AppError> {
    connection
        .execute(
            "
            INSERT INTO generation_jobs (
              roll_id,
              job_type,
              status,
              provider_key,
              provider_model,
              request_payload_json
            )
            VALUES (?1, 'alternate_take', 'queued', 'openai', 'gpt-image-1', ?2)
            ",
            params![
                roll_id,
                serde_json::json!({
                    "simulation": "local_placeholder_alternate_take"
                })
                .to_string()
            ],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to create alternate take job for roll {roll_id}"),
            source,
        })?;
    Ok(connection.last_insert_rowid())
}

pub fn mark_job_running_by_id(connection: &Connection, job_id: i64) -> Result<(), AppError> {
    mark_job_running(connection, job_id)
}
