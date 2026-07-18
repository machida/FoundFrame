use std::{fs, io::Cursor};

use tauri::Wry;

use crate::errors::AppError;

use super::ensure_roll_image_dir;

const THUMBNAIL_MAX_EDGE: u32 = 384;

fn analog_soften_png(bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let image =
        image::load_from_memory_with_format(bytes, image::ImageFormat::Png).map_err(|error| {
            AppError::Config {
                context: format!("failed to decode generated PNG for analog softening: {error}"),
            }
        })?;
    let mut softened = image.blur(0.35).to_rgba8();
    let (width, height) = softened.dimensions();
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_distance = (center_x.powi(2) + center_y.powi(2)).sqrt().max(1.0);

    for (x, y, pixel) in softened.enumerate_pixels_mut() {
        let distance = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
        let edge = (distance / max_distance).clamp(0.0, 1.0);
        let vignette = 1.0 - edge.powi(2) * 0.08;
        let noise_seed = (x as i32 * 17 + y as i32 * 31 + (x as i32 ^ y as i32) * 7) % 9 - 4;
        for channel in 0..3 {
            let value = f32::from(pixel[channel]);
            let lifted_black = value * 0.985 + 3.0;
            let noisy = lifted_black + noise_seed as f32;
            pixel[channel] = noisy.mul_add(vignette, 0.0).round().clamp(0.0, 255.0) as u8;
        }
    }

    let mut encoded = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(softened)
        .write_to(&mut encoded, image::ImageFormat::Png)
        .map_err(|error| AppError::Config {
            context: format!("failed to encode softened generated PNG: {error}"),
        })?;

    Ok(encoded.into_inner())
}

fn create_thumbnail_png(bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let image =
        image::load_from_memory_with_format(bytes, image::ImageFormat::Png).map_err(|error| {
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
    let display_bytes = analog_soften_png(bytes)?;

    fs::write(&image_path, &display_bytes).map_err(|source| AppError::Io {
        context: format!(
            "failed to write generated image at {}",
            image_path.display()
        ),
        source,
    })?;
    let thumbnail_bytes = create_thumbnail_png(&display_bytes)?;
    fs::write(&thumb_path, thumbnail_bytes).map_err(|source| AppError::Io {
        context: format!(
            "failed to write generated thumbnail at {}",
            thumb_path.display()
        ),
        source,
    })?;

    Ok((
        image_path.display().to_string(),
        thumb_path.display().to_string(),
    ))
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

    use super::{analog_soften_png, create_thumbnail_png};

    #[test]
    fn generated_thumbnail_is_small_and_keeps_aspect_ratio() {
        let source = DynamicImage::ImageRgb8(RgbImage::from_pixel(1024, 512, Rgb([80, 120, 160])));
        let mut source_bytes = Cursor::new(Vec::new());
        source
            .write_to(&mut source_bytes, ImageFormat::Png)
            .expect("encode source image");

        let thumbnail_bytes =
            create_thumbnail_png(&source_bytes.into_inner()).expect("create thumbnail");
        let thumbnail = image::load_from_memory(&thumbnail_bytes).expect("decode thumbnail");

        assert_eq!(thumbnail.dimensions(), (384, 192));
        assert!(thumbnail_bytes.len() < 1024 * 1024);
    }

    #[test]
    fn analog_softening_keeps_png_decodable_and_dimensions() {
        let source = DynamicImage::ImageRgb8(RgbImage::from_pixel(640, 480, Rgb([80, 120, 160])));
        let mut source_bytes = Cursor::new(Vec::new());
        source
            .write_to(&mut source_bytes, ImageFormat::Png)
            .expect("encode source image");

        let softened_bytes = analog_soften_png(&source_bytes.into_inner()).expect("soften source");
        let softened = image::load_from_memory(&softened_bytes).expect("decode softened");

        assert_eq!(softened.dimensions(), (640, 480));
    }
}
