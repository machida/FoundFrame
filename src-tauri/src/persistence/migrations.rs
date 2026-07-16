use rusqlite::Connection;

use crate::errors::AppError;

const INITIAL_MIGRATION: &str = include_str!("../../migrations/0001_initial.sql");
const INITIAL_SCHEMA_VERSION: i64 = 1;
const INITIAL_SCHEMA_TABLES: [&str; 12] = [
    "countries",
    "dictionary_categories",
    "dictionary_bundles",
    "dictionary_entries",
    "rolls",
    "generation_jobs",
    "frames",
    "review_results",
    "favorites",
    "presets",
    "roll_events",
    "provider_health",
];

fn sqlite_error(context: impl Into<String>, source: rusqlite::Error) -> AppError {
    AppError::Sqlite {
        context: context.into(),
        source,
    }
}

fn schema_version(connection: &Connection) -> Result<i64, AppError> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|source| sqlite_error("failed to read database schema version", source))
}

fn existing_initial_table_count(connection: &Connection) -> Result<usize, AppError> {
    let mut count = 0;

    for table_name in INITIAL_SCHEMA_TABLES {
        let exists = connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
                [table_name],
                |row| row.get::<_, bool>(0),
            )
            .map_err(|source| {
                sqlite_error(
                    format!("failed to inspect initial schema table {table_name}"),
                    source,
                )
            })?;

        if exists {
            count += 1;
        }
    }

    Ok(count)
}

fn mark_initial_schema_applied(connection: &Connection) -> Result<(), AppError> {
    connection
        .pragma_update(None, "user_version", INITIAL_SCHEMA_VERSION)
        .map_err(|source| sqlite_error("failed to record initial database schema version", source))
}

pub fn apply_all(connection: &Connection) -> Result<(), AppError> {
    let version = schema_version(connection)?;

    if version == INITIAL_SCHEMA_VERSION {
        return Ok(());
    }

    if version > INITIAL_SCHEMA_VERSION {
        return Err(AppError::Config {
            context: format!(
                "database schema version {version} is newer than supported version {INITIAL_SCHEMA_VERSION}"
            ),
        });
    }

    let existing_table_count = existing_initial_table_count(connection)?;
    if existing_table_count == INITIAL_SCHEMA_TABLES.len() {
        // Databases created before schema version tracking already contain the complete v1 schema.
        // Preserve their data and adopt them instead of replaying CREATE TABLE statements.
        return mark_initial_schema_applied(connection);
    }

    if existing_table_count > 0 {
        return Err(AppError::Config {
            context: format!(
                "database has a partial initial schema ({existing_table_count}/{} expected tables); refusing to overwrite it",
                INITIAL_SCHEMA_TABLES.len()
            ),
        });
    }

    connection
        .execute_batch("BEGIN IMMEDIATE")
        .map_err(|source| sqlite_error("failed to begin initial migration transaction", source))?;

    let result = (|| {
        connection
            .execute_batch(INITIAL_MIGRATION)
            .map_err(|source| sqlite_error("failed to apply initial migration batch", source))?;
        mark_initial_schema_applied(connection)?;
        connection.execute_batch("COMMIT").map_err(|source| {
            sqlite_error("failed to commit initial migration transaction", source)
        })
    })();

    if result.is_err() {
        let _ = connection.execute_batch("ROLLBACK");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{apply_all, schema_version, INITIAL_MIGRATION, INITIAL_SCHEMA_VERSION};
    use crate::errors::AppError;
    use rusqlite::Connection;

    #[test]
    fn applying_migrations_twice_is_safe() {
        let connection = Connection::open_in_memory().expect("open database");

        apply_all(&connection).expect("apply initial migration");
        apply_all(&connection).expect("reapply migrations");

        assert_eq!(
            schema_version(&connection).expect("read version"),
            INITIAL_SCHEMA_VERSION
        );
    }

    #[test]
    fn adopts_complete_legacy_schema_without_losing_data() {
        let connection = Connection::open_in_memory().expect("open database");
        connection
            .execute_batch(INITIAL_MIGRATION)
            .expect("create legacy schema");
        connection
            .execute(
                "INSERT INTO countries (code, display_name, is_default) VALUES ('jp', 'Japan', 1)",
                [],
            )
            .expect("insert legacy data");

        apply_all(&connection).expect("adopt legacy schema");

        let country_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM countries", [], |row| row.get(0))
            .expect("count countries");
        assert_eq!(country_count, 1);
        assert_eq!(
            schema_version(&connection).expect("read version"),
            INITIAL_SCHEMA_VERSION
        );
    }

    #[test]
    fn refuses_to_overwrite_partial_legacy_schema() {
        let connection = Connection::open_in_memory().expect("open database");
        connection
            .execute_batch("CREATE TABLE countries (id INTEGER PRIMARY KEY)")
            .expect("create partial schema");

        let error = apply_all(&connection).expect_err("partial schema should fail");
        assert!(matches!(error, AppError::Config { .. }));
    }
}
