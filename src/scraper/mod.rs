use reqwest::Body;

use crate::error::AppError;
use  crate::models::event::EconomicEvent;
use crate::models::enums::Impact;

pub async fn fetch_events() -> Result<Vec<EconomicEvent>, AppError> {
    let url = "https://example.com"; // فعلاً placeholder

    let body = reqwest::get(url)
        .await
        .map_err(|e| AppError::Scraper(e.to_string()))?
        .text()
        .await
        .map_err(|e| AppError::Scraper(e.to_string()))?;

    // فعلاً داده fake برمی‌گردونیم
    let events = vec![
        EconomicEvent {
            title: "CPI m/m".to_string(),
            currency: "USD".to_string(),
            impact: Impact::High,
        },
        EconomicEvent {
            title: "Unemployment Rate".to_string(),
            currency: "EUR".to_string(),
            impact: Impact::Medium,
        },
    ];

    Ok(events)
}
    
