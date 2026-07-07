use rusqlite::{params, Connection};

use crate::dto::frame::{
    ArchiveQueryRequest, ArchiveRollSummary, FrameSummary, RollDetail, RollEventSummary,
};
use crate::errors::AppError;
use crate::persistence::review_repository;

pub fn insert_contact_sheet_frame(
    connection: &Connection,
    roll_id: i64,
    source_job_id: i64,
    frame_index: i64,
    image_path: &str,
    thumbnail_path: &str,
    width: i64,
    height: i64,
    metadata_json: &str,
) -> Result<(), AppError> {
    connection
        .execute(
            "
            INSERT INTO frames (
              roll_id,
              source_job_id,
              parent_frame_id,
              frame_index,
              stage,
              image_path,
              thumbnail_path,
              storage_kind,
              width,
              height,
              metadata_json,
              review_status
            )
            VALUES (?1, ?2, NULL, ?3, 'contact_sheet', ?4, ?5, 'app_managed', ?6, ?7, ?8, 'pending')
            ",
            params![
                roll_id,
                source_job_id,
                frame_index,
                image_path,
                thumbnail_path,
                width,
                height,
                metadata_json
            ],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to insert frame {frame_index} for roll {roll_id}"),
            source,
        })?;

    Ok(())
}

pub fn insert_alternate_take_frame(
    connection: &Connection,
    roll_id: i64,
    source_job_id: i64,
    parent_frame_id: i64,
    image_path: &str,
    thumbnail_path: &str,
    width: i64,
    height: i64,
    metadata_json: &str,
) -> Result<i64, AppError> {
    connection
        .execute(
            "
            INSERT INTO frames (
              roll_id,
              source_job_id,
              parent_frame_id,
              frame_index,
              stage,
              image_path,
              thumbnail_path,
              storage_kind,
              width,
              height,
              metadata_json,
              review_status
            )
            VALUES (?1, ?2, ?3, 0, 'alternate_take', ?4, ?5, 'app_managed', ?6, ?7, ?8, 'pending')
            ",
            params![
                roll_id,
                source_job_id,
                parent_frame_id,
                image_path,
                thumbnail_path,
                width,
                height,
                metadata_json
            ],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to insert alternate take frame for roll {roll_id}"),
            source,
        })?;

    Ok(connection.last_insert_rowid())
}

pub fn update_frame_review_status(connection: &Connection, frame_id: i64, status: &str) -> Result<(), AppError> {
    connection
        .execute(
            "UPDATE frames SET review_status = ?2 WHERE id = ?1",
            params![frame_id, status],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to update review status for frame {frame_id}"),
            source,
        })?;
    Ok(())
}

pub fn frame_exists(connection: &Connection, frame_id: i64, roll_id: i64) -> Result<bool, AppError> {
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM frames WHERE id = ?1 AND roll_id = ?2",
            params![frame_id, roll_id],
            |row| row.get(0),
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to verify frame {frame_id} for roll {roll_id}"),
            source,
        })?;
    Ok(count > 0)
}

