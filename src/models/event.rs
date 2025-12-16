use super::enums::Impact;

#[derive(Debug)]
pub struct EconomicEvent {
    pub title: String,
    pub currency: String,
    pub impact: Impact,
}
