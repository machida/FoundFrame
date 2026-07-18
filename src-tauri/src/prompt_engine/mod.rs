use serde::Serialize;
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

Tiny detail policy:

- the tiny detail is a clue, not a required foreground prop
- do not repeat the same cup, bag, hand, tray, bottle, or object in every frame
- the tiny detail may appear in one or two frames, be off to the side, be partly hidden, become a background trace, or be absent
- the roll should vary between place-led, surface-led, object-led, and people-at-a-distance frames

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

#[derive(Clone, Debug, Serialize)]
pub struct FramePlan {
    pub frame_index: usize,
    pub role: &'static str,
    pub fault: &'static str,
    pub tiny_detail_use: &'static str,
    pub aspect: &'static str,
    pub size: &'static str,
}

pub struct FramePrompt {
    pub plan: FramePlan,
    pub prompt: String,
    pub size: String,
}

fn frame_plans(frame_count: usize) -> Vec<FramePlan> {
    let templates = [
        FramePlan {
            frame_index: 0,
            role: "place-led frame with no necessary person; ordinary surface, storefront, wall, pavement, or signage may carry the image",
            fault: "almost too plain; too much empty pavement, wall, counter, sky, or road",
            tiny_detail_use: "absent or only a distant trace",
            aspect: "landscape",
            size: "1536x1024",
        },
        FramePlan {
            frame_index: 1,
            role: "people-at-a-distance frame; people are small and incidental within the location",
            fault: "slightly late timing; people overlap or are partly cut by the edge",
            tiny_detail_use: "not visible",
            aspect: "landscape",
            size: "1536x1024",
        },
        FramePlan {
            frame_index: 2,
            role: "sign, label, shelf, vending machine, menu, poster, number, or window-led frame",
            fault: "sign or surface is sharper than people; framing is a little too close or too loose",
            tiny_detail_use: "may become a surface trace, label, mark, condensation, paper, or edge detail",
            aspect: "portrait",
            size: "1024x1536",
        },
        FramePlan {
            frame_index: 3,
            role: "object-led but not product-like; ordinary table, chair, bagging area, counter, shelf, car interior, or doorway",
            fault: "focus lands on background or surface instead of the apparent object",
            tiny_detail_use: "may appear once, off-center and not as the largest shape",
            aspect: "square",
            size: "1024x1024",
        },
        FramePlan {
            frame_index: 4,
            role: "movement frame; walking, passing traffic, crossing, entering, leaving, or weather movement",
            fault: "mild motion blur from walking or a rushed shutter",
            tiny_detail_use: "blurred, partly hidden, or absent",
            aspect: "landscape",
            size: "1536x1024",
        },
        FramePlan {
            frame_index: 5,
            role: "failed keeper frame; a boring or awkward record that still belongs to the roll",
            fault: "underexposed corner, weak flash falloff, or exposure chosen for the wrong surface",
            tiny_detail_use: "not visible or only at an edge",
            aspect: "square",
            size: "1024x1024",
        },
        FramePlan {
            frame_index: 6,
            role: "edge-person frame; one person may be front-facing, profile, or looking down but remains small/off-center",
            fault: "accidental crop; too much background, ceiling, road, grass, or wall",
            tiny_detail_use: "do not use as a foreground prop",
            aspect: "portrait",
            size: "1024x1536",
        },
        FramePlan {
            frame_index: 7,
            role: "quiet trace frame; after-action evidence, empty chair, receipt, wet mark, shelf gap, sign, road marking, or light patch",
            fault: "nearly no subject; the frame feels like a leftover exposure",
            tiny_detail_use: "may be implied rather than shown",
            aspect: "landscape",
            size: "1536x1024",
        },
    ];

    templates
        .iter()
        .take(frame_count)
        .enumerate()
        .map(|(index, plan)| {
            let mut plan = plan.clone();
            plan.frame_index = index;
            plan
        })
        .collect()
}

fn build_contact_sheet_prompt_with_plan(
    roll_dna: &Value,
    frame_count: usize,
    plan: &FramePlan,
) -> String {
    format!(
        "{UNIVERSAL_SNAPSHOT_PROMPT}\n\nUser Variables\n{}\n\nContact Sheet Policy\nGenerate frame {} of {} from the same roll of film. The situation, country, camera behavior, and ordinary world are shared across the roll, but this individual frame has its own role and failure pattern.\n\nFrame Role: {}\nFrame Fault: {}\nTiny Detail Use: {}\nAspect: {}\n\nThe frame should feel like one exposure from a roll, not a complete set by itself. Let some frames be almost too plain. Do not make a memorable subject mandatory. Do not make this a curated variation, portrait, fashion image, travel image, product photograph, street-photography trophy, street-photography trophies, or cinematic storyboard. If people appear, keep them incidental, off-center, small, partially hidden, or visually interrupted, but do not make everyone face away from the camera and do not make them all turn their backs to the camera. Do not solve person-centered composition by replacing the person with a centered hand, bag, sleeve, cup, tumbler, bottle, tray, or other foreground object. Do not repeat the same tiny detail as a foreground prop across the roll. This frame may be mostly place, signs, shopfronts, shelves, chairs, vending machines, walls, roads, tables, objects, weather, light, empty space, or traces of activity rather than people.",
        base_situation_block(roll_dna),
        plan.frame_index + 1,
        frame_count,
        plan.role,
        plan.fault,
        plan.tiny_detail_use,
        plan.aspect,
    )
}

