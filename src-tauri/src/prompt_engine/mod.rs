use serde_json::Value;

const PROMPT_ENGINE_VERSION: &str = "universal-snapshot-v25";

const UNIVERSAL_SNAPSHOT_PROMPT: &str = r#"This image is not created.

It is the unavoidable result of one ordinary person's day.

The world existed first.

The photograph happened afterwards.

The photographer:

- was doing something else
- carried an ordinary automatic compact camera
- did not expect to make a memorable photograph
- had less than one second to react
- never took a second frame
- never checked the result

The camera:

- focused automatically
- exposed automatically
- decided white balance automatically
- occasionally made one small mistake

The photographer accepted every decision.

The world never cooperates.

Nobody poses.

Nobody notices the camera.

Traffic continues.

People overlap.

Objects block each other.

Some important things are outside the frame.

Some unimportant things interrupt the frame without becoming the subject.

The photograph survives only because it quietly remained interesting years later.

Not because it was successful.

Not because it was beautiful.

Not because it was technically impressive.

Never create:

- perfect framing
- perfect timing
- perfect focus
- perfect exposure
- balanced composition
- hero subjects
- centered people
- centered foreground obstructions
- foreground hands as the main subject
- shopping bags or plastic bags as the main subject
- portrait-like images
- model-like faces or bodies
- clear main characters
- cinematic lighting
- cinematic color grading
- teal-orange grading
- glossy digital texture
- painterly texture
- intentional nostalgia
- artificial film grain
- commercial beauty
- portfolio aesthetics

Things that should quietly exist:

- ordinary people only as part of the environment
- minor clutter
- visual interruptions
- slightly awkward framing
- meaningless objects
- shopfronts, signs, shelves, vending machines, chairs, walls, windows, traffic markings, and small public surfaces
- large areas of ordinary pavement, sky, wall, counter, road, grass, or water
- small primary colors
- everyday movement
- subtle imperfection
- quiet mystery

People policy:

- people may appear, but they are not the point of the photograph
- faces may be front-facing, three-quarter, profile, looking down, partly blocked, small, soft, or outside the frame
- do not make everyone face away from the camera
- avoid a repeated pattern of backs of heads, turned backs, and people walking away
- no person should occupy the center as a clean subject
- no person should feel cast, styled, posed, or emotionally directed
- the frame may work even if no person is visible

Everyday surface policy:

- a frame may be led by a place, surface, sign, shelf, chair, vending machine, storefront, window, table, street marking, wall, number, or ordinary object
- keep local text, signs, price boards, posters, labels, stickers, menus, packaging, and street furniture visible when they naturally belong
- allow plain empty space and distance; the subject does not need to fill the frame
- many successful frames can feel like someone noticed a small public surface, not a person
- avoid turning these objects into clean product photography, graphic design studies, or symmetrical catalog shots

Color and texture policy:

- use ordinary automatic-camera color, not a designed palette
- keep whites, shadows, skin, concrete, plastic, metal, and fabric mundane
- allow mixed indoor light, weak flash, mild blur, sensor noise, and uneven exposure
- avoid smooth AI skin, waxy surfaces, hyper-detailed eyes, synthetic bokeh, and HDR clarity
- avoid the specific look of generated photography, fashion editorials, travel advertising, or movie stills

Obstruction policy:

- interruptions should feel incidental, not like a visual gimmick
- a hand, sleeve, bag, strap, umbrella, cup, or reflection may clip an edge or cross a corner
- do not place a hand, plastic bag, or other meaningless object in the middle as the largest readable shape
- blocked sight lines should reveal more of the ordinary place, not replace it with a centered obstruction
- at least most of the frame should still be the mundane location, weather, light, and traces of activity

If this image looks intentionally photographed, reject it.

If this image looks intentionally beautiful, reject it.

If this image looks intentionally nostalgic, reject it.

If this image feels like a fragment of someone's life that happened to survive, accept it."#;

fn resolved_text(roll_dna: &Value, key: &str, fallback: &str) -> String {
    roll_dna
        .get(key)
        .and_then(|node| node.get("resolved_value"))
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(fallback)
        .to_string()
}

fn country_code(roll_dna: &Value) -> String {
    roll_dna
        .get("country_context")
        .and_then(|node| node.get("code"))
        .and_then(Value::as_str)
        .unwrap_or("jp")
        .to_string()
}

fn camera_text(roll_dna: &Value, key: &str, fallback: &str) -> String {
    roll_dna
        .get("camera_profile")
        .and_then(|node| node.get(key))
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(fallback)
        .replace('_', " ")
}

fn base_situation_block(roll_dna: &Value) -> String {
    format!(
        "Country: {}\nMoment: {}\nPlace: {}\nTime: {}\nSeason: {}\nWeather: {}\nTiny Detail: {}\nCamera Behavior: {}\nLens Behavior: {}\nColor Response: {}\nFlash Behavior: {}",
        country_code(roll_dna),
        resolved_text(roll_dna, "moment_context", "ordinary passing moment"),
        resolved_text(roll_dna, "place_context", "somewhere routine"),
        resolved_text(roll_dna, "time_context", "afternoon"),
        resolved_text(roll_dna, "season_context", "autumn"),
        resolved_text(roll_dna, "weather_context", "cloudy"),
        resolved_text(roll_dna, "tiny_detail_context", "something half noticed"),
        camera_text(roll_dna, "family", "consumer point and shoot"),
        camera_text(roll_dna, "lens_behavior", "modest center sharpness soft edges"),
        camera_text(roll_dna, "color_response", "plain consumer color"),
        camera_text(roll_dna, "flash_behavior", "available light or weak auto flash"),
    )
}

