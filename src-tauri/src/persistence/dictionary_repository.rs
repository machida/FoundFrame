use rusqlite::{params, Connection, OptionalExtension, Transaction};

use crate::dto::dictionary::{
    DictionaryBundleFile, DictionaryCategoriesFile, DictionaryEntriesFile, DictionaryEntryRecord,
};
use crate::errors::AppError;

fn country_display_name(code: &str) -> &str {
    match code {
        "jp" => "Japan",
        "us" => "United States",
        _ => code,
    }
}

fn to_json<T: serde::Serialize>(value: &Option<T>) -> Result<Option<String>, AppError> {
    value
        .as_ref()
        .map(|item| {
            serde_json::to_string(item).map_err(|source| AppError::Json {
                context: "failed to serialize value to json".to_string(),
                source,
            })
        })
        .transpose()
}

fn get_country_id(tx: &Transaction<'_>, code: &str) -> Result<i64, AppError> {
    tx.query_row("SELECT id FROM countries WHERE code = ?1", [code], |row| row.get(0))
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to fetch country id for code {code}"),
            source,
        })
}

fn get_category_id(tx: &Transaction<'_>, key: &str) -> Result<i64, AppError> {
    tx.query_row(
        "SELECT id FROM dictionary_categories WHERE key = ?1",
        [key],
        |row| row.get(0),
    )
    .map_err(|source| AppError::Sqlite {
        context: format!("failed to fetch category id for key {key}"),
        source,
    })
}

pub fn upsert_categories(
    connection: &mut Connection,
    categories: &DictionaryCategoriesFile,
) -> Result<(), AppError> {
    let tx = connection.transaction().map_err(|source| AppError::Sqlite {
        context: "failed to begin category transaction".to_string(),
        source,
    })?;

    for category in &categories.categories {
        tx.execute(
            "
            INSERT INTO dictionary_categories (key, display_name, description)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(key) DO UPDATE SET
              display_name = excluded.display_name,
              description = excluded.description
            ",
            params![category.key, category.display_name, category.description],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to upsert category {}", category.key),
            source,
        })?;
    }

    tx.commit().map_err(|source| AppError::Sqlite {
        context: "failed to commit category transaction".to_string(),
        source,
    })
}

pub fn upsert_bundle(connection: &Connection, bundle: &DictionaryBundleFile) -> Result<(), AppError> {
    let country_scope_json =
        serde_json::to_string(&bundle.country_scope).map_err(|source| AppError::Json {
            context: "failed to serialize bundle country scope".to_string(),
            source,
        })?;

    connection
        .execute(
            "
            INSERT INTO dictionary_bundles (version, country_scope, notes)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(version) DO UPDATE SET
              country_scope = excluded.country_scope,
              notes = excluded.notes
            ",
            params![bundle.version, country_scope_json, bundle.notes],
        )
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to upsert dictionary bundle {}", bundle.version),
            source,
        })?;

    Ok(())
}

fn upsert_country(tx: &Transaction<'_>, code: &str, is_default: bool) -> Result<(), AppError> {
    tx.execute(
        "
        INSERT INTO countries (code, display_name, is_default, is_featured, status)
        VALUES (?1, ?2, ?3, 1, 'active')
        ON CONFLICT(code) DO UPDATE SET
          display_name = excluded.display_name,
          is_default = excluded.is_default,
          is_featured = excluded.is_featured,
          status = excluded.status
        ",
        params![code, country_display_name(code), if is_default { 1 } else { 0 }],
    )
    .map_err(|source| AppError::Sqlite {
        context: format!("failed to upsert country {code}"),
        source,
    })?;

    Ok(())
}

fn upsert_entry(
    tx: &Transaction<'_>,
    country_id: i64,
    category_id: i64,
    entry: &DictionaryEntryRecord,
) -> Result<(), AppError> {
    tx.execute(
        "
        INSERT INTO dictionary_entries (
          country_id, category_id, slug, label, description, rarity, weight, visibility,
          seasonality_json, weather_json, time_context_json, environment_json,
          compatibility_json, tags_json, editorial_notes, status, source_type
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, 'yaml')
        ON CONFLICT(country_id, slug) DO UPDATE SET
          category_id = excluded.category_id,
          label = excluded.label,
          description = excluded.description,
          rarity = excluded.rarity,
          weight = excluded.weight,
          visibility = excluded.visibility,
          seasonality_json = excluded.seasonality_json,
          weather_json = excluded.weather_json,
          time_context_json = excluded.time_context_json,
          environment_json = excluded.environment_json,
          compatibility_json = excluded.compatibility_json,
          tags_json = excluded.tags_json,
          editorial_notes = excluded.editorial_notes,
          status = excluded.status,
          source_type = excluded.source_type,
          updated_at = CURRENT_TIMESTAMP
        ",
        params![
            country_id,
            category_id,
            entry.slug,
            entry.label,
            entry.description,
            entry.rarity,
            entry.weight,
            entry.visibility,
            to_json(&entry.seasonality)?,
            to_json(&entry.weather)?,
            to_json(&entry.time_context)?,
            to_json(&entry.environment)?,
            to_json(&entry.compatibility)?,
            to_json(&entry.tags)?,
            entry.editorial_notes,
            entry.status
        ],
    )
    .map_err(|source| AppError::Sqlite {
        context: format!("failed to upsert dictionary entry {}", entry.slug),
        source,
    })?;

    Ok(())
}

