use chrono::{Local, NaiveDate};

use crate::config::AppConfig;
use crate::error::AppError;
use crate::filter::filter_events;
use crate::normalize::events::normalize_events;
use crate::normalize::render_model::build_render_events;
use crate::sources::forexfactory::ForexFactorySource;
use crate::state::{DailyCache, STATE};

pub async fn execute_get(date: Option<NaiveDate>) -> Result<(), AppError> {
    // 1) Load config
    let config = AppConfig::load("config/default.yaml")?;

    // 2) Fetch raw calendar JSON
    let source = ForexFactorySource;
    let target_date = date.unwrap_or_else(|| Local::now().date_naive());

    let day_slug = if date.is_some() {
        to_forexfactory_slug(target_date)
    } else {
        "today".to_string()
    };
    let json = source.fetch_calendar_json(&day_slug).await?;

    // 3) Parse JSON -> Vec<EconomicEvent>
    let events = source.parse_events_from_json(&json)?;

    if events.is_empty() {
        println!("No economic events found from ForexFactory.");
        return Ok(());
    }

    println!("Total parsed events: {}", events.len());

    // 4) Apply filters
    let filtered = filter_events(events, &config);

    if filtered.is_empty() {
        println!("No events matched the configured filters.");
        return Ok(());
    }

    println!("Filtered events: {}", filtered.len());

    // 5) Normalize (timezone)
    let normalized = normalize_events(filtered, config.timezone);

    // 6) Build render model (dedup etc.)
    let render_events = build_render_events(normalized);

    if render_events.is_empty() {
        println!("No renderable events after deduplication.");
        return Ok(());
    }

    // 7) Store in memory with requested date
    let mut state = STATE.lock().unwrap();
    state.daily_cache = Some(DailyCache {
        date: target_date, // ✅ همون تاریخی که fetch شده
        events: render_events,
    });

    // Invalidate any existing poll
    state.poll = None;

    println!("✅ Daily news fetched and cached for {}", target_date);

    Ok(())
}

use chrono::Datelike;

/// Converts NaiveDate → "dec2.2025"
pub fn to_forexfactory_slug(date: NaiveDate) -> String {
    let month = date.format("%b").to_string().to_lowercase();
    format!("{}{}.{}", month, date.day(), date.year())
}
