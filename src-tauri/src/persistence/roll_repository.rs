use rusqlite::{params, Connection};
use serde_json::Value;

use crate::domain::roll_dna::{self, ResolvedSetupValues};
use crate::dto::roll::CreateRollRequest;
use crate::errors::AppError;
const DEFAULT_PROVIDER_KEY: &str = "openai";
use crate::prompt_engine;

const DEFAULT_PROVIDER_MODEL: &str = "gpt-image-1";
const DEFAULT_ROLL_DNA_VERSION: &str = "v1";

pub struct CreatedRollRecord {
    pub roll_id: i64,
    pub status: String,
    pub country_code: String,
    pub prompt_engine_version: String,
    pub provider_key: String,
    pub provider_model: String,
    pub contact_sheet_frame_count: i64,
    pub created_at: String,
    pub generation_job_id: i64,
    pub generation_job_status: String,
}

pub struct RollGenerationContext {
    pub roll_dna: Value,
    pub prompt_engine_version: String,
}

pub fn update_roll_status(
    connection: &Connection,
    roll_id: i64,
    status: &str,
) -> Result<(), AppError> {
    connection
        .execute(
            "
            UPDATE rolls
            SET status = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            ",
            params![roll_id, status],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to update roll {roll_id} status to {status}"),
            source,
        })?;

    Ok(())
}

pub fn select_frame(connection: &Connection, roll_id: i64, frame_id: i64) -> Result<(), AppError> {
    connection
        .execute(
            "
            UPDATE rolls
            SET selected_frame_id = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            ",
            params![roll_id, frame_id],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to set selected frame {frame_id} for roll {roll_id}"),
            source,
        })?;
    Ok(())
}

pub fn create_roll(
    connection: &mut Connection,
    country_id: i64,
    country_code: &str,
    request: &CreateRollRequest,
    resolved_values: &ResolvedSetupValues,
) -> Result<CreatedRollRecord, AppError> {
    let status = "queued";
    let roll_dna_json = roll_dna::resolve_roll_dna(request, country_code, resolved_values);

    let input_snapshot_json = serde_json::to_value(request).map_err(|source| AppError::Json {
        context: "failed to serialize roll input snapshot".to_string(),
        source,
    })?;

    let roll_dna_json_string =
        serde_json::to_string(&roll_dna_json).map_err(|source| AppError::Json {
            context: "failed to serialize roll dna".to_string(),
            source,
        })?;
    let input_snapshot_json_string =
        serde_json::to_string(&input_snapshot_json).map_err(|source| AppError::Json {
            context: "failed to serialize roll input snapshot string".to_string(),
            source,
        })?;

    let tx = connection
        .transaction()
        .map_err(|source| AppError::Sqlite {
            context: "failed to begin roll creation transaction".to_string(),
            source,
        })?;

    tx.execute(
        "
        INSERT INTO rolls (
          country_id,
          status,
          roll_dna_version,
          roll_dna_json,
          input_snapshot_json,
          dictionary_bundle_id,
          prompt_engine_version,
          provider_key,
          provider_model,
          contact_sheet_frame_count
        )
        VALUES (?1, ?2, ?3, ?4, ?5,
          (SELECT id FROM dictionary_bundles WHERE version = 'v1-initial' LIMIT 1),
          ?6, ?7, ?8, 8
        )
        ",
        params![
            country_id,
            status,
            DEFAULT_ROLL_DNA_VERSION,
            roll_dna_json_string,
            input_snapshot_json_string,
            prompt_engine::prompt_engine_version(),
            DEFAULT_PROVIDER_KEY,
            DEFAULT_PROVIDER_MODEL
        ],
    )
    .map_err(|source| AppError::Sqlite {
        context: "failed to insert roll".to_string(),
        source,
    })?;

    let roll_id = tx.last_insert_rowid();

    tx.execute(
        "
        INSERT INTO generation_jobs (
          roll_id,
          job_type,
          status,
          provider_key,
          provider_model,
          request_payload_json
        )
        VALUES (?1, 'contact_sheet', 'queued', ?2, ?3, ?4)
        ",
        params![
            roll_id,
            DEFAULT_PROVIDER_KEY,
            DEFAULT_PROVIDER_MODEL,
            roll_dna_json_string
        ],
    )
    .map_err(|source| AppError::Sqlite {
        context: "failed to create contact sheet generation job".to_string(),
        source,
    })?;

    let generation_job_id = tx.last_insert_rowid();

    tx.execute(
        "
        INSERT INTO roll_events (roll_id, event_type, payload_json)
        VALUES (?1, 'roll_created', ?2)
        ",
        params![roll_id, input_snapshot_json_string],
    )
    .map_err(|source| AppError::Sqlite {
        context: "failed to insert roll event".to_string(),
        source,
    })?;

    tx.execute(
        "
        INSERT INTO roll_events (roll_id, event_type, payload_json)
        VALUES (?1, 'contact_sheet_queued', ?2)
        ",
        params![
            roll_id,
            serde_json::json!({
                "generation_job_id": generation_job_id,
                "status": "queued"
            })
            .to_string()
        ],
    )
    .map_err(|source| AppError::Sqlite {
        context: "failed to insert contact sheet queued event".to_string(),
        source,
    })?;

    let created_at: String = tx
        .query_row(
            "SELECT created_at FROM rolls WHERE id = ?1",
            [roll_id],
            |row| row.get(0),
        )
        .map_err(|source| AppError::Sqlite {
            context: "failed to fetch created roll timestamp".to_string(),
            source,
        })?;

    tx.commit().map_err(|source| AppError::Sqlite {
        context: "failed to commit roll creation transaction".to_string(),
        source,
    })?;

    Ok(CreatedRollRecord {
        roll_id,
        status: status.to_string(),
        country_code: country_code.to_string(),
        prompt_engine_version: prompt_engine::prompt_engine_version().to_string(),
        provider_key: DEFAULT_PROVIDER_KEY.to_string(),
        provider_model: DEFAULT_PROVIDER_MODEL.to_string(),
        contact_sheet_frame_count: 8,
        created_at,
        generation_job_id,
        generation_job_status: "queued".to_string(),
    })
}

