use std::fs;

use tauri::Wry;

use crate::errors::AppError;

use super::ensure_roll_image_dir;

pub fn write_placeholder_frame_svg(
    app: &tauri::AppHandle<Wry>,
    roll_id: i64,
    frame_index: i64,
    country_code: &str,
    roll_status: &str,
) -> Result<(String, String), AppError> {
    let dir = ensure_roll_image_dir(app, roll_id)?;
    let image_path = dir.join(format!("frame-{frame_index}.svg"));
    let thumb_path = dir.join(format!("frame-{frame_index}-thumb.svg"));

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="960" height="960" viewBox="0 0 960 960">
<rect width="960" height="960" fill="#efe2d2"/>
<rect x="48" y="48" width="864" height="864" fill="#f8f0e7" stroke="#7b5b43" stroke-width="6"/>
<text x="88" y="140" font-size="36" fill="#5a4333" font-family="Georgia, serif">FoundFrame</text>
<text x="88" y="214" font-size="72" fill="#241913" font-family="Georgia, serif">Frame {frame_index}</text>
<text x="88" y="292" font-size="28" fill="#6b5648" font-family="Georgia, serif">country: {country_code}</text>
<text x="88" y="338" font-size="28" fill="#6b5648" font-family="Georgia, serif">roll status: {roll_status}</text>
<text x="88" y="384" font-size="28" fill="#6b5648" font-family="Georgia, serif">simulation: local placeholder contact sheet</text>
<rect x="88" y="470" width="784" height="320" fill="#e2d1bf" stroke="#8b6b52" stroke-width="3"/>
<circle cx="230" cy="630" r="62" fill="#b58763"/>
<rect x="372" y="540" width="178" height="178" fill="#c79d78"/>
<path d="M620 730 L820 550 L820 730 Z" fill="#9d7658"/>
</svg>"##
    );

    let thumb_svg = svg.replace("width=\"960\" height=\"960\"", "width=\"320\" height=\"320\"");

    fs::write(&image_path, svg).map_err(|source| AppError::Io {
        context: format!("failed to write placeholder frame svg at {}", image_path.display()),
        source,
    })?;
    fs::write(&thumb_path, thumb_svg).map_err(|source| AppError::Io {
        context: format!("failed to write placeholder thumb svg at {}", thumb_path.display()),
        source,
    })?;

    Ok((image_path.display().to_string(), thumb_path.display().to_string()))
}

pub fn write_placeholder_alternate_take_svg(
    app: &tauri::AppHandle<Wry>,
    roll_id: i64,
    parent_frame_id: i64,
    country_code: &str,
) -> Result<(String, String), AppError> {
    let dir = ensure_roll_image_dir(app, roll_id)?;
    let image_path = dir.join(format!("alternate-take-{parent_frame_id}.svg"));
    let thumb_path = dir.join(format!("alternate-take-{parent_frame_id}-thumb.svg"));

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="960" height="960" viewBox="0 0 960 960">
<rect width="960" height="960" fill="#ead9cb"/>
<rect x="48" y="48" width="864" height="864" fill="#f6ede4" stroke="#715544" stroke-width="6"/>
<text x="88" y="140" font-size="36" fill="#5a4333" font-family="Georgia, serif">FoundFrame Nearby Take</text>
<text x="88" y="214" font-size="64" fill="#241913" font-family="Georgia, serif">Parent Frame {parent_frame_id}</text>
<text x="88" y="292" font-size="28" fill="#6b5648" font-family="Georgia, serif">country: {country_code}</text>
<text x="88" y="338" font-size="28" fill="#6b5648" font-family="Georgia, serif">simulation: local placeholder alternate take</text>
<rect x="88" y="470" width="784" height="320" fill="#d8c0ab" stroke="#8b6b52" stroke-width="3"/>
<circle cx="270" cy="620" r="72" fill="#b27e58"/>
<rect x="450" y="560" width="210" height="150" fill="#c79470"/>
<path d="M680 760 L820 540 L860 760 Z" fill="#9a6f50"/>
</svg>"##
    );

    let thumb_svg = svg.replace("width=\"960\" height=\"960\"", "width=\"320\" height=\"320\"");

    fs::write(&image_path, svg).map_err(|source| AppError::Io {
        context: format!("failed to write alternate take svg at {}", image_path.display()),
        source,
    })?;
    fs::write(&thumb_path, thumb_svg).map_err(|source| AppError::Io {
        context: format!("failed to write alternate take thumb svg at {}", thumb_path.display()),
        source,
    })?;

    Ok((image_path.display().to_string(), thumb_path.display().to_string()))
}
