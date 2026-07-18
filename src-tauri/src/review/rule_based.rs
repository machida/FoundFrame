use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};

pub struct ReviewComputation {
    pub ai_feeling: f64,
    pub accidental_feeling: f64,
    pub everyday_life: f64,
    pub memory_quality: f64,
    pub imperfection: f64,
    pub composition_balance: f64,
    pub overall: f64,
    pub summary: String,
}

fn field_has_manual_value(field: &SetupInputField) -> bool {
    matches!(field.mode, InputMode::Manual) && field.value.is_some()
}

pub fn evaluate(
    input: &CreateRollRequest,
    stage: &str,
    storage_kind: &str,
    image_path: &str,
) -> ReviewComputation {
    // v1 keeps review local and rule-based so workflow iteration does not depend on a vision model yet.
    let manual_count = [
        &input.country,
        &input.moment,
        &input.place,
        &input.time,
        &input.season,
        &input.weather,
        &input.tiny_detail,
    ]
    .iter()
    .filter(|field| field_has_manual_value(field))
    .count() as f64;

    let random_bonus = (7.0 - manual_count) / 7.0;
    let has_png = image_path.ends_with(".png");
    let is_alternate_take = stage == "alternate_take";
    let is_placeholder = storage_kind != "app_managed" || image_path.ends_with(".svg");

    let ai_feeling = if is_placeholder { 0.62 } else { 0.28 };
    let everyday_life = (0.58 + random_bonus * 0.34).min(0.96);
    let accidental_feeling =
        (0.44 + random_bonus * 0.28 + if is_alternate_take { 0.08 } else { 0.0 }).min(0.94);
    let memory_quality = (0.49 + random_bonus * 0.18 + if has_png { 0.06 } else { 0.0 }).min(0.9);
    let imperfection =
        (0.52 + random_bonus * 0.24 + if is_placeholder { 0.08 } else { 0.0 }).min(0.95);
    let composition_balance = (0.36 + manual_count * 0.04).min(0.68);
    let overall = ((1.0 - ai_feeling) * 0.24
        + accidental_feeling * 0.24
        + everyday_life * 0.2
        + memory_quality * 0.18
        + imperfection * 0.14)
        .min(0.95);

    let summary = if overall >= 0.75 {
        "This frame survives because it feels ordinary, slightly unplanned, and quietly memorable."
    } else if ai_feeling > 0.5 {
        "This frame retains some everyday texture, but it still feels a bit too generated or resolved."
    } else {
        "This frame is usable, but it needs more accidental friction and less compositional stability."
    };

    ReviewComputation {
        ai_feeling,
        accidental_feeling,
        everyday_life,
        memory_quality,
        imperfection,
        composition_balance,
        overall,
        summary: summary.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate;
    use crate::dto::roll::{CreateRollRequest, InputMode, SetupInputField};

    fn field(mode: InputMode, value: Option<&str>) -> SetupInputField {
        SetupInputField {
            mode,
            value: value.map(|item| item.to_string()),
        }
    }

    fn random_request() -> CreateRollRequest {
        CreateRollRequest {
            country: field(InputMode::Random, Some("jp")),
            moment: field(InputMode::Random, None),
            place: field(InputMode::Random, None),
            time: field(InputMode::Random, None),
            season: field(InputMode::Random, None),
            weather: field(InputMode::Random, None),
            tiny_detail: field(InputMode::Random, None),
        }
    }

    fn manual_request() -> CreateRollRequest {
        CreateRollRequest {
            country: field(InputMode::Manual, Some("jp")),
            moment: field(InputMode::Manual, Some("commute home")),
            place: field(InputMode::Manual, Some("station platform")),
            time: field(InputMode::Manual, Some("evening")),
            season: field(InputMode::Manual, Some("winter")),
            weather: field(InputMode::Manual, Some("clear")),
            tiny_detail: field(InputMode::Manual, Some("plastic bag")),
        }
    }

    #[test]
    fn random_inputs_score_as_more_accidental_than_manual_inputs() {
        let random_review = evaluate(
            &random_request(),
            "contact_sheet",
            "app_managed",
            "frame.png",
        );
        let manual_review = evaluate(
            &manual_request(),
            "contact_sheet",
            "app_managed",
            "frame.png",
        );

        assert!(random_review.accidental_feeling > manual_review.accidental_feeling);
        assert!(random_review.everyday_life > manual_review.everyday_life);
    }

    #[test]
    fn placeholder_images_increase_ai_feeling_and_imperfection() {
        let request = random_request();
        let remote_review = evaluate(&request, "contact_sheet", "app_managed", "frame.png");
        let placeholder_review = evaluate(&request, "contact_sheet", "app_managed", "frame.svg");

        assert!(placeholder_review.ai_feeling > remote_review.ai_feeling);
        assert!(placeholder_review.imperfection > remote_review.imperfection);
    }
}
