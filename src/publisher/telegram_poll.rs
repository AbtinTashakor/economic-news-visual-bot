use crate::error::AppError;
use crate::normalize::render_model::RenderEvent;
use crate::publisher::TelegramConfig;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use teloxide::prelude::*;
use teloxide::types::InputPollOption;

#[derive(Debug, Clone)]
pub struct PollContext {
    /// Telegram poll id (String form of PollId)
    pub poll_id: String,
    /// RenderEvent options in the same order as poll options
    pub options: Vec<RenderEvent>,
    /// Selected option indices (filled by poll_answer handler later)
    pub selected_indices: Vec<usize>,
}

static ACTIVE_POLL: Lazy<Mutex<Option<PollContext>>> =
    Lazy::new(|| Mutex::new(None));

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
        .map(|e| {
            InputPollOption::new(
                format!("{} {} — {}", e.currency, e.title, e.time),
            )
        })
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

    let mut ctx = ACTIVE_POLL.lock().unwrap();
    *ctx = Some(PollContext {
        poll_id: poll.id.to_string(), // PollId -> String
        options: events,
        selected_indices: Vec::new(),
    });

    Ok(())
}

/// Records poll answers (option indices) for the active poll.
/// Should be called from poll_answer update handler.
pub fn handle_poll_answer(poll_id: &str, option_indices: &[u32]) {
    let mut ctx = ACTIVE_POLL.lock().unwrap();

    if let Some(active) = ctx.as_mut() {
        if active.poll_id == poll_id {
            active.selected_indices = option_indices
                .iter()
                .map(|&i| i as usize)
                .collect();
        }
    }
}

/// Returns the RenderEvents selected by admins (based on poll answers).
pub fn get_selected_events() -> Vec<RenderEvent> {
    let ctx = ACTIVE_POLL.lock().unwrap();

    if let Some(active) = ctx.as_ref() {
        active
            .selected_indices
            .iter()
            .filter_map(|&i| active.options.get(i).cloned())
            .collect()
    } else {
        Vec::new()
    }
}

/// Clears the active poll context (after rendering or on reset).
pub fn clear_active_poll() {
    let mut ctx = ACTIVE_POLL.lock().unwrap();
    *ctx = None;
}
