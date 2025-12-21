use crate::config::AppConfig;
use crate::models::event::EconomicEvent;

pub fn filter_events(
    events: Vec<EconomicEvent>,
    config: &AppConfig,
) -> Vec<EconomicEvent> {
    events
        .into_iter()
        .filter(|event| {
            config.impact.iter().any(|i| i.eq_ignore_ascii_case(&format!("{:?}", event.impact)))
        })
        .filter(|event| {
            config.currency.iter().any(|c| c == &event.currency)
        })
        .collect()
}
