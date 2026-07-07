use rusqlite::Connection;

use crate::errors::AppError;

const INITIAL_MIGRATION: &str = include_str!("../../migrations/0001_initial.sql");

pub fn apply_all(connection: &Connection) -> Result<(), AppError> {
    connection
        .execute_batch(INITIAL_MIGRATION)
        .map_err(|source| AppError::Sqlite {
            context: "failed to apply initial migration batch".to_string(),
            source,
        })
}
