use rusqlite::Connection;

use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};
use crate::domain::roll_dna::ResolvedSetupValues;
use crate::errors::AppError;
use crate::persistence::dictionary_repository;

const TIME_OPTIONS: [&str; 7] = [
    "early_morning",
    "morning",
    "noon",
    "afternoon",
    "late_afternoon",
    "evening",
    "night",
];
const SEASON_OPTIONS: [&str; 4] = ["spring", "summer", "autumn", "winter"];
const WEATHER_OPTIONS: [&str; 6] = ["clear", "cloudy", "rain", "drizzle", "humid", "snow"];

fn stable_index(seed: &str, len: usize) -> usize {
    let sum = seed
        .bytes()
        .fold(0_usize, |acc, byte| acc.wrapping_add(byte as usize));
    sum % len
}

fn choose_controlled_value(field: &SetupInputField, options: &[&str], seed: &str) -> String {
    match field.mode {
        InputMode::Manual | InputMode::LockedRandom => field
            .value
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| options[stable_index(seed, options.len())].to_string()),
        InputMode::Random => options[stable_index(seed, options.len())].to_string(),
    }
}

pub fn resolved_country_code(
    connection: &Connection,
    request: &CreateRollRequest,
    fallback: Option<String>,
) -> Result<String, AppError> {
    Ok(match request.country.mode {
        InputMode::Manual | InputMode::LockedRandom => request
            .country
            .value
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| fallback.unwrap_or_else(|| "jp".to_string())),
        InputMode::Random => dictionary_repository::random_country_code(connection)?
            .or(fallback)
            .unwrap_or_else(|| "jp".to_string()),
    })
}

