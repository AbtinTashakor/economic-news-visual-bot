use crate::normalize::render_model::RenderEvent;
use crate::models::Impact;

/// Selects top N events based on priority rules.
///
/// Priority rules:
/// 1. Impact is the primary factor (High > Medium > Low > None/Holiday)
/// 2. Within the same impact level, USD events are prioritized
/// 3. If still equal, earlier time comes first
pub fn select_top_events(
    mut events: Vec<RenderEvent>,
    limit: usize,
) -> Vec<RenderEvent> {
    events.sort_by(|a, b| {
        let score_a = priority_score(a);
        let score_b = priority_score(b);

        score_b
            .cmp(&score_a) // higher score first
            .then_with(|| a.time.cmp(&b.time)) // earlier time first
    });

    events.truncate(limit);
    events
}

fn priority_score(event: &RenderEvent) -> i32 {
    impact_score(&event.impact) + currency_score(&event.currency)
}

fn impact_score(impact: &Impact) -> i32 {
    match impact {
        Impact::High => 300,
        Impact::Medium => 200,
        Impact::Low => 100,
        Impact::None | Impact::Holiday => 0,
    }
}

fn currency_score(currency: &str) -> i32 {
    if currency == "USD" { 10 } else { 0 }
}
