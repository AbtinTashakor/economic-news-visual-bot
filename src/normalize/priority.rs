use crate::normalize::render_model::RenderEvent;
use crate::models::Impact;

/// Selects top N events based on priority rules.
/// Priority order:
/// 1. USD events first
/// 2. Higher impact
/// 3. Earlier time (lexicographical, since already normalized)
pub fn select_top_events(
    mut events: Vec<RenderEvent>,
    limit: usize,
) -> Vec<RenderEvent> {
    events.sort_by(|a, b| {
        let score_a = priority_score(a);
        let score_b = priority_score(b);

        // Descending by score
        score_b.cmp(&score_a)
            // If equal score, earlier time first
            .then_with(|| a.time.cmp(&b.time))
    });

    events.truncate(limit);
    events
}

fn priority_score(event: &RenderEvent) -> i32 {
    currency_score(&event.currency) + impact_score(&event.impact)
}

fn currency_score(currency: &str) -> i32 {
    if currency == "USD" { 10 } else { 0 }
}

fn impact_score(impact: &Impact) -> i32 {
    match impact {
        Impact::High => 3,
        Impact::Medium => 2,
        Impact::Low => 1,
        Impact::None | Impact::Holiday => 0,
    }
}
