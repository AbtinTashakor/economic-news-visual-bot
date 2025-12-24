use std::path::Path;

use ab_glyph::{FontArc, PxScale};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;

use crate::error::AppError;

pub fn draw_text(
    image: &mut DynamicImage,
    text: &str,
    position: (u32, u32),
    font_size: f32,
    color: Rgba<u8>,
) -> Result<(), AppError> {
    // مسیر فونت (طبق قراردادی که گفتی داخل پروژه می‌ذاری)
    let font_path = Path::new("assets/fonts/Inter-Regular.ttf");

    let font_data = std::fs::read(font_path)
        .map_err(|e| AppError::Image(format!("failed to read font {:?}: {}", font_path, e)))?;

    let font = FontArc::try_from_vec(font_data)
        .map_err(|e| AppError::Image(format!("failed to parse font {:?}: {}", font_path, e)))?;

    let scale = PxScale::from(font_size);

    draw_text_mut(
        image,
        color,
        position.0 as i32,
        position.1 as i32,
        scale,
        &font,
        text,
    );

    Ok(())
}

pub fn draw_icon(
    image: &mut DynamicImage,
    icon: &DynamicImage,
    position: (u32, u32),
) {
    image::imageops::overlay(image, icon, position.0.into(), position.1.into());
}