pub fn fetch_roll_detail(connection: &Connection, roll_id: i64) -> Result<RollDetail, AppError> {
    let (
        status,
        country_code,
        created_at,
        prompt_engine_version,
        provider_key,
        provider_model,
        contact_sheet_frame_count,
        selected_frame_id,
    ): (String, String, String, String, String, String, i64, Option<i64>) = connection
        .query_row(
            "
            SELECT
              rolls.status,
              countries.code,
              rolls.created_at,
              rolls.prompt_engine_version,
              rolls.provider_key,
              rolls.provider_model,
              rolls.contact_sheet_frame_count,
              rolls.selected_frame_id
            FROM rolls
            INNER JOIN countries ON countries.id = rolls.country_id
            WHERE rolls.id = ?1
            ",
            [roll_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                ))
            },
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch roll detail for {roll_id}"),
            source,
        })?;

    let (job_id, job_status, error_code, error_message): (
        Option<i64>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) = connection
        .query_row(
            "
            SELECT id, status, error_code, error_message
            FROM generation_jobs
            WHERE roll_id = ?1 AND job_type = 'contact_sheet'
            ORDER BY id DESC
            LIMIT 1
            ",
            [roll_id],
            |row| Ok((Some(row.get(0)?), Some(row.get(1)?), row.get(2)?, row.get(3)?)),
        )
        .unwrap_or((None, None, None, None));

    let mut stmt = connection
        .prepare(
            "
            SELECT
              frames.id,
              frames.frame_index,
              frames.stage,
              frames.image_path,
              frames.thumbnail_path,
              frames.review_status,
              frames.created_at,
              CASE WHEN favorites.id IS NULL THEN 0 ELSE 1 END AS is_favorite
            FROM frames
            LEFT JOIN favorites ON favorites.frame_id = frames.id
            WHERE roll_id = ?1
            ORDER BY frame_index ASC, id ASC
            ",
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to prepare frame query for roll {roll_id}"),
            source,
        })?;

    let rows = stmt
        .query_map([roll_id], |row| {
            Ok(FrameSummary {
                id: row.get(0)?,
                frame_index: row.get(1)?,
                stage: row.get(2)?,
                image_path: row.get(3)?,
                thumbnail_path: row.get(4)?,
                review_status: row.get(5)?,
                created_at: row.get(6)?,
                is_favorite: row.get::<_, i64>(7)? > 0,
            })
        })
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch frames for roll {roll_id}"),
            source,
        })?;

    let mut frames = Vec::new();
    let mut alternate_take_frame_id = None;
    for row in rows {
        let frame = row.map_err(|source| AppError::Sqlite {
            context: format!("failed to decode frame row for roll {roll_id}"),
            source,
        })?;

        if frame.stage == "alternate_take" {
            alternate_take_frame_id = Some(frame.id);
        }

        frames.push(frame);
    }

    let mut events_stmt = connection
        .prepare(
            "
            SELECT id, event_type, payload_json, created_at
            FROM roll_events
            WHERE roll_id = ?1
            ORDER BY id DESC
            LIMIT 24
            ",
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to prepare roll events query for roll {roll_id}"),
            source,
        })?;

    let event_rows = events_stmt
        .query_map([roll_id], |row| {
            Ok(RollEventSummary {
                id: row.get(0)?,
                event_type: row.get(1)?,
                payload_json: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch roll events for roll {roll_id}"),
            source,
        })?;

    let mut events = Vec::new();
    for row in event_rows {
        events.push(row.map_err(|source| AppError::Sqlite {
            context: format!("failed to decode roll event row for roll {roll_id}"),
            source,
        })?);
    }

    let latest_review = review_repository::latest_review_for_roll(connection, roll_id)?;

    Ok(RollDetail {
        roll_id,
        status,
        country_code,
        created_at,
        prompt_engine_version,
        provider_key,
        provider_model,
        contact_sheet_frame_count,
        generation_job_id: job_id,
        generation_job_status: job_status,
        generation_error_code: error_code,
        generation_error_message: error_message,
        selected_frame_id,
        alternate_take_frame_id,
        latest_review,
        frames,
        events,
    })
}

pub fn list_recent_rolls(
    connection: &Connection,
    request: &ArchiveQueryRequest,
) -> Result<Vec<ArchiveRollSummary>, AppError> {
    let limit = request.limit.unwrap_or(24).clamp(1, 200);
    let normalized_query = request
        .query
        .as_ref()
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty());
    let normalized_status = request
        .status
        .as_ref()
        .map(|value| value.trim().to_lowercase())
        .filter(|value| !value.is_empty() && value != "all");
    let sort = request.sort.as_deref().unwrap_or("newest");
    let order_by = match sort {
        "oldest" => "rolls.created_at ASC, rolls.id ASC",
        "favorites" => "favorite_count DESC, rolls.created_at DESC, rolls.id DESC",
        _ => "rolls.created_at DESC, rolls.id DESC",
    };

    let query_sql = format!(
        "
        SELECT
          rolls.id,
          rolls.status,
          countries.code,
          rolls.created_at,
          rolls.selected_frame_id,
          (
            SELECT id FROM frames
            WHERE roll_id = rolls.id AND stage = 'alternate_take'
            ORDER BY id DESC
            LIMIT 1
          ) AS alternate_take_frame_id,
          (
            SELECT image_path FROM frames
            WHERE roll_id = rolls.id
            ORDER BY
              CASE
                WHEN stage = 'alternate_take' THEN 0
                WHEN stage = 'contact_sheet' THEN 1
                ELSE 2
              END,
              frame_index ASC
            LIMIT 1
          ) AS preview_image_path,
          (
            SELECT COUNT(*)
            FROM favorites
            INNER JOIN frames ON frames.id = favorites.frame_id
            WHERE frames.roll_id = rolls.id
          ) AS favorite_count
        FROM rolls
        INNER JOIN countries ON countries.id = rolls.country_id
        WHERE (?1 IS NULL OR rolls.status = ?1)
          AND (
            ?2 IS NULL
            OR lower(countries.code) LIKE ?3
            OR CAST(rolls.id AS TEXT) LIKE ?3
            OR EXISTS (
              SELECT 1
              FROM frames
              WHERE frames.roll_id = rolls.id
                AND lower(frames.image_path) LIKE ?3
            )
          )
        ORDER BY {order_by}
        LIMIT ?4
        "
    );

    let mut stmt = connection
        .prepare(&query_sql)
        .map_err(|source| AppError::Sqlite {
            context: "failed to prepare recent rolls query".to_string(),
            source,
        })?;

    let query_like = normalized_query.as_ref().map(|value| format!("%{value}%"));
    let rows = stmt
        .query_map(
            rusqlite::params![normalized_status, normalized_query, query_like, limit],
            |row| {
            Ok(ArchiveRollSummary {
                roll_id: row.get(0)?,
                status: row.get(1)?,
                country_code: row.get(2)?,
                created_at: row.get(3)?,
                selected_frame_id: row.get(4)?,
                alternate_take_frame_id: row.get(5)?,
                preview_image_path: row.get(6)?,
                favorite_count: row.get(7)?,
            })
        },
        )
        .map_err(|source| AppError::Sqlite {
            context: "failed to query recent rolls".to_string(),
            source,
        })?;

    let mut archive = Vec::new();
    for row in rows {
        let item = row.map_err(|source| AppError::Sqlite {
            context: "failed to decode recent roll row".to_string(),
            source,
        })?;
        archive.push(item);
    }

    Ok(archive)
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::list_recent_rolls;
    use crate::dto::frame::ArchiveQueryRequest;
    use crate::persistence::migrations;

    fn setup_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("in-memory sqlite");
        migrations::apply_all(&connection).expect("apply migrations");
        connection
            .execute(
                "
                INSERT INTO countries (id, code, display_name, is_default, is_featured, status)
                VALUES
                  (1, 'jp', 'Japan', 1, 1, 'active'),
                  (2, 'us', 'United States', 0, 1, 'active')
                ",
                [],
            )
            .expect("insert countries");
        connection
    }

    fn insert_roll(connection: &Connection, id: i64, country_id: i64, status: &str, created_at: &str) {
        connection
            .execute(
                "
                INSERT INTO rolls (
                  id, country_id, status, roll_dna_version, roll_dna_json, input_snapshot_json,
                  dictionary_bundle_id, prompt_engine_version, provider_key, provider_model,
                  contact_sheet_frame_count, selected_frame_id, created_at, updated_at
                )
                VALUES (?1, ?2, ?3, 'v1', '{}', '{}', NULL, 'snapshot-v25', 'openai', 'gpt-image-1', 8, NULL, ?4, ?4)
                ",
                rusqlite::params![id, country_id, status, created_at],
            )
            .expect("insert roll");
    }

    fn insert_frame(connection: &Connection, id: i64, roll_id: i64, frame_index: i64, stage: &str, image_path: &str) {
        connection
            .execute(
                "
                INSERT INTO frames (
                  id, roll_id, source_job_id, parent_frame_id, frame_index, stage, image_path,
                  thumbnail_path, storage_kind, width, height, metadata_json, review_status, created_at
                )
                VALUES (?1, ?2, NULL, NULL, ?3, ?4, ?5, NULL, 'app_managed', 1000, 1000, '{}', 'pending', CURRENT_TIMESTAMP)
                ",
                rusqlite::params![id, roll_id, frame_index, stage, image_path],
            )
            .expect("insert frame");
    }

    fn insert_favorite(connection: &Connection, frame_id: i64) {
        connection
            .execute(
                "INSERT INTO favorites (frame_id, notes) VALUES (?1, NULL)",
                [frame_id],
            )
            .expect("insert favorite");
    }

    #[test]
    fn recent_rolls_can_filter_by_status_and_query() {
        let connection = setup_connection();
        insert_roll(&connection, 1, 1, "completed", "2026-07-01T10:00:00Z");
        insert_roll(&connection, 2, 2, "failed", "2026-07-02T10:00:00Z");
        insert_roll(&connection, 3, 1, "contact_sheet_ready", "2026-07-03T10:00:00Z");
        insert_frame(&connection, 11, 1, 0, "contact_sheet", "/images/jp-alley.png");
        insert_frame(&connection, 21, 2, 0, "contact_sheet", "/images/us-train.png");
        insert_frame(&connection, 31, 3, 0, "contact_sheet", "/images/jp-river.png");

        let failed = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: None,
                status: Some("failed".to_string()),
                sort: Some("newest".to_string()),
                limit: Some(24),
            },
        )
        .expect("query failed status");
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].roll_id, 2);

        let jp_only = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: Some("jp".to_string()),
                status: None,
                sort: Some("newest".to_string()),
                limit: Some(24),
            },
        )
        .expect("query jp");
        assert_eq!(jp_only.len(), 2);
        assert_eq!(jp_only[0].roll_id, 3);
        assert_eq!(jp_only[1].roll_id, 1);

        let image_match = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: Some("train".to_string()),
                status: None,
                sort: Some("newest".to_string()),
                limit: Some(24),
            },
        )
        .expect("query image path");
        assert_eq!(image_match.len(), 1);
        assert_eq!(image_match[0].roll_id, 2);
    }

    #[test]
    fn recent_rolls_can_sort_by_favorites_then_recency() {
        let connection = setup_connection();
        insert_roll(&connection, 1, 1, "completed", "2026-07-01T10:00:00Z");
        insert_roll(&connection, 2, 2, "completed", "2026-07-02T10:00:00Z");
        insert_roll(&connection, 3, 1, "completed", "2026-07-03T10:00:00Z");
        insert_frame(&connection, 11, 1, 0, "contact_sheet", "/images/one.png");
        insert_frame(&connection, 12, 1, 1, "contact_sheet", "/images/one-b.png");
        insert_frame(&connection, 21, 2, 0, "contact_sheet", "/images/two.png");
        insert_frame(&connection, 31, 3, 0, "contact_sheet", "/images/three.png");
        insert_favorite(&connection, 11);
        insert_favorite(&connection, 12);
        insert_favorite(&connection, 31);

        let archive = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: None,
                status: None,
                sort: Some("favorites".to_string()),
                limit: Some(24),
            },
        )
        .expect("query favorites sort");

        assert_eq!(archive.len(), 3);
        assert_eq!(archive[0].roll_id, 1);
        assert_eq!(archive[0].favorite_count, 2);
        assert_eq!(archive[1].roll_id, 3);
        assert_eq!(archive[1].favorite_count, 1);
        assert_eq!(archive[2].roll_id, 2);
        assert_eq!(archive[2].favorite_count, 0);
    }

    #[test]
    fn recent_rolls_use_default_limit_and_clamp_requested_limit() {
        let connection = setup_connection();
        for roll_id in 1..=30 {
            insert_roll(
                &connection,
                roll_id,
                if roll_id % 2 == 0 { 1 } else { 2 },
                "completed",
                &format!("2026-07-{roll_id:02}T10:00:00Z"),
            );
            insert_frame(
                &connection,
                100 + roll_id,
                roll_id,
                0,
                "contact_sheet",
                &format!("/images/{roll_id}.png"),
            );
        }

        let default_limited = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: None,
                status: None,
                sort: Some("newest".to_string()),
                limit: None,
            },
        )
        .expect("query default limit");
        assert_eq!(default_limited.len(), 24);
        assert_eq!(default_limited[0].roll_id, 30);
        assert_eq!(default_limited[23].roll_id, 7);

        let min_clamped = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: None,
                status: None,
                sort: Some("newest".to_string()),
                limit: Some(0),
            },
        )
        .expect("query min-clamped limit");
        assert_eq!(min_clamped.len(), 1);
        assert_eq!(min_clamped[0].roll_id, 30);

        let max_clamped = list_recent_rolls(
            &connection,
            &ArchiveQueryRequest {
                query: None,
                status: None,
                sort: Some("newest".to_string()),
                limit: Some(999),
            },
        )
        .expect("query max-clamped limit");
        assert_eq!(max_clamped.len(), 30);
        assert_eq!(max_clamped[0].roll_id, 30);
        assert_eq!(max_clamped[29].roll_id, 1);
    }
}