pub fn build_contact_sheet_frame_prompts(roll_dna: &Value, frame_count: usize) -> Vec<FramePrompt> {
    frame_plans(frame_count)
        .into_iter()
        .map(|plan| FramePrompt {
            prompt: build_contact_sheet_prompt_with_plan(roll_dna, frame_count, &plan),
            size: plan.size.to_string(),
            plan,
        })
        .collect()
}

pub fn build_alternate_take_prompt(roll_dna: &Value) -> String {
    format!(
        "{UNIVERSAL_SNAPSHOT_PROMPT}\n\nUser Variables\n{}\n\nGenerate one nearby alternate take from the same roll. It should feel like it happened a moment before or after the chosen frame, with the same place and light, but slightly shifted timing, edge interruptions, incidental people, object positions, signs, surfaces, empty space, and camera mistakes. Keep it ordinary and accidental. Do not improve the chosen frame. Do not make it cleaner, more centered, more portrait-like, more foreground-obstructed, more colorful, or more beautiful.",
        base_situation_block(roll_dna)
    )
}

#[cfg(test)]
mod tests {
    use super::{build_alternate_take_prompt, build_contact_sheet_frame_prompts};

    fn contact_sheet_prompt_text(roll_dna: &serde_json::Value) -> String {
        build_contact_sheet_frame_prompts(roll_dna, 8)
            .into_iter()
            .map(|frame_prompt| frame_prompt.prompt)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn contact_sheet_prompt_discourages_portrait_like_people() {
        let roll_dna = serde_json::json!({});
        let prompt = contact_sheet_prompt_text(&roll_dna);

        assert!(prompt.contains("centered people"));
        assert!(prompt.contains("centered foreground obstructions"));
        assert!(prompt.contains("portrait-like images"));
        assert!(prompt.contains("If people appear, keep them incidental"));
        assert!(prompt.contains("This frame may be mostly place, signs, shopfronts"));
    }

    #[test]
    fn prompts_discourage_centered_foreground_gimmicks() {
        let roll_dna = serde_json::json!({});
        let contact_sheet_prompt = contact_sheet_prompt_text(&roll_dna);
        let alternate_take_prompt = build_alternate_take_prompt(&roll_dna);

        assert!(contact_sheet_prompt.contains("without becoming the subject"));
        assert!(contact_sheet_prompt
            .contains("do not place a hand, plastic bag, or other meaningless object"));
        assert!(contact_sheet_prompt.contains("Do not solve person-centered composition"));
        assert!(alternate_take_prompt.contains("more foreground-obstructed"));
    }

    #[test]
    fn prompts_discourage_generated_color_and_texture_tells() {
        let roll_dna = serde_json::json!({});
        let contact_sheet_prompt = contact_sheet_prompt_text(&roll_dna);
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
        let prompt = contact_sheet_prompt_text(&roll_dna);

        assert!(prompt.contains("Camera Behavior: cheap plastic lens 35mm"));
        assert!(prompt.contains("Lens Behavior: soft edges low microcontrast"));
        assert!(prompt.contains("Color Response: mild warm or green cast"));
        assert!(prompt.contains("Flash Behavior: small direct flash when light is low"));
    }

    #[test]
    fn prompts_discourage_everyone_facing_away() {
        let roll_dna = serde_json::json!({});
        let prompt = contact_sheet_prompt_text(&roll_dna);

        assert!(prompt.contains("do not make everyone face away"));
        assert!(prompt.contains("do not make them all turn their backs"));
        assert!(prompt.contains("front-facing, three-quarter, profile"));
    }

    #[test]
    fn prompts_allow_place_and_surface_led_frames() {
        let roll_dna = serde_json::json!({});
        let prompt = contact_sheet_prompt_text(&roll_dna);

        assert!(prompt.contains("Everyday surface policy"));
        assert!(prompt.contains("shopfronts, signs, shelves, vending machines"));
        assert!(prompt.contains("plain empty space and distance"));
        assert!(prompt.contains("street-photography trophies"));
    }

    #[test]
    fn prompts_prevent_repeated_tiny_detail_foreground_props() {
        let roll_dna = serde_json::json!({});
        let prompt = contact_sheet_prompt_text(&roll_dna);

        assert!(prompt.contains("Tiny detail policy"));
        assert!(prompt.contains("do not repeat the same cup, bag, hand, tray, bottle, or object"));
        assert!(prompt.contains("Do not repeat the same tiny detail as a foreground prop"));
        assert!(prompt.contains("cup, tumbler, bottle, tray"));
    }

    #[test]
    fn contact_sheet_uses_distinct_frame_roles_faults_and_aspects() {
        let roll_dna = serde_json::json!({});
        let frame_prompts = build_contact_sheet_frame_prompts(&roll_dna, 8);
        let sizes = frame_prompts
            .iter()
            .map(|frame_prompt| frame_prompt.size.as_str())
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(frame_prompts.len(), 8);
        assert!(sizes.contains("1536x1024"));
        assert!(sizes.contains("1024x1536"));
        assert!(sizes.contains("1024x1024"));
        assert!(frame_prompts
            .iter()
            .any(|frame_prompt| frame_prompt.plan.role.contains("failed keeper")));
        assert!(frame_prompts
            .iter()
            .any(|frame_prompt| frame_prompt.plan.fault.contains("motion blur")));
        assert!(frame_prompts
            .iter()
            .any(|frame_prompt| frame_prompt.plan.tiny_detail_use.contains("absent")));
    }
}
