use chrono::NaiveTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Impact {
    Low,
    Medium,
    High,
    Holiday,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventTime {
    Exact(NaiveTime),
    AllDay,
    Tentative,
}
