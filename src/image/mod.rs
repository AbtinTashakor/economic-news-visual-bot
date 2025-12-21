use crate::error::AppError;
use crate::models::event::EconomicEvent;

use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontArc, PxScale};

use std::fs;

pub fn render_image(
    events: &[EconomicEvent],
    output_path: &str,
) -> Result<(), AppError> {
    // 1) ساخت تصویر پایه
    let width = 800;
    let height = 400;
    let mut img = RgbaImage::from_pixel(
        width,
        height,
        Rgba([20, 20, 20, 255]),
    );

    // 2) لود فونت (ab_glyph)
    let font_data = fs::read("assets/fonts/DejaVuSans.ttf")
        .map_err(|e| AppError::Image(e.to_string()))?;

    let font = FontArc::try_from_vec(font_data)
        .map_err(|_| AppError::Image("Failed to load font".into()))?;

    let scale = PxScale::from(28.0);

    // 3) عنوان
    draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        20,
        20,
        scale,
        &font,
        "Economic Events",
    );

    // 4) لیست خبرها
    let mut y = 80;
    for event in events {
        let line = format!(
            "{} | {} | {:?}",
            event.currency, event.title, event.impact
        );

        draw_text_mut(
            &mut img,
            Rgba([200, 200, 200, 255]),
            20,
            y,
            PxScale::from(22.0),
            &font,
            &line,
        );

        y += 40;
    }

    // 5) ذخیره
    img.save(output_path)
        .map_err(|e| AppError::Image(e.to_string()))?;

    Ok(())
}
