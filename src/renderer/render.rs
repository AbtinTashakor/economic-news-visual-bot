use std::path::Path;

use image::DynamicImage;

use crate::error::AppError;
use crate::models::Impact;

use image::ImageFormat;
use std::io::Cursor;

use super::{
    assets::RenderAssets,
    draw::{draw_icon, draw_text},
    layout::Layout,
    types::{RenderEvent, RenderMeta},
};

pub fn render_png(
    assets_dir: &Path,
    meta: RenderMeta,
    events: &[RenderEvent],
) -> Result<Vec<u8>, AppError> {
    let assets = RenderAssets::load(assets_dir)?;
    let layout = Layout::default();

    let mut canvas: DynamicImage = assets.template.clone();

    // --- Header ---
    draw_text(
        &mut canvas,
        &meta.day_name,
        layout.header_day_pos,
        80.0,
        image::Rgba([255, 255, 255, 255]),
    )?;

    draw_text(
        &mut canvas,
        &format!("{}", meta.date_label),
        layout.header_date_pos,
        36.0,
        image::Rgba([255, 255, 255, 255]),
    )?;

    // --- Events ---
    for (idx, event) in events.iter().enumerate() {
        let y = layout.first_row_y + idx as u32 * layout.row_height;

        let icon = match event.impact {
            Impact::High => &assets.impact_high,
            Impact::Medium => &assets.impact_medium,
            _ => &assets.impact_low,
        };

        draw_icon(&mut canvas, icon, (layout.impact_icon_pos_x, y));

        draw_text(
            &mut canvas,
            &event.title,
            (layout.title_pos_x, y),
            28.0,
            layout.title_color,
        )?;

        let right_text = format!("{} | {}", event.currency, event.time_label);

        draw_text(
            &mut canvas,
            &right_text,
            (layout.right_text_pos_x, y),
            26.0,
            layout.meta_color,
        )?;
    }

    // --- Encode PNG ---
    let mut out = Cursor::new(Vec::new());

    canvas
        .write_to(&mut out, ImageFormat::Png)
        .map_err(|e| AppError::Image(format!("failed to encode png: {}", e)))?;

    Ok(out.into_inner())
}
