use rusqlite::{params, Connection, OptionalExtension};

use crate::dto::roll::CreateRollRequest;
use crate::dto::setup::PresetSummary;
use crate::errors::AppError;

pub fn save_preset(
    connection: &Connection,
    name: &str,
    country_id: i64,
    request: &CreateRollRequest,
    is_locked_random_template: bool,
) -> Result<i64, AppError> {
    let normalized_name = name.trim();
    if normalized_name.is_empty() {
        return Err(AppError::Config {
            context: "preset name cannot be empty".to_string(),
        });
    }

    let input_snapshot_json = serde_json::to_string(request).map_err(|source| AppError::Json {
        context: "failed to serialize preset input snapshot".to_string(),
        source,
    })?;

    let existing_id: Option<i64> = connection
        .query_row(
            "SELECT id FROM presets WHERE name = ?1 LIMIT 1",
            [normalized_name],
            |row| row.get(0),
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to check existing preset {normalized_name}"),
            source,
        })?;

    match existing_id {
        Some(preset_id) => {
            connection
                .execute(
                    "
                    UPDATE presets
                    SET country_id = ?2,
                        input_snapshot_json = ?3,
                        is_locked_random_template = ?4,
                        updated_at = CURRENT_TIMESTAMP
                    WHERE id = ?1
                    ",
                    params![
                        preset_id,
                        country_id,
                        input_snapshot_json,
                        if is_locked_random_template { 1 } else { 0 }
                    ],
                )
                .map_err(|source| AppError::Sqlite {
                    context: format!("failed to update preset {normalized_name}"),
                    source,
                })?;
            Ok(preset_id)
        }
        None => {
            connection
                .execute(
                    "
                    INSERT INTO presets (
                      name,
                      country_id,
                      input_snapshot_json,
                      is_locked_random_template
                    )
                    VALUES (?1, ?2, ?3, ?4)
                    ",
                    params![
                        normalized_name,
                        country_id,
                        input_snapshot_json,
                        if is_locked_random_template { 1 } else { 0 }
                    ],
                )
                .map_err(|source| AppError::Sqlite {
                    context: format!("failed to save preset {normalized_name}"),
                    source,
                })?;

            Ok(connection.last_insert_rowid())
        }
    }
}

pub fn list_presets(connection: &Connection) -> Result<Vec<PresetSummary>, AppError> {
    let mut stmt = connection
        .prepare(
            "
            SELECT
              presets.id,
              presets.name,
              countries.code,
              presets.input_snapshot_json,
              presets.is_locked_random_template,
              presets.created_at,
              presets.updated_at
            FROM presets
            INNER JOIN countries ON countries.id = presets.country_id
            ORDER BY presets.updated_at DESC, presets.id DESC
            ",
        )
        .map_err(|source| AppError::Sqlite {
            context: "failed to prepare preset list query".to_string(),
            source,
        })?;

    let rows = stmt
        .query_map([], |row| {
            let input_snapshot_json: String = row.get(3)?;
            let input_snapshot = serde_json::from_str::<CreateRollRequest>(&input_snapshot_json)
                .map_err(|error| {
                    rusqlite::Error::FromSqlConversionFailure(
                        3,
                        rusqlite::types::Type::Text,
                        Box::new(error),
                    )
                })?;

            Ok(PresetSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                country_code: row.get(2)?,
                input_snapshot,
                is_locked_random_template: row.get::<_, i64>(4)? == 1,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|source| AppError::Sqlite {
            context: "failed to query presets".to_string(),
            source,
        })?;

    let mut presets = Vec::new();
    for row in rows {
        presets.push(row.map_err(|source| AppError::Sqlite {
            context: "failed to decode preset row".to_string(),
            source,
        })?);
    }

    Ok(presets)
}

pub fn delete_preset(connection: &Connection, preset_id: i64) -> Result<(), AppError> {
    connection
        .execute("DELETE FROM presets WHERE id = ?1", [preset_id])
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to delete preset {preset_id}"),
            source,
        })?;

    Ok(())
}

pub fn rename_preset(connection: &Connection, preset_id: i64, name: &str) -> Result<(), AppError> {
    let normalized_name = name.trim();
    if normalized_name.is_empty() {
        return Err(AppError::Config {
            context: "preset name cannot be empty".to_string(),
        });
    }

    let duplicate_id: Option<i64> = connection
        .query_row(
            "SELECT id FROM presets WHERE name = ?1 AND id != ?2 LIMIT 1",
            params![normalized_name, preset_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to check duplicate preset name {normalized_name}"),
            source,
        })?;

    if duplicate_id.is_some() {
        return Err(AppError::Config {
            context: format!("preset name {normalized_name} is already in use"),
        });
    }

    connection
        .execute(
            "
            UPDATE presets
            SET name = ?2,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?1
            ",
            params![preset_id, normalized_name],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to rename preset {preset_id}"),
            source,
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::{list_presets, rename_preset, save_preset};
    use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};
    use crate::persistence::migrations;

    fn field(mode: InputMode, value: Option<&str>) -> SetupInputField {
        SetupInputField {
            mode,
            value: value.map(|item| item.to_string()),
        }
    }

    fn request(place: &str) -> CreateRollRequest {
        CreateRollRequest {
            country: field(InputMode::Manual, Some("jp")),
            moment: field(InputMode::Random, None),
            place: field(InputMode::Manual, Some(place)),
            time: field(InputMode::Random, None),
            season: field(InputMode::Random, None),
            weather: field(InputMode::Random, None),
            tiny_detail: field(InputMode::Random, None),
        }
    }

    fn setup_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("in-memory sqlite");
        migrations::apply_all(&connection).expect("apply migrations");
        connection
            .execute(
                "
                INSERT INTO countries (code, display_name, is_default, is_featured, status)
                VALUES ('jp', 'Japan', 1, 1, 'active')
                ",
                [],
            )
            .expect("insert country");
        connection
    }

    #[test]
    fn save_preset_overwrites_existing_name() {
        let connection = setup_connection();

        let first_id = save_preset(
            &connection,
            "Night Walk",
            1,
            &request("station platform"),
            false,
        )
        .expect("save first preset");
        let second_id = save_preset(&connection, "Night Walk", 1, &request("quiet street"), true)
            .expect("overwrite preset");

        let presets = list_presets(&connection).expect("list presets");
        assert_eq!(first_id, second_id);
        assert_eq!(presets.len(), 1);
        assert_eq!(presets[0].name, "Night Walk");
        assert_eq!(
            presets[0]
                .input_snapshot
                .place
                .value
                .as_deref()
                .expect("place value"),
            "quiet street"
        );
        assert!(presets[0].is_locked_random_template);
    }

    #[test]
    fn rename_preset_changes_name_without_creating_duplicate() {
        let connection = setup_connection();
        let preset_id = save_preset(
            &connection,
            "Night Walk",
            1,
            &request("station platform"),
            false,
        )
        .expect("save preset");

        rename_preset(&connection, preset_id, "Night Train").expect("rename preset");

        let presets = list_presets(&connection).expect("list presets");
        assert_eq!(presets.len(), 1);
        assert_eq!(presets[0].name, "Night Train");
    }
}
