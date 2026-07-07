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

Some unimportant things dominate the frame.

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
- cinematic lighting
- intentional nostalgia
- commercial beauty
- portfolio aesthetics

Things that should quietly exist:

- ordinary people
- minor clutter
- visual interruptions
- slightly awkward framing
- meaningless objects
- small primary colors
- everyday movement
- subtle imperfection
- quiet mystery

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

fn base_situation_block(roll_dna: &Value) -> String {
    format!(
        "Country: {}\nMoment: {}\nPlace: {}\nTime: {}\nSeason: {}\nWeather: {}\nTiny Detail: {}",
        country_code(roll_dna),
        resolved_text(roll_dna, "moment_context", "ordinary passing moment"),
        resolved_text(roll_dna, "place_context", "somewhere routine"),
        resolved_text(roll_dna, "time_context", "afternoon"),
        resolved_text(roll_dna, "season_context", "autumn"),
        resolved_text(roll_dna, "weather_context", "cloudy"),
        resolved_text(roll_dna, "tiny_detail_context", "something half noticed"),
    )
}

pub fn prompt_engine_version() -> &'static str {
    PROMPT_ENGINE_VERSION
}

pub fn build_contact_sheet_prompt(roll_dna: &Value, frame_count: usize) -> String {
    format!(
        "{UNIVERSAL_SNAPSHOT_PROMPT}\n\nUser Variables\n{}\n\nGenerate {} square frames from the same roll of film. The situation, country, camera behavior, and ordinary world are shared across the roll. Each frame should differ naturally through timing drift, overlapping people, small focus mistakes, blocked sight lines, and accidental composition changes. Do not make the frames feel like curated variations or cinematic storyboards. They are separate survivals from one mundane roll.",
        base_situation_block(roll_dna),
        frame_count
    )
}

pub fn build_alternate_take_prompt(roll_dna: &Value) -> String {
    format!(
        "{UNIVERSAL_SNAPSHOT_PROMPT}\n\nUser Variables\n{}\n\nGenerate one nearby alternate take from the same roll. It should feel like it happened a moment before or after the chosen frame, with the same people, place, and light, but slightly shifted timing, blocking, subject positions, and camera mistakes. Keep it ordinary and accidental. Do not improve the chosen frame. Do not make it cleaner or more beautiful.",
        base_situation_block(roll_dna)
    )
}
