use crate::config::AppConfig;
use crate::error::AppError;
use crate::filter::filter_events;
use crate::normalize::events::normalize_events;
use crate::normalize::priority::select_top_events;
use crate::normalize::render_model::build_render_events;
use crate::publisher::load_telegram_config;
use crate::publisher::telegram_poll::send_events_poll;
use crate::sources::forexfactory::ForexFactorySource;

pub async fn run() -> Result<(), AppError> {
    // 1) Load config (filters, timezone, language, ...)
    let config = AppConfig::load("config/default.yaml")?;
    // 2) Fetch raw calendar JSON (Playwright -> tmp/forexfactory.json)
    let source = ForexFactorySource;
    let json = source.fetch_calendar_json().await?;

    // 3) Parse JSON -> Vec<EconomicEvent>
    let events = source.parse_events_from_json(&json)?;

    if events.is_empty() {
        println!("No economic events found from ForexFactory.");
        return Ok(());
    }

    // Test events scrap is ok
    println!("Total parsed events: {}", events.len());

    for e in &events {
        println!(
            "Event: {:<5} | {:?} | {:?} | {}",
            e.currency, e.impact, e.time, e.title
        );
    }

    // 4) Apply business filters (impact/currency/...)
    let filtered = filter_events(events, &config);

    if filtered.is_empty() {
        println!("No events matched the configured filters.");
        return Ok(());
    }

    println!("Filtered events: {}", filtered.len());

    let normalized = normalize_events(filtered, config.timezone);

    for e in &normalized {
        println!("{:?} | {:?} | {}", e.currency, e.time, e.title);
    }

    let render_events = build_render_events(normalized);

    if render_events.is_empty() {
        println!("No renderable events after deduplication.");
        return Ok(());
    }

    let poll_candidates = select_top_events(render_events, 12);


    println!("🧪 Starting Telegram poll test...");

    // 1️⃣ Load Telegram config (bot + recipient)
    let tg = load_telegram_config()?;

    // 3️⃣ Send poll to Telegram
    send_events_poll(&tg, poll_candidates).await?;

    println!("✅ Poll sent. Vote in Telegram, then stop program manually.");

    // 5) Next steps in pipeline (if already implemented in your project):
    // - Render image from `filtered`
    // - Publish to Telegram
    //

    Ok(())
}
