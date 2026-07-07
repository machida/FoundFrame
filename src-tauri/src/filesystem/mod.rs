use std::path::PathBuf;

use tauri::Wry;

use crate::errors::AppError;
use crate::persistence::database;

mod placeholders;
mod storage;

pub fn ensure_roll_image_dir(
    app: &tauri::AppHandle<Wry>,
    roll_id: i64,
) -> Result<PathBuf, AppError> {
    let dir = database::ensure_app_data_dir(app)?.join("images").join(format!("roll-{roll_id}"));

    std::fs::create_dir_all(&dir).map_err(|source| AppError::Io {
        context: format!("failed to create roll image dir at {}", dir.display()),
        source,
    })?;

    Ok(dir)
}

pub use placeholders::{write_placeholder_alternate_take_svg, write_placeholder_frame_svg};
pub use storage::{read_image_bytes, write_generated_png};
