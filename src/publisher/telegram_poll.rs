use crate::error::AppError;
use crate::normalize::render_model::RenderEvent;
use crate::publisher::TelegramConfig;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use teloxide::prelude::*;
use teloxide::types::InputPollOption;

use crate::state::STATE;

#[derive(Debug, Clone)]
pub struct PollContext {
    /// Telegram poll id (String form of PollId)
    pub poll_id: String,
    /// RenderEvent options in the same order as poll options
    pub options: Vec<RenderEvent>,
    /// Selected option indices (filled by poll_answer handler later)
    pub selected_indices: Vec<usize>,
}

static ACTIVE_POLL: Lazy<Mutex<Option<PollContext>>> = Lazy::new(|| Mutex::new(None));

/// Sends a Telegram poll with candidate economic events.
/// Allows multiple answers and stores poll context in memory.
pub async fn send_events_poll(
    cfg: &TelegramConfig,
    events: Vec<RenderEvent>,
) -> Result<(), AppError> {
    if events.is_empty() {
        return Err(AppError::Publisher(
            "No events available to create poll".into(),
        ));
    }

    // Telegram requires InputPollOption, not String
    let options: Vec<InputPollOption> = events
        .iter()
        .map(|e| InputPollOption::new(format!("{} {} — {}", e.currency, e.title, e.time)))
        .collect();

    let msg = cfg
        .bot
        .send_poll(
            cfg.recipient.clone(),
            "📊 Select today’s events for rendering (max 5)",
            options,
        )
        .is_anonymous(true)
        .allows_multiple_answers(true)
        .send()
        .await
        .map_err(|e| AppError::Publisher(e.to_string()))?;

    // In this teloxide version, poll is accessed via method
    let poll = msg
        .poll()
        .ok_or_else(|| AppError::Publisher("SendPoll response missing poll".into()))?;

    let mut state = STATE.lock().unwrap();

    state.poll = Some(PollContext {
        poll_id: poll.id.to_string(),
        options: events,
        selected_indices: Vec::new(),
    });

    Ok(())
}

pub fn handle_poll_answer(poll_id: &str, option_indices: &[u32]) {
    let mut state = STATE.lock().unwrap();

    let Some(active_poll) = state.poll.as_mut() else {
        return; // no active poll
    };

    if active_poll.poll_id != poll_id {
        return; // old or unrelated poll
    }

    active_poll.selected_indices = option_indices.iter().map(|&i| i as usize).collect();

    println!("Poll updated: selected {:?}", active_poll.selected_indices);
}

pub fn get_selected_events() -> Vec<RenderEvent> {
    let state = STATE.lock().unwrap();

    let Some(active_poll) = state.poll.as_ref() else {
        return Vec::new();
    };

    active_poll
        .selected_indices
        .iter()
        .filter_map(|&idx| active_poll.options.get(idx).cloned())
        .collect()
}

pub fn clear_active_poll() {
    let mut state = STATE.lock().unwrap();
    state.poll = None;
}
