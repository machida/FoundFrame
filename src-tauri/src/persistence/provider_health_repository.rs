use rusqlite::{params, Connection, OptionalExtension};

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct ProviderHealthRecord {
    pub status: String,
    pub last_check_message: Option<String>,
    pub last_check_at: Option<String>,
}

pub fn fetch_provider_health(
    connection: &Connection,
    provider_key: &str,
) -> Result<Option<ProviderHealthRecord>, AppError> {
    connection
        .query_row(
            "
            SELECT status, last_check_message, last_check_at
            FROM provider_health
            WHERE provider_key = ?1
            ",
            params![provider_key],
            |row| {
                Ok(ProviderHealthRecord {
                    status: row.get(0)?,
                    last_check_message: row.get(1)?,
                    last_check_at: row.get(2)?,
                })
            },
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch provider health for {provider_key}"),
            source,
        })
}

pub fn upsert_provider_health(
    connection: &Connection,
    provider_key: &str,
    status: &str,
    last_check_message: Option<&str>,
    last_check_at: Option<&str>,
) -> Result<(), AppError> {
    connection
        .execute(
            "
            INSERT INTO provider_health (
              provider_key,
              status,
              last_check_message,
              last_check_at,
              updated_at
            )
            VALUES (?1, ?2, ?3, ?4, CURRENT_TIMESTAMP)
            ON CONFLICT(provider_key) DO UPDATE SET
              status = excluded.status,
              last_check_message = excluded.last_check_message,
              last_check_at = excluded.last_check_at,
              updated_at = CURRENT_TIMESTAMP
            ",
            params![provider_key, status, last_check_message, last_check_at],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to upsert provider health for {provider_key}"),
            source,
        })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::{fetch_provider_health, upsert_provider_health};
    use crate::persistence::migrations;

    fn setup_connection() -> Connection {
        let connection = Connection::open_in_memory().expect("in-memory sqlite");
        migrations::apply_all(&connection).expect("apply migrations");
        connection
    }

    #[test]
    fn upsert_provider_health_creates_and_updates_one_record() {
        let connection = setup_connection();

        upsert_provider_health(
            &connection,
            "openai",
            "saved_unverified",
            Some("saved but unchecked"),
            Some("2026-07-01T10:00:00Z"),
        )
        .expect("insert provider health");

        let first = fetch_provider_health(&connection, "openai")
            .expect("fetch provider health")
            .expect("provider health exists");
        assert_eq!(first.status, "saved_unverified");
        assert_eq!(first.last_check_message.as_deref(), Some("saved but unchecked"));
        assert_eq!(first.last_check_at.as_deref(), Some("2026-07-01T10:00:00Z"));

        upsert_provider_health(
            &connection,
            "openai",
            "degraded",
            Some("provider_timeout"),
            Some("2026-07-01T11:00:00Z"),
        )
        .expect("update provider health");

        let second = fetch_provider_health(&connection, "openai")
            .expect("fetch provider health")
            .expect("provider health exists");
        assert_eq!(second.status, "degraded");
        assert_eq!(second.last_check_message.as_deref(), Some("provider_timeout"));
        assert_eq!(second.last_check_at.as_deref(), Some("2026-07-01T11:00:00Z"));

        let count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM provider_health WHERE provider_key = 'openai'",
                [],
                |row| row.get(0),
            )
            .expect("count provider health rows");
        assert_eq!(count, 1);
    }
}
