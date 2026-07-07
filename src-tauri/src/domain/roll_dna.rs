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
        "camera_profile": {
            "kind": "ordinary_automatic_compact",
            "focus": "auto",
            "exposure": "auto",
            "white_balance": "auto"
        },
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
