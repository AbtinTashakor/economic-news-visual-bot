use crate::config::AppConfig;
use crate::error::AppError;
use crate::scraper;

pub async fn run() -> Result<(), AppError> {
    let config = AppConfig::load("config/default.yaml")?;

    println!("Loaded config: {:?}", config);

    let events = scraper::fetch_events().await?;
    println!("Fetched {} events",events.len());

    Ok(())
}