pub fn resolve_setup_values(
    connection: &Connection,
    request: &CreateRollRequest,
    country_code: &str,
) -> Result<ResolvedSetupValues, AppError> {
    let time = choose_controlled_value(&request.time, &TIME_OPTIONS, &format!("{country_code}:time"));
    let season =
        choose_controlled_value(&request.season, &SEASON_OPTIONS, &format!("{country_code}:season"));
    let weather =
        choose_controlled_value(&request.weather, &WEATHER_OPTIONS, &format!("{country_code}:weather"));

    let moment = match request.moment.mode {
        InputMode::Manual => request
            .moment
            .value
            .clone()
            .unwrap_or_else(|| "ordinary passing moment".to_string()),
        InputMode::LockedRandom if request.moment.value.clone().unwrap_or_default().trim().is_empty() => {
            dictionary_repository::random_entry_label(connection, country_code, "moment", None, None, Some(&time))?
                .unwrap_or_else(|| "ordinary passing moment".to_string())
        }
        InputMode::LockedRandom => request
            .moment
            .value
            .clone()
            .unwrap_or_else(|| "ordinary passing moment".to_string()),
        InputMode::Random => dictionary_repository::random_entry_label(connection, country_code, "moment", None, None, Some(&time))?
            .unwrap_or_else(|| "ordinary passing moment".to_string()),
    };

    let place = match request.place.mode {
        InputMode::Manual => request
            .place
            .value
            .clone()
            .unwrap_or_else(|| "somewhere routine".to_string()),
        InputMode::LockedRandom if request.place.value.clone().unwrap_or_default().trim().is_empty() => {
            dictionary_repository::random_entry_label(
                connection,
                country_code,
                "place",
                Some(&season),
                Some(&weather),
                Some(&time),
            )?
            .unwrap_or_else(|| "somewhere routine".to_string())
        }
        InputMode::LockedRandom => request.place.value.clone().unwrap_or_else(|| "somewhere routine".to_string()),
        InputMode::Random => dictionary_repository::random_entry_label(
            connection,
            country_code,
            "place",
            Some(&season),
            Some(&weather),
            Some(&time),
        )?
        .unwrap_or_else(|| "somewhere routine".to_string()),
    };

    let tiny_detail = match request.tiny_detail.mode {
        InputMode::Manual => request
            .tiny_detail
            .value
            .clone()
            .unwrap_or_else(|| "something half noticed".to_string()),
        InputMode::LockedRandom if request.tiny_detail.value.clone().unwrap_or_default().trim().is_empty() => {
            dictionary_repository::random_entry_label(
                connection,
                country_code,
                "object_detail",
                Some(&season),
                Some(&weather),
                Some(&time),
            )?
            .unwrap_or_else(|| "something half noticed".to_string())
        }
        InputMode::LockedRandom => request
            .tiny_detail
            .value
            .clone()
            .unwrap_or_else(|| "something half noticed".to_string()),
        InputMode::Random => dictionary_repository::random_entry_label(
            connection,
            country_code,
            "object_detail",
            Some(&season),
            Some(&weather),
            Some(&time),
        )?
        .unwrap_or_else(|| "something half noticed".to_string()),
    };

    Ok(ResolvedSetupValues {
        moment,
        place,
        time,
        season,
        weather,
        tiny_detail,
    })
}

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::{resolve_setup_values, resolved_country_code};
    use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};
    use crate::persistence::migrations;

    fn field(mode: InputMode, value: Option<&str>) -> SetupInputField {
        SetupInputField {
            mode,
            value: value.map(|item| item.to_string()),
        }
    }

    fn request() -> CreateRollRequest {
        CreateRollRequest {
            country: field(InputMode::Random, None),
            moment: field(InputMode::Random, None),
            place: field(InputMode::Random, None),
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
                INSERT INTO countries (id, code, display_name, is_default, is_featured, status)
                VALUES
                  (1, 'jp', 'Japan', 1, 1, 'active'),
                  (2, 'us', 'United States', 0, 1, 'active')
                ",
                [],
            )
            .expect("insert countries");
        connection
            .execute(
                "
                INSERT INTO dictionary_categories (id, key, display_name, description)
                VALUES
                  (1, 'moment', 'Moment', 'Moment'),
                  (2, 'place', 'Place', 'Place'),
                  (3, 'object_detail', 'Object Detail', 'Object Detail')
                ",
                [],
            )
            .expect("insert categories");
        connection
            .execute(
                "
                INSERT INTO dictionary_entries (
                  country_id, category_id, slug, label, description, rarity, weight, visibility,
                  seasonality_json, weather_json, time_context_json, environment_json,
                  compatibility_json, tags_json, editorial_notes, status, source_type
                )
                VALUES
                  (1, 1, 'jp-moment-1', 'after-school errand', 'moment', 'common', 1.0, 'normal', NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'active', 'test'),
                  (1, 2, 'jp-place-1', 'covered shopping street', 'place', 'common', 1.0, 'normal', NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'active', 'test'),
                  (1, 3, 'jp-detail-1', 'creased receipt in one hand', 'detail', 'common', 1.0, 'normal', NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'active', 'test')
                ",
                [],
            )
            .expect("insert dictionary entries");

        connection
    }

    fn setup_sparse_country_connection() -> Connection {
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
            .execute(
                "
                INSERT INTO dictionary_categories (id, key, display_name, description)
                VALUES
                  (1, 'moment', 'Moment', 'Moment'),
                  (2, 'place', 'Place', 'Place'),
                  (3, 'object_detail', 'Object Detail', 'Object Detail')
                ",
                [],
            )
            .expect("insert categories");
        connection
            .execute(
                "
                INSERT INTO dictionary_entries (
                  country_id, category_id, slug, label, description, rarity, weight, visibility,
                  seasonality_json, weather_json, time_context_json, environment_json,
                  compatibility_json, tags_json, editorial_notes, status, source_type
                )
                VALUES
                  (2, 2, 'us-place-1', 'apartment mailbox corridor', 'place', 'common', 1.0, 'normal', NULL, NULL, NULL, NULL, NULL, NULL, NULL, 'active', 'test')
                ",
                [],
            )
            .expect("insert sparse dictionary entries");

        connection
    }

    #[test]
    fn manual_values_override_dictionary_resolution() {
        let connection = setup_connection();
        let mut current = request();
        current.country = field(InputMode::Manual, Some("jp"));
        current.moment = field(InputMode::Manual, Some("waiting for the crossing light"));
        current.place = field(InputMode::Manual, Some("small station entrance"));
        current.time = field(InputMode::Manual, Some("night"));
        current.season = field(InputMode::Manual, Some("winter"));
        current.weather = field(InputMode::Manual, Some("snow"));
        current.tiny_detail = field(InputMode::Manual, Some("one glove tucked into a pocket"));

        let resolved = resolve_setup_values(&connection, &current, "jp").expect("resolve setup");

        assert_eq!(resolved.moment, "waiting for the crossing light");
        assert_eq!(resolved.place, "small station entrance");
        assert_eq!(resolved.time, "night");
        assert_eq!(resolved.season, "winter");
        assert_eq!(resolved.weather, "snow");
        assert_eq!(resolved.tiny_detail, "one glove tucked into a pocket");
    }

    #[test]
    fn empty_locked_random_values_resolve_from_dictionary_and_controlled_vocab() {
        let connection = setup_connection();
        let mut current = request();
        current.country = field(InputMode::Manual, Some("jp"));
        current.moment = field(InputMode::LockedRandom, None);
        current.place = field(InputMode::LockedRandom, None);
        current.time = field(InputMode::LockedRandom, None);
        current.season = field(InputMode::LockedRandom, None);
        current.weather = field(InputMode::LockedRandom, None);
        current.tiny_detail = field(InputMode::LockedRandom, None);

        let resolved = resolve_setup_values(&connection, &current, "jp").expect("resolve setup");

        assert_eq!(resolved.time, "early_morning");
        assert_eq!(resolved.season, "summer");
        assert_eq!(resolved.weather, "rain");
        assert_eq!(resolved.moment, "after-school errand");
        assert_eq!(resolved.place, "covered shopping street");
        assert_eq!(resolved.tiny_detail, "creased receipt in one hand");
    }

    #[test]
    fn random_country_uses_fallback_when_database_has_no_country() {
        let connection = Connection::open_in_memory().expect("in-memory sqlite");
        migrations::apply_all(&connection).expect("apply migrations");
        let current = request();

        let country_code =
            resolved_country_code(&connection, &current, Some("jp".to_string())).expect("resolve country");

        assert_eq!(country_code, "jp");
    }

    #[test]
    fn sparse_country_uses_country_specific_entries_when_present_and_generic_fallbacks_when_missing() {
        let connection = setup_sparse_country_connection();
        let mut current = request();
        current.country = field(InputMode::Manual, Some("us"));
        current.time = field(InputMode::Manual, Some("afternoon"));
        current.season = field(InputMode::Manual, Some("summer"));
        current.weather = field(InputMode::Manual, Some("humid"));

        let resolved = resolve_setup_values(&connection, &current, "us").expect("resolve setup");

        assert_eq!(resolved.place, "apartment mailbox corridor");
        assert_eq!(resolved.moment, "ordinary passing moment");
        assert_eq!(resolved.tiny_detail, "something half noticed");
    }

    #[test]
    fn random_country_can_resolve_sparse_country_without_failing() {
        let connection = setup_sparse_country_connection();
        let mut current = request();
        current.time = field(InputMode::Manual, Some("afternoon"));
        current.season = field(InputMode::Manual, Some("summer"));
        current.weather = field(InputMode::Manual, Some("humid"));

        let country_code =
            resolved_country_code(&connection, &current, Some("jp".to_string())).expect("resolve country");
        let resolved = resolve_setup_values(&connection, &current, &country_code).expect("resolve setup");

        if country_code == "us" {
            assert_eq!(resolved.place, "apartment mailbox corridor");
            assert_eq!(resolved.moment, "ordinary passing moment");
            assert_eq!(resolved.tiny_detail, "something half noticed");
        } else {
            assert_eq!(country_code, "jp");
        }
    }
}
