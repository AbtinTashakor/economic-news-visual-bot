use crate::config::AppConfig;
use crate::error::AppError;
use crate::filter::filter_events;
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
        println!("Event: {:<5} | {:?} | {}", e.currency, e.impact, e.title);
    }

    // 4) Apply business filters (impact/currency/...)
    let filtered = filter_events(events, &config);

    if filtered.is_empty() {
        println!("No events matched the configured filters.");
        return Ok(());
    }

    println!("Filtered events: {}", filtered.len());

    // 5) Next steps in pipeline (if already implemented in your project):
    // - Render image from `filtered`
    // - Publish to Telegram
    //


    Ok(())
}