pub fn upsert_entries_file(
    connection: &mut Connection,
    entries_file: &DictionaryEntriesFile,
) -> Result<(), AppError> {
    let tx = connection.transaction().map_err(|source| AppError::Sqlite {
        context: "failed to begin dictionary entry transaction".to_string(),
        source,
    })?;

    let is_default_country = entries_file.country == "jp";
    upsert_country(&tx, &entries_file.country, is_default_country)?;
    let country_id = get_country_id(&tx, &entries_file.country)?;

    for entry in &entries_file.entries {
        let category_id = get_category_id(&tx, &entry.category)?;
        upsert_entry(&tx, country_id, category_id, entry)?;
    }

    tx.commit().map_err(|source| AppError::Sqlite {
        context: format!(
            "failed to commit dictionary entries for country {}",
            entries_file.country
        ),
        source,
    })
}

pub fn count_countries(connection: &Connection) -> Result<i64, AppError> {
    connection
        .query_row("SELECT COUNT(*) FROM countries", [], |row| row.get(0))
        .map_err(|source| AppError::Sqlite {
            context: "failed to count countries".to_string(),
            source,
        })
}

pub fn count_entries(connection: &Connection) -> Result<i64, AppError> {
    connection
        .query_row("SELECT COUNT(*) FROM dictionary_entries", [], |row| row.get(0))
        .map_err(|source| AppError::Sqlite {
            context: "failed to count dictionary entries".to_string(),
            source,
        })
}

pub fn list_countries(connection: &Connection) -> Result<Vec<(String, String, bool)>, AppError> {
    let mut stmt = connection
        .prepare(
            "SELECT code, display_name, is_default
             FROM countries
             WHERE status = 'active'
             ORDER BY is_default DESC, display_name ASC",
        )
        .map_err(|source| AppError::Sqlite {
            context: "failed to prepare country list query".to_string(),
            source,
        })?;

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)? == 1,
            ))
        })
        .map_err(|source| AppError::Sqlite {
            context: "failed to query countries".to_string(),
            source,
        })?;

    let mut countries = Vec::new();
    for row in rows {
        countries.push(row.map_err(|source| AppError::Sqlite {
            context: "failed to decode country row".to_string(),
            source,
        })?);
    }

    Ok(countries)
}

pub fn default_country_code(connection: &Connection) -> Result<Option<String>, AppError> {
    connection
        .query_row(
            "SELECT code FROM countries WHERE is_default = 1 LIMIT 1",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: "failed to query default country".to_string(),
            source,
        })
}

pub fn country_id_by_code(connection: &Connection, code: &str) -> Result<Option<i64>, AppError> {
    connection
        .query_row("SELECT id FROM countries WHERE code = ?1 LIMIT 1", [code], |row| row.get(0))
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!("failed to query country id for code {code}"),
            source,
        })
}

pub fn random_country_code(connection: &Connection) -> Result<Option<String>, AppError> {
    connection
        .query_row(
            "
            SELECT code
            FROM countries
            WHERE status = 'active'
            ORDER BY RANDOM()
            LIMIT 1
            ",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: "failed to select random country code".to_string(),
            source,
        })
}

