use rusqlite::Connection;

use crate::errors::AppError;

pub fn set_favorite(
    connection: &Connection,
    frame_id: i64,
    is_favorite: bool,
) -> Result<(), AppError> {
    if is_favorite {
        connection
            .execute(
                "
                INSERT INTO favorites (frame_id)
                VALUES (?1)
                ON CONFLICT(frame_id) DO NOTHING
                ",
                [frame_id],
            )
            .map_err(|source| AppError::Sqlite {
                context: format!("failed to favorite frame {frame_id}"),
                source,
            })?;
    } else {
        connection
            .execute("DELETE FROM favorites WHERE frame_id = ?1", [frame_id])
            .map_err(|source| AppError::Sqlite {
                context: format!("failed to unfavorite frame {frame_id}"),
                source,
            })?;
    }

    Ok(())
}
