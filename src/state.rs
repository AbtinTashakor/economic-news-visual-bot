use chrono::NaiveDate;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::normalize::render_model::RenderEvent;
use crate::publisher::telegram_poll::PollContext;

#[derive(Debug)]
pub struct DailyCache {
    pub date: NaiveDate,
    pub events: Vec<RenderEvent>,
}

#[derive(Debug)]
pub struct AppState {
    pub daily_cache: Option<DailyCache>,
    pub poll: Option<PollContext>,
}

pub static STATE: Lazy<Mutex<AppState>> = Lazy::new(|| {
    Mutex::new(AppState {
        daily_cache: None,
        poll: None,
    })
});