pub fn random_entry_label(
    connection: &Connection,
    country_code: &str,
    category_key: &str,
    season: Option<&str>,
    weather: Option<&str>,
    time_context: Option<&str>,
) -> Result<Option<String>, AppError> {
    connection
        .query_row(
            "
            SELECT dictionary_entries.label
            FROM dictionary_entries
            INNER JOIN countries ON countries.id = dictionary_entries.country_id
            INNER JOIN dictionary_categories ON dictionary_categories.id = dictionary_entries.category_id
            WHERE countries.code = ?1
              AND countries.status = 'active'
              AND dictionary_categories.key = ?2
              AND dictionary_entries.status = 'active'
              AND (?3 IS NULL OR dictionary_entries.seasonality_json IS NULL OR dictionary_entries.seasonality_json LIKE '%' || '\"' || ?3 || '\"' || '%')
              AND (?4 IS NULL OR dictionary_entries.weather_json IS NULL OR dictionary_entries.weather_json LIKE '%' || '\"' || ?4 || '\"' || '%')
              AND (?5 IS NULL OR dictionary_entries.time_context_json IS NULL OR dictionary_entries.time_context_json LIKE '%' || '\"' || ?5 || '\"' || '%')
            ORDER BY RANDOM()
            LIMIT 1
            ",
            params![country_code, category_key, season, weather, time_context],
            |row| row.get(0),
        )
        .optional()
        .map_err(|source| AppError::Sqlite {
            context: format!(
                "failed to select random dictionary entry for country {country_code} and category {category_key}"
            ),
            source,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    use crate::dto::dictionary::{
        DictionaryCategoriesFile, DictionaryCategoryRecord, DictionaryEntriesFile, DictionaryEntryRecord,
    };
    use crate::persistence::migrations;

    fn categories_fixture() -> DictionaryCategoriesFile {
        DictionaryCategoriesFile {
            version: 1,
            categories: vec![
                DictionaryCategoryRecord {
                    key: "moment".to_string(),
                    display_name: "Moment".to_string(),
                    description: None,
                },
                DictionaryCategoryRecord {
                    key: "place".to_string(),
                    display_name: "Place".to_string(),
                    description: None,
                },
                DictionaryCategoryRecord {
                    key: "object_detail".to_string(),
                    display_name: "Object Detail".to_string(),
                    description: None,
                },
            ],
        }
    }

    fn entry(
        slug: &str,
        label: &str,
        category: &str,
    ) -> DictionaryEntryRecord {
        DictionaryEntryRecord {
            slug: slug.to_string(),
            label: label.to_string(),
            description: None,
            category: category.to_string(),
            rarity: "common".to_string(),
            weight: 50,
            visibility: "foreground".to_string(),
            seasonality: None,
            weather: None,
            time_context: None,
            environment: None,
            tags: None,
            compatibility: None,
            editorial_notes: None,
            status: "active".to_string(),
        }
    }

    #[test]
    fn upsert_entries_file_creates_country_and_supports_random_lookup() {
        let mut connection = Connection::open_in_memory().expect("open in-memory database");
        migrations::apply_all(&connection).expect("apply migrations");
        upsert_categories(&mut connection, &categories_fixture()).expect("upsert categories");

        let us_entries = DictionaryEntriesFile {
            version: 1,
            country: "us".to_string(),
            entries: vec![
                entry(
                    "carrying-takeout-back-to-the-car-before-it-gets-cold",
                    "carrying takeout back to the car before it gets cold",
                    "moment",
                ),
                entry(
                    "neighborhood-gas-station-side-lot",
                    "neighborhood gas station side lot",
                    "place",
                ),
                entry(
                    "paper-takeout-bag-darkening-with-grease",
                    "paper takeout bag darkening with grease",
                    "object_detail",
                ),
            ],
        };

        upsert_entries_file(&mut connection, &us_entries).expect("upsert us entries");

        assert_eq!(count_countries(&connection).expect("count countries"), 1);
        assert_eq!(default_country_code(&connection).expect("default country"), None);

        let countries = list_countries(&connection).expect("list countries");
        assert_eq!(
            countries,
            vec![("us".to_string(), "United States".to_string(), false)]
        );

        let moment = random_entry_label(&connection, "us", "moment", None, None, None)
            .expect("query random moment");
        let detail = random_entry_label(&connection, "us", "object_detail", None, None, None)
            .expect("query random object detail");

        assert_eq!(
            moment,
            Some("carrying takeout back to the car before it gets cold".to_string())
        );
        assert_eq!(
            detail,
            Some("paper takeout bag darkening with grease".to_string())
        );
    }

    #[test]
    fn japan_entries_remain_default_country_when_imported() {
        let mut connection = Connection::open_in_memory().expect("open in-memory database");
        migrations::apply_all(&connection).expect("apply migrations");
        upsert_categories(&mut connection, &categories_fixture()).expect("upsert categories");

        let jp_entries = DictionaryEntriesFile {
            version: 1,
            country: "jp".to_string(),
            entries: vec![entry(
                "waiting-for-someone-who-is-late",
                "waiting for someone who is late",
                "moment",
            )],
        };

        upsert_entries_file(&mut connection, &jp_entries).expect("upsert jp entries");

        assert_eq!(
            default_country_code(&connection).expect("default country"),
            Some("jp".to_string())
        );
    }
}
