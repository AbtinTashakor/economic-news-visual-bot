use crate::models::Impact;



#[derive(Debug, Clone)]
pub struct RenderMeta {
    pub day_name: String,    // "Monday"
    pub date_label: String,  // "December 15"
}

#[derive(Debug, Clone)]
pub struct RenderEvent {
    pub title: String,
    pub currency: String,
    pub time_label: String, // "08:30" | "All Day" | "Tentative"
    pub impact: Impact,
}
