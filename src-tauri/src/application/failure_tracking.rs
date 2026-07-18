use rusqlite::Connection;
use serde_json::{json, Value};

use crate::errors::AppError;
use crate::persistence::{generation_job_repository, provider_health_repository, roll_repository};

pub fn persist_generation_failure(
    connection: &Connection,
    roll_id: i64,
    job_id: i64,
    error: &AppError,
    event_type: &str,
    extra_payload: Value,
) -> Result<(), AppError> {
    let error_code = error.code();
    let error_message = error.to_string();

    generation_job_repository::mark_job_failed(connection, job_id, &error_code, &error_message)?;
    roll_repository::update_roll_status(connection, roll_id, "failed")?;

    if error_code.starts_with("provider_") {
        provider_health_repository::upsert_provider_health(
            connection,
            "openai",
            "degraded",
            Some(&error_message),
            Some(&chrono::Utc::now().to_rfc3339()),
        )?;
    }

    let mut payload = match extra_payload {
        Value::Object(map) => map,
        _ => serde_json::Map::new(),
    };
    payload.insert("error_code".to_string(), json!(error_code));
    payload.insert("error_message".to_string(), json!(error_message));

    connection
        .execute(
            "
            INSERT INTO roll_events (roll_id, event_type, payload_json)
            VALUES (?1, ?2, ?3)
            ",
            rusqlite::params![roll_id, event_type, Value::Object(payload).to_string()],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to insert {event_type} event for roll {roll_id}"),
            source,
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use serde_json::Value;

    use super::persist_generation_failure;
    use crate::errors::AppError;
    use crate::persistence::{migrations, provider_health_repository};

    fn setup_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("in-memory sqlite");
        migrations::apply_all(&connection).expect("apply migrations");
        connection
            .execute(
                "
                INSERT INTO countries (id, code, display_name, is_default, is_featured, status)
                VALUES (1, 'jp', 'Japan', 1, 1, 'active')
                ",
                [],
            )
            .expect("insert country");
        connection
            .execute(
                "
                INSERT INTO rolls (
                  id, country_id, status, roll_dna_version, roll_dna_json, input_snapshot_json,
                  dictionary_bundle_id, prompt_engine_version, provider_key, provider_model,
                  contact_sheet_frame_count, selected_frame_id
                )
                VALUES (1, 1, 'queued', 'v1', '{}', '{}', NULL, 'snapshot-v25', 'openai', 'gpt-image-1', 8, NULL)
                ",
                [],
            )
            .expect("insert roll");
        connection
            .execute(
                "
                INSERT INTO generation_jobs (
                  id, roll_id, job_type, status, provider_key, provider_model, request_payload_json
                )
                VALUES (1, 1, 'contact_sheet', 'running', 'openai', 'gpt-image-1', '{}')
                ",
                [],
            )
            .expect("insert job");
        connection
    }

    #[test]
    fn provider_failures_degrade_provider_health_and_record_event() {
        let connection = setup_connection();
        let error = AppError::Provider {
            code: "provider_timeout".to_string(),
            context: "provider timeout while generating".to_string(),
        };

        persist_generation_failure(
            &connection,
            1,
            1,
            &error,
            "contact_sheet_failed",
            serde_json::json!({ "generation_job_id": 1 }),
        )
        .expect("persist provider failure");

        let provider_health =
            provider_health_repository::fetch_provider_health(&connection, "openai")
                .expect("fetch provider health")
                .expect("provider health exists");
        assert_eq!(provider_health.status, "degraded");
        assert_eq!(
            provider_health.last_check_message.as_deref(),
            Some("provider timeout while generating")
        );

        let roll_status: String = connection
            .query_row("SELECT status FROM rolls WHERE id = 1", [], |row| {
                row.get(0)
            })
            .expect("query roll status");
        assert_eq!(roll_status, "failed");

        let job_status: String = connection
            .query_row(
                "SELECT status FROM generation_jobs WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .expect("query job status");
        assert_eq!(job_status, "failed");

        let payload_json: String = connection
            .query_row(
                "SELECT payload_json FROM roll_events WHERE roll_id = 1 AND event_type = 'contact_sheet_failed'",
                [],
                |row| row.get(0),
            )
            .expect("query failure payload");
        let payload: Value = serde_json::from_str(&payload_json).expect("parse payload");
        assert_eq!(
            payload.get("error_code").and_then(Value::as_str),
            Some("provider_timeout")
        );
    }

    #[test]
    fn non_provider_failures_do_not_create_provider_health_record() {
        let connection = setup_connection();
        let error = AppError::Config {
            context: "frame did not belong to roll".to_string(),
        };

        persist_generation_failure(
            &connection,
            1,
            1,
            &error,
            "alternate_take_failed",
            serde_json::json!({ "frame_id": 9 }),
        )
        .expect("persist config failure");

        let provider_health =
            provider_health_repository::fetch_provider_health(&connection, "openai")
                .expect("fetch provider health");
        assert!(provider_health.is_none());

        let payload_json: String = connection
            .query_row(
                "SELECT payload_json FROM roll_events WHERE roll_id = 1 AND event_type = 'alternate_take_failed'",
                [],
                |row| row.get(0),
            )
            .expect("query failure payload");
        let payload: Value = serde_json::from_str(&payload_json).expect("parse payload");
        assert_eq!(
            payload.get("error_code").and_then(Value::as_str),
            Some("config_error")
        );
    }
}