pub fn prompt_engine_version() -> &'static str {
    PROMPT_ENGINE_VERSION
}

pub fn build_contact_sheet_prompt(roll_dna: &Value, frame_count: usize) -> String {
    format!(
        "{UNIVERSAL_SNAPSHOT_PROMPT}\n\nUser Variables\n{}\n\nGenerate {} square frames from the same roll of film. The situation, country, camera behavior, and ordinary world are shared across the roll. Each frame should differ naturally through timing drift, overlapping people, small focus mistakes, edge interruptions, partial blocked sight lines, distance, empty areas, and accidental composition changes. Do not make the frames feel like curated variations, portraits, fashion images, travel images, product photography, street-photography trophies, or cinematic storyboards. If people appear, keep them incidental, off-center, small, partially hidden, or visually interrupted, but do not make them all turn their backs to the camera. Do not solve person-centered composition by replacing the person with a centered hand, bag, sleeve, or other foreground object. Several frames may be mostly place, signs, shopfronts, shelves, chairs, vending machines, walls, roads, tables, objects, weather, light, or traces of activity rather than people. They are separate survivals from one mundane roll.",
        base_situation_block(roll_dna),
        frame_count
    )
}

pub fn build_alternate_take_prompt(roll_dna: &Value) -> String {
    format!(
        "{UNIVERSAL_SNAPSHOT_PROMPT}\n\nUser Variables\n{}\n\nGenerate one nearby alternate take from the same roll. It should feel like it happened a moment before or after the chosen frame, with the same place and light, but slightly shifted timing, edge interruptions, incidental people, object positions, signs, surfaces, empty space, and camera mistakes. Keep it ordinary and accidental. Do not improve the chosen frame. Do not make it cleaner, more centered, more portrait-like, more foreground-obstructed, more colorful, or more beautiful.",
        base_situation_block(roll_dna)
    )
}

#[cfg(test)]
mod tests {
    use super::{build_alternate_take_prompt, build_contact_sheet_prompt};

    #[test]
    fn contact_sheet_prompt_discourages_portrait_like_people() {
        let roll_dna = serde_json::json!({});
        let prompt = build_contact_sheet_prompt(&roll_dna, 8);

        assert!(prompt.contains("centered people"));
        assert!(prompt.contains("centered foreground obstructions"));
        assert!(prompt.contains("portrait-like images"));
        assert!(prompt.contains("If people appear, keep them incidental"));
        assert!(prompt.contains("Several frames may be mostly place, signs, shopfronts"));
    }

    #[test]
    fn prompts_discourage_centered_foreground_gimmicks() {
        let roll_dna = serde_json::json!({});
        let contact_sheet_prompt = build_contact_sheet_prompt(&roll_dna, 8);
        let alternate_take_prompt = build_alternate_take_prompt(&roll_dna);

        assert!(contact_sheet_prompt.contains("without becoming the subject"));
        assert!(contact_sheet_prompt.contains("do not place a hand, plastic bag, or other meaningless object"));
        assert!(contact_sheet_prompt.contains("Do not solve person-centered composition"));
        assert!(alternate_take_prompt.contains("more foreground-obstructed"));
    }

    #[test]
    fn prompts_discourage_generated_color_and_texture_tells() {
        let roll_dna = serde_json::json!({});
        let contact_sheet_prompt = build_contact_sheet_prompt(&roll_dna, 8);
        let alternate_take_prompt = build_alternate_take_prompt(&roll_dna);

        assert!(contact_sheet_prompt.contains("cinematic color grading"));
        assert!(contact_sheet_prompt.contains("smooth AI skin"));
        assert!(contact_sheet_prompt.contains("HDR clarity"));
        assert!(alternate_take_prompt.contains("more colorful"));
        assert!(alternate_take_prompt.contains("more portrait-like"));
    }

    #[test]
    fn prompts_translate_camera_profile_into_capture_behavior() {
        let roll_dna = serde_json::json!({
            "camera_profile": {
                "family": "cheap_plastic_lens_35mm",
                "lens_behavior": "soft_edges_low_microcontrast",
                "color_response": "mild_warm_or_green_cast",
                "flash_behavior": "small_direct_flash_when_light_is_low"
            }
        });
        let prompt = build_contact_sheet_prompt(&roll_dna, 8);

        assert!(prompt.contains("Camera Behavior: cheap plastic lens 35mm"));
        assert!(prompt.contains("Lens Behavior: soft edges low microcontrast"));
        assert!(prompt.contains("Color Response: mild warm or green cast"));
        assert!(prompt.contains("Flash Behavior: small direct flash when light is low"));
    }

    #[test]
    fn prompts_discourage_everyone_facing_away() {
        let roll_dna = serde_json::json!({});
        let prompt = build_contact_sheet_prompt(&roll_dna, 8);

        assert!(prompt.contains("do not make everyone face away"));
        assert!(prompt.contains("do not make them all turn their backs"));
        assert!(prompt.contains("front-facing, three-quarter, profile"));
    }

    #[test]
    fn prompts_allow_place_and_surface_led_frames() {
        let roll_dna = serde_json::json!({});
        let prompt = build_contact_sheet_prompt(&roll_dna, 8);

        assert!(prompt.contains("Everyday surface policy"));
        assert!(prompt.contains("shopfronts, signs, shelves, vending machines"));
        assert!(prompt.contains("plain empty space and distance"));
        assert!(prompt.contains("street-photography trophies"));
    }
}
