use crate::models::{EconomicEvent, EventTime};
use crate::config::TargetTimezone;
use chrono::{NaiveDateTime, FixedOffset};
use chrono::TimeZone;


pub fn normalize_events(
    mut events: Vec<EconomicEvent>,
    target_tz: TargetTimezone,
) -> Vec<EconomicEvent> {
    // 1) timezone conversion (only Exact)
    for e in &mut events {
        if let EventTime::Exact(t) = e.time {
            let ny_offset = FixedOffset::west_opt(5 * 3600).unwrap();   // UTC-5 (NY)
            let ir_offset = FixedOffset::east_opt(3 * 3600 + 1800).unwrap(); // UTC+3:30

            let dt_ny = NaiveDateTime::new(e.date, t);
            let dt_ny = ny_offset.from_local_datetime(&dt_ny).unwrap();

            let converted = match target_tz {
                TargetTimezone::NY => dt_ny,
                TargetTimezone::IR => dt_ny.with_timezone(&ir_offset),
            };

            e.date = converted.date_naive();
            e.time = EventTime::Exact(converted.time());
        }
    }

    // 2) sort
    events.sort_by(|a, b| compare_events(a, b));

    events
}


fn compare_events(a: &EconomicEvent, b: &EconomicEvent) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;

    // اول date
    match a.date.cmp(&b.date) {
        Equal => {}
        ord => return ord,
    }

    match (&a.time, &b.time) {
        // Exact ها اول
        (EventTime::Exact(t1), EventTime::Exact(t2)) => t1.cmp(t2),
        (EventTime::Exact(_), _) => Less,
        (_, EventTime::Exact(_)) => Greater,

        // Tentative بعد Exact
        (EventTime::Tentative, EventTime::Tentative) => Equal,
        (EventTime::Tentative, EventTime::AllDay) => Less,
        (EventTime::AllDay, EventTime::Tentative) => Greater,

        // AllDay آخر
        (EventTime::AllDay, EventTime::AllDay) => Equal,
    }
}
