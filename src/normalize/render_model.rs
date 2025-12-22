use std::collections::HashSet;
use crate::models::{EconomicEvent, EventTime, Impact};

#[derive(Debug, Clone)]
pub struct RenderEvent {
    pub impact: Impact,
    pub title: String,
    pub time: String,
    pub currency: String,
}

pub fn build_render_events(events: Vec<EconomicEvent>) -> Vec<RenderEvent> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    for e in events {
        let key = (
            e.date,
            e.currency.clone(),
            e.title.clone(),     
            time_key(&e.time),
        );

        if !seen.insert(key) {
            continue;
        }

        out.push(RenderEvent {
            impact: e.impact,
            title: e.title,
            currency: e.currency,
            time: format_time(&e.time),
        });
    }

    out
}

fn format_time(time: &EventTime) -> String {
    match time {
        EventTime::Exact(t) => t.format("%H:%M").to_string(),
        EventTime::AllDay => "All Day".to_string(),
        EventTime::Tentative => "Tentative".to_string(),
    }
}

fn time_key(time: &EventTime) -> String {
    match time {
        EventTime::Exact(t) => t.format("%H:%M").to_string(),
        EventTime::AllDay => "ALL_DAY".into(),
        EventTime::Tentative => "TENTATIVE".into(),
    }
}
