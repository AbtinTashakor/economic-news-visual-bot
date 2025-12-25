use crate::error::AppError;
use crate::models::EventTime;
use crate::models::Impact;
use crate::models::event::EconomicEvent;
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;
use std::{fs, process::Command};


/// ForexFactory source implemented via Playwright (headless Chromium).
/// The Playwright script writes JSON to: tmp/forexfactory.json
pub struct ForexFactorySource;

impl ForexFactorySource {
    /// Runs the Playwright script and returns the raw JSON string saved in tmp/forexfactory.json
    pub async fn fetch_calendar_json(&self, day_slug: &str) -> Result<String, AppError> {
        let status = Command::new("node")
            .arg("scripts/forexfactory_fetch.js")
            .arg(day_slug)
            .status()
            .map_err(|e| AppError::Scraper(format!("Failed to run node: {e}")))?;

        if !status.success() {
            return Err(AppError::Scraper(format!(
                "Playwright script failed with status: {status}"
            )));
        }

        let json = fs::read_to_string("tmp/forexfactory.json")
            .map_err(|e| AppError::Scraper(format!("Failed to read tmp/forexfactory.json: {e}")))?;

        Ok(json)
    }

    /// Parses raw ForexFactory calendar JSON into domain events.
    pub fn parse_events_from_json(&self, json: &str) -> Result<Vec<EconomicEvent>, AppError> {
        let state: CalendarState = serde_json::from_str(json)
            .map_err(|e| AppError::Scraper(format!("serde_json error: {e}")))?;

        Ok(calendar_state_to_events(state))
    }

    // pub fn to_forexfactory_slug(date: NaiveDate) -> String {
    //     let month = date.format("%b").to_string().to_lowercase();
    //     format!("{}{}.{}", month, date.day(), date.year())
    // }
}

/* ----------------------------- DTOs (site JSON) ----------------------------- */

#[derive(Debug, Deserialize)]
struct CalendarState {
    #[serde(default)]
    days: Vec<Day>,
}

#[derive(Debug, Deserialize)]
struct Day {
    #[serde(default)]
    events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Event {
    // Example: "Dec 22, 2025"
    #[serde(default)]
    date: String,

    // Example: "2:00am" / "All Day" / "Tentative"
    #[serde(default)]
    timeLabel: String,

    #[serde(default)]
    currency: String,

    // Example: "high" / "medium" / "low" / ""
    #[serde(default)]
    impactName: String,

    // Title fields may differ; keep both and pick best
    #[serde(default)]
    name: String,

    #[serde(default)]
    soloTitle: String,
}

/* -------------------------- DTO -> Domain conversion ------------------------- */

fn calendar_state_to_events(state: CalendarState) -> Vec<EconomicEvent> {
    let mut out = Vec::new();

    for day in state.days {
        for e in day.events {
            // Parse date
            let date = match NaiveDate::parse_from_str(e.date.trim(), "%b %d, %Y") {
                Ok(d) => d,
                Err(_) => continue,
            };

            // Pick best title
            let title = pick_title(&e);
            if title.is_empty() {
                continue;
            }

            let currency = e.currency.trim().to_string();
            if currency.is_empty() {
                continue;
            }

            let time = parse_event_time(&e.timeLabel);
            let impact = map_impact_name(e.impactName.trim());

            out.push(EconomicEvent {
                date,
                time,
                currency,
                impact,
                title,
            });
        }
    }

    out
}

fn pick_title(e: &Event) -> String {
    let st = e.soloTitle.trim();
    if !st.is_empty() {
        return st.to_string();
    }
    e.name.trim().to_string()
}

fn map_impact_name(name: &str) -> Impact {
    match name {
        "high" => Impact::High,
        "medium" => Impact::Medium,
        "low" => Impact::Low,
        "holiday" => Impact::Holiday,
        _ => Impact::None,
    }
}

/// ForexFactory timeLabel examples:
/// - "2:00am"
/// - "All Day"
/// - "Tentative"

fn parse_event_time(label: &str) -> EventTime {
    let s = label.trim().to_lowercase();

    if s.contains("all day") {
        return EventTime::AllDay;
    }

    if s.contains("tentative") {
        return EventTime::Tentative;
    }

    match NaiveTime::parse_from_str(&s, "%I:%M%p") {
        Ok(t) => EventTime::Exact(t),
        Err(_) => EventTime::Tentative,
    }
}
