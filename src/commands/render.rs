use chrono::Datelike;
use std::path::Path;

use crate::error::AppError;
use crate::normalize::render_model::RenderEvent as ModelEvent;
use crate::publisher::load_telegram_config;
use crate::publisher::telegram::send_image;
use crate::publisher::telegram_poll::get_selected_events;
use crate::renderer::{
    render::render_png,
    types::{RenderEvent as ImgEvent, RenderMeta},
};
use crate::state::STATE;

pub async fn execute_render() -> Result<(), AppError> {
    // 1️⃣ انتخاب رویدادها (poll یا fallback)
    let mut events: Vec<ModelEvent> = get_selected_events();

    if events.is_empty() {
        let state = STATE.lock().unwrap();
        let cache = state
            .daily_cache
            .as_ref()
            .ok_or_else(|| AppError::Publisher("no daily cache found, run /get first".into()))?;
        events = cache.events.clone();
    }

    if events.is_empty() {
        return Err(AppError::Publisher("no events available to render".into()));
    }

    // 2️⃣ محدود به 6 خبر
    events.truncate(6);

    // 3️⃣ meta از تاریخ
    let meta = {
        let state = STATE.lock().unwrap();
        let cache = state.daily_cache.as_ref().unwrap();
        build_render_meta(cache.date)
    };

    // 4️⃣ تبدیل به ورودی renderer
    let render_events: Vec<ImgEvent> = events.into_iter().map(model_to_img_event).collect();

    // 5️⃣ تولید PNG (bytes)
    let png = render_png(Path::new("assets"), meta, &render_events)?;

    // 6️⃣ ذخیره موقت روی دیسک
    let out_path = format!("tmp/render_{}.png", chrono::Utc::now().timestamp());

    std::fs::write(&out_path, png)
        .map_err(|e| AppError::Image(format!("failed to write render image: {}", e)))?;

    // 7️⃣ ارسال به تلگرام (📌 مسیر فایل)

    // 3️⃣ Load Telegram config
    let tg = load_telegram_config()?;

    send_image(&tg, &out_path, "Economic Calendar").await?;

    Ok(())
}

fn model_to_img_event(e: ModelEvent) -> ImgEvent {
    ImgEvent {
        title: e.title,
        currency: e.currency,
        time_label: e.time,
        impact: e.impact,
    }
}

fn build_render_meta(date: chrono::NaiveDate) -> RenderMeta {
    RenderMeta {
        day_name: date.format("%A").to_string(),
        date_label: date.format("%B %d").to_string(),
    }
}
