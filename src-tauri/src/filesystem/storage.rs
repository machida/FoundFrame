use std::fs;

use tauri::Wry;

use crate::errors::AppError;

use super::ensure_roll_image_dir;

pub fn write_generated_png(
    app: &tauri::AppHandle<Wry>,
    roll_id: i64,
    filename: &str,
    bytes: &[u8],
) -> Result<(String, String), AppError> {
    let dir = ensure_roll_image_dir(app, roll_id)?;
    let image_path = dir.join(format!("{filename}.png"));
    let thumb_path = dir.join(format!("{filename}-thumb.png"));

    fs::write(&image_path, bytes).map_err(|source| AppError::Io {
        context: format!("failed to write generated image at {}", image_path.display()),
        source,
    })?;
    fs::write(&thumb_path, bytes).map_err(|source| AppError::Io {
        context: format!("failed to write generated thumbnail at {}", thumb_path.display()),
        source,
    })?;

    Ok((image_path.display().to_string(), thumb_path.display().to_string()))
}

pub fn read_image_bytes(image_path: &str) -> Result<Vec<u8>, AppError> {
    fs::read(image_path).map_err(|source| AppError::Io {
        context: format!("failed to read image bytes from {image_path}"),
        source,
    })
}
