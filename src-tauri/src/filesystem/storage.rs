use std::{fs, io::Cursor};

use tauri::Wry;

use crate::errors::AppError;

use super::ensure_roll_image_dir;

const THUMBNAIL_MAX_EDGE: u32 = 384;

fn create_thumbnail_png(bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png).map_err(|error| {
        AppError::Config {
            context: format!("failed to decode generated PNG for thumbnail: {error}"),
        }
    })?;
    let thumbnail = image.thumbnail(THUMBNAIL_MAX_EDGE, THUMBNAIL_MAX_EDGE);
    let mut encoded = Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut encoded, image::ImageFormat::Png)
        .map_err(|error| AppError::Config {
            context: format!("failed to encode generated thumbnail: {error}"),
        })?;

    Ok(encoded.into_inner())
}

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
    let thumbnail_bytes = create_thumbnail_png(bytes)?;
    fs::write(&thumb_path, thumbnail_bytes).map_err(|source| AppError::Io {
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use image::{DynamicImage, GenericImageView, ImageFormat, Rgb, RgbImage};

    use super::create_thumbnail_png;

    #[test]
    fn generated_thumbnail_is_small_and_keeps_aspect_ratio() {
        let source = DynamicImage::ImageRgb8(RgbImage::from_pixel(1024, 512, Rgb([80, 120, 160])));
        let mut source_bytes = Cursor::new(Vec::new());
        source
            .write_to(&mut source_bytes, ImageFormat::Png)
            .expect("encode source image");

        let thumbnail_bytes = create_thumbnail_png(&source_bytes.into_inner()).expect("create thumbnail");
        let thumbnail = image::load_from_memory(&thumbnail_bytes).expect("decode thumbnail");

        assert_eq!(thumbnail.dimensions(), (384, 192));
        assert!(thumbnail_bytes.len() < 1024 * 1024);
    }
}
