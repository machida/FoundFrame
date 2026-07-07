use rusqlite::{params, Connection, OptionalExtension};
use serde_json::Value;

use crate::dto::frame::ReviewSummary;
use crate::dto::roll::CreateRollRequest;
use crate::errors::AppError;
use crate::review;

fn decode_review_summary(
    frame_id: i64,
    review_engine_version: String,
    evaluator_type: String,
    scores_json: String,
    summary_json: Option<String>,
) -> Result<ReviewSummary, AppError> {
    let scores: Value = serde_json::from_str(&scores_json).map_err(|source| AppError::Json {
        context: format!("failed to parse review scores for frame {frame_id}"),
        source,
    })?;
    let summary_value = match summary_json {
        Some(json) => serde_json::from_str::<Value>(&json).map_err(|source| AppError::Json {
            context: format!("failed to parse review summary for frame {frame_id}"),
            source,
        })?,
        None => Value::Null,
    };

    Ok(ReviewSummary {
        frame_id,
        review_engine_version,
        evaluator_type,
        overall_score: scores.get("overall").and_then(Value::as_f64).unwrap_or(0.0),
        ai_feeling: scores.get("ai_feeling").and_then(Value::as_f64).unwrap_or(0.0),
        accidental_feeling: scores
            .get("accidental_feeling")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        everyday_life: scores.get("everyday_life").and_then(Value::as_f64).unwrap_or(0.0),
        memory_quality: scores
            .get("memory_quality")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        imperfection: scores.get("imperfection").and_then(Value::as_f64).unwrap_or(0.0),
        composition_balance: scores
            .get("composition_balance")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        summary: summary_value
            .get("summary")
            .and_then(Value::as_str)
            .unwrap_or("No review summary.")
            .to_string(),
    })
}

pub fn insert_review_result(connection: &Connection, frame_id: i64) -> Result<ReviewSummary, AppError> {
    let (input_snapshot_json, stage, storage_kind, image_path): (String, String, String, String) = connection
        .query_row(
            "
            SELECT rolls.input_snapshot_json, frames.stage, frames.storage_kind, frames.image_path
            FROM frames
            INNER JOIN rolls ON rolls.id = frames.roll_id
            WHERE frames.id = ?1
            ",
            [frame_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to load review context for frame {frame_id}"),
            source,
        })?;

    let input_snapshot: CreateRollRequest =
        serde_json::from_str(&input_snapshot_json).map_err(|source| AppError::Json {
            context: format!("failed to parse input snapshot for frame {frame_id}"),
            source,
        })?;
    let computed = review::evaluate(&input_snapshot, &stage, &storage_kind, &image_path);
    let scores = serde_json::json!({
        "ai_feeling": computed.ai_feeling,
        "everyday_life": computed.everyday_life,
        "accidental_feeling": computed.accidental_feeling,
        "memory_quality": computed.memory_quality,
        "imperfection": computed.imperfection,
        "composition_balance": computed.composition_balance,
        "overall": computed.overall
    });

    connection
        .execute(
            "
            INSERT INTO review_results (
              frame_id,
              review_engine_version,
              evaluator_type,
              scores_json,
              summary_json
            )
            VALUES (?1, 'rule_based_v1', 'rule_based', ?2, ?3)
            ",
            params![
                frame_id,
                scores.to_string(),
                serde_json::json!({
                    "summary": computed.summary
                })
                .to_string()
            ],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to insert review result for frame {frame_id}"),
            source,
        })?;

    Ok(ReviewSummary {
        frame_id,
        review_engine_version: "rule_based_v1".to_string(),
        evaluator_type: "rule_based".to_string(),
        overall_score: computed.overall,
        ai_feeling: computed.ai_feeling,
        accidental_feeling: computed.accidental_feeling,
        everyday_life: computed.everyday_life,
        memory_quality: computed.memory_quality,
        imperfection: computed.imperfection,
        composition_balance: computed.composition_balance,
        summary: computed.summary,
    })
}

pub fn latest_review_for_roll(connection: &Connection, roll_id: i64) -> Result<Option<ReviewSummary>, AppError> {
    let row = connection
        .query_row(
            "
            SELECT
              review_results.frame_id,
              review_results.review_engine_version,
              review_results.evaluator_type,
              review_results.scores_json,
              review_results.summary_json
            FROM review_results
            INNER JOIN frames ON frames.id = review_results.frame_id
            WHERE frames.roll_id = ?1
            ORDER BY review_results.id DESC
            LIMIT 1
            ",
            [roll_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to load latest review for roll {roll_id}"),
            source,
        })?;

    row.map(
        |(frame_id, review_engine_version, evaluator_type, scores_json, summary_json)| {
            decode_review_summary(
                frame_id,
                review_engine_version,
                evaluator_type,
                scores_json,
                summary_json,
            )
        },
    )
    .transpose()
}
