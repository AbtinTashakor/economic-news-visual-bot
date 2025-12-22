use chrono::NaiveDate;
use crate::models::{Impact, EventTime};

#[derive(Debug, Clone)]
pub struct EconomicEvent {
    pub date: NaiveDate,
    pub time: EventTime,
    pub currency: String,
    pub impact: Impact,
    pub title: String,
}