pub fn fetch_roll_generation_context(
    connection: &Connection,
    roll_id: i64,
) -> Result<RollGenerationContext, AppError> {
    let (roll_dna_json, prompt_engine_version): (String, String) = connection
        .query_row(
            "
            SELECT roll_dna_json, prompt_engine_version
            FROM rolls
            WHERE id = ?1
            ",
            [roll_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch generation context for roll {roll_id}"),
            source,
        })?;

    let roll_dna = serde_json::from_str(&roll_dna_json).map_err(|source| AppError::Json {
        context: format!("failed to parse roll_dna_json for roll {roll_id}"),
        source,
    })?;

    Ok(RollGenerationContext {
        roll_dna,
        prompt_engine_version,
    })
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;
    use serde_json::Value;

    use super::{create_roll, fetch_roll_generation_context};
    use crate::domain::roll_dna::ResolvedSetupValues;
    use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};
    use crate::persistence::migrations;
    use crate::prompt_engine;

    fn field(mode: InputMode, value: Option<&str>) -> SetupInputField {
        SetupInputField {
            mode,
            value: value.map(|item| item.to_string()),
        }
    }

    fn request() -> CreateRollRequest {
        CreateRollRequest {
            country: field(InputMode::Manual, Some("jp")),
            moment: field(InputMode::Random, None),
            place: field(InputMode::Manual, Some("covered shopping street")),
            time: field(InputMode::LockedRandom, None),
            season: field(InputMode::Random, None),
            weather: field(InputMode::Random, None),
            tiny_detail: field(
                InputMode::Manual,
                Some("clear umbrella dripping near the door"),
            ),
        }
    }

    fn resolved_values() -> ResolvedSetupValues {
        ResolvedSetupValues {
            moment: "stopping by the convenience store before going home".to_string(),
            place: "covered shopping street".to_string(),
            time: "evening".to_string(),
            season: "autumn".to_string(),
            weather: "drizzle".to_string(),
            tiny_detail: "clear umbrella dripping near the door".to_string(),
        }
    }

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
                INSERT INTO dictionary_bundles (id, version, country_scope, notes)
                VALUES (1, 'v1-initial', '[\"jp\"]', 'test bundle')
                ",
                [],
            )
            .expect("insert bundle");
        connection
    }

    #[test]
    fn create_roll_persists_roll_dna_job_and_initial_events() {
        let mut connection = setup_connection();
        let current = request();
        let resolved = resolved_values();

        let created =
            create_roll(&mut connection, 1, "jp", &current, &resolved).expect("create roll");

        assert_eq!(created.status, "queued");
        assert_eq!(created.country_code, "jp");
        assert_eq!(created.provider_key, "openai");
        assert_eq!(created.provider_model, "gpt-image-1");
        assert_eq!(created.contact_sheet_frame_count, 8);
        assert_eq!(created.generation_job_status, "queued");
        assert_eq!(
            created.prompt_engine_version,
            prompt_engine::prompt_engine_version()
        );

        let (status, dna_version, provider_key, provider_model, frame_count, selected_frame_id): (
            String,
            String,
            String,
            String,
            i64,
            Option<i64>,
        ) = connection
            .query_row(
                "
                SELECT status, roll_dna_version, provider_key, provider_model, contact_sheet_frame_count, selected_frame_id
                FROM rolls
                WHERE id = ?1
                ",
                [created.roll_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                    ))
                },
            )
            .expect("query roll");
        assert_eq!(status, "queued");
        assert_eq!(dna_version, "v1");
        assert_eq!(provider_key, "openai");
        assert_eq!(provider_model, "gpt-image-1");
        assert_eq!(frame_count, 8);
        assert_eq!(selected_frame_id, None);

        let context = fetch_roll_generation_context(&connection, created.roll_id)
            .expect("fetch generation context");
        assert_eq!(
            context.prompt_engine_version,
            prompt_engine::prompt_engine_version()
        );
        assert_eq!(
            context
                .roll_dna
                .get("country_context")
                .and_then(|node| node.get("code"))
                .and_then(Value::as_str),
            Some("jp")
        );
        assert_eq!(
            context
                .roll_dna
                .get("moment_context")
                .and_then(|node| node.get("resolved_value"))
                .and_then(Value::as_str),
            Some("stopping by the convenience store before going home")
        );
        assert_eq!(
            context
                .roll_dna
                .get("time_context")
                .and_then(|node| node.get("source_mode"))
                .and_then(Value::as_str),
            Some("locked_random")
        );

        let (job_status, job_type, request_payload_json): (String, String, String) = connection
            .query_row(
                "
                SELECT status, job_type, request_payload_json
                FROM generation_jobs
                WHERE id = ?1
                ",
                [created.generation_job_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .expect("query generation job");
        assert_eq!(job_status, "queued");
        assert_eq!(job_type, "contact_sheet");
        let request_payload: Value =
            serde_json::from_str(&request_payload_json).expect("parse request payload json");
        assert_eq!(
            request_payload
                .get("place_context")
                .and_then(|node| node.get("resolved_value"))
                .and_then(Value::as_str),
            Some("covered shopping street")
        );

        let event_rows: Vec<(String, String)> = {
            let mut stmt = connection
                .prepare(
                    "
                    SELECT event_type, payload_json
                    FROM roll_events
                    WHERE roll_id = ?1
                    ORDER BY id ASC
                    ",
                )
                .expect("prepare roll events query");
            let rows = stmt
                .query_map([created.roll_id], |row| Ok((row.get(0)?, row.get(1)?)))
                .expect("query roll events");
            rows.map(|row| row.expect("decode roll event row"))
                .collect()
        };
        assert_eq!(event_rows.len(), 2);
        assert_eq!(event_rows[0].0, "roll_created");
        assert_eq!(event_rows[1].0, "contact_sheet_queued");

        let created_payload: Value =
            serde_json::from_str(&event_rows[0].1).expect("parse roll_created payload");
        assert_eq!(
            created_payload
                .get("place")
                .and_then(|node| node.get("value"))
                .and_then(Value::as_str),
            Some("covered shopping street")
        );

        let queued_payload: Value =
            serde_json::from_str(&event_rows[1].1).expect("parse queued payload");
        assert_eq!(
            queued_payload
                .get("generation_job_id")
                .and_then(Value::as_i64),
            Some(created.generation_job_id)
        );
    }
}
