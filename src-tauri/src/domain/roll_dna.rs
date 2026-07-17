use serde_json::{json, Value};

use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};

pub struct ResolvedSetupValues {
    pub moment: String,
    pub place: String,
    pub time: String,
    pub season: String,
    pub weather: String,
    pub tiny_detail: String,
}

fn resolve_field(field: &SetupInputField, resolved_value: &str, fallback: &str) -> Value {
    match field.mode {
        InputMode::Manual => json!({
            "resolved_value": field.value.clone().unwrap_or_else(|| resolved_value.to_string()),
            "source_mode": "manual"
        }),
        InputMode::LockedRandom => json!({
            "resolved_value": if resolved_value.is_empty() { fallback.to_string() } else { resolved_value.to_string() },
            "source_mode": "locked_random"
        }),
        InputMode::Random => json!({
            "resolved_value": if resolved_value.is_empty() { fallback.to_string() } else { resolved_value.to_string() },
            "source_mode": "random"
        }),
    }
}

fn camera_profile_for(country_code: &str, resolved: &ResolvedSetupValues) -> Value {
    let fingerprint = format!(
        "{}|{}|{}|{}|{}|{}|{}",
        country_code,
        resolved.moment,
        resolved.place,
        resolved.time,
        resolved.season,
        resolved.weather,
        resolved.tiny_detail
    );
    let profile_index = stable_profile_index(&fingerprint, 5);

    match profile_index {
        0 => json!({
            "kind": "ordinary_automatic_compact",
            "family": "consumer_point_and_shoot",
            "focus": "auto",
            "exposure": "auto",
            "white_balance": "auto",
            "lens_behavior": "modest_center_sharpness_soft_edges",
            "color_response": "plain_consumer_color",
            "flash_behavior": "available_light_or_weak_auto_flash"
        }),
        1 => json!({
            "kind": "disposable_camera",
            "family": "cheap_plastic_lens_35mm",
            "focus": "fixed_focus",
            "exposure": "simple_auto",
            "white_balance": "film_stock_response",
            "lens_behavior": "soft_edges_low_microcontrast",
            "color_response": "mild_warm_or_green_cast",
            "flash_behavior": "small_direct_flash_when_light_is_low"
        }),
        2 => json!({
            "kind": "instant_camera",
            "family": "consumer_instant_film",
            "focus": "simple_fixed_or_zone_focus",
            "exposure": "imperfect_auto",
            "white_balance": "chemical_film_response",
            "lens_behavior": "soft_focus_gentle_bloom",
            "color_response": "lifted_blacks_and_soft_color_shift",
            "flash_behavior": "flat_on_camera_flash_when_needed"
        }),
        3 => json!({
            "kind": "lomo_like_compact",
            "family": "toy_lens_35mm",
            "focus": "loose_zone_focus",
            "exposure": "imprecise_auto",
            "white_balance": "film_stock_response",
            "lens_behavior": "uneven_sharpness_subtle_vignette",
            "color_response": "small_unpredictable_color_shift",
            "flash_behavior": "mostly_available_light"
        }),
        _ => json!({
            "kind": "cheap_point_and_shoot",
            "family": "consumer_compact_35mm",
            "focus": "auto_with_minor_misses",
            "exposure": "auto_with_small_errors",
            "white_balance": "auto_or_film_stock_response",
            "lens_behavior": "ordinary_small_lens_softness",
            "color_response": "muted_one_hour_print_color",
            "flash_behavior": "weak_direct_flash_when_light_is_low"
        }),
    }
}

fn stable_profile_index(value: &str, bucket_count: u64) -> u64 {
    let mut hash = 14_695_981_039_346_656_037_u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    hash % bucket_count
}

pub fn resolve_roll_dna(
    request: &CreateRollRequest,
    country_code: &str,
    resolved: &ResolvedSetupValues,
) -> Value {
    json!({
        "version": "v1",
        "country_context": {
            "code": country_code,
            "kind": "daily_life_context"
        },
        "moment_context": resolve_field(&request.moment, &resolved.moment, "ordinary passing moment"),
        "place_context": resolve_field(&request.place, &resolved.place, "somewhere routine"),
        "time_context": resolve_field(&request.time, &resolved.time, "afternoon"),
        "season_context": resolve_field(&request.season, &resolved.season, "autumn"),
        "weather_context": resolve_field(&request.weather, &resolved.weather, "cloudy"),
        "tiny_detail_context": resolve_field(&request.tiny_detail, &resolved.tiny_detail, "something half noticed"),
        "dictionary_selections": {
            "bundle_version": "v1-initial"
        },
        "camera_profile": camera_profile_for(country_code, resolved),
        "imperfection_profile": {
            "framing": "slightly_awkward",
            "timing": "ordinary",
            "error_budget": "one_small_mistake"
        },
        "frame_variation_policy": {
            "contact_sheet_frame_count": 8,
            "timing_drift": "subtle",
            "obstruction_variance": "subtle",
            "focus_variance": "subtle"
        },
        "provider_context": {
            "provider_key": "openai",
            "provider_model": "unconfigured"
        },
        "extensions": {}
    })
}

#[cfg(test)]
mod tests {
    use super::{resolve_roll_dna, ResolvedSetupValues};
    use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};

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
            place: field(InputMode::Random, None),
            time: field(InputMode::Random, None),
            season: field(InputMode::Random, None),
            weather: field(InputMode::Random, None),
            tiny_detail: field(InputMode::Random, None),
        }
    }

    fn resolved_values(place: &str) -> ResolvedSetupValues {
        ResolvedSetupValues {
            moment: "buying a drink before the train".to_string(),
            place: place.to_string(),
            time: "evening".to_string(),
            season: "summer".to_string(),
            weather: "humid".to_string(),
            tiny_detail: "a receipt folded under a thumb".to_string(),
        }
    }

    #[test]
    fn roll_dna_assigns_reproducible_camera_profile() {
        let current = request();
        let resolved = resolved_values("station kiosk");

        let first = resolve_roll_dna(&current, "jp", &resolved);
        let second = resolve_roll_dna(&current, "jp", &resolved);

        assert_eq!(first.get("camera_profile"), second.get("camera_profile"));
        assert!(first
            .get("camera_profile")
            .and_then(|node| node.get("family"))
            .and_then(serde_json::Value::as_str)
            .is_some());
        assert!(first
            .get("camera_profile")
            .and_then(|node| node.get("lens_behavior"))
            .and_then(serde_json::Value::as_str)
            .is_some());
    }
}
