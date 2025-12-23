use crate::error::AppError;
use crate::normalize::priority::select_top_events;
use crate::publisher::load_telegram_config;
use crate::publisher::telegram_poll::send_events_poll;
use crate::state::STATE;

pub async fn execute_poll() -> Result<(), AppError> {
    // 1️⃣ Read cache
    let cached_events = {
        let state = STATE.lock().unwrap();
        match &state.daily_cache {
            Some(cache) => cache.events.clone(),
            None => {
                return Err(AppError::Publisher(
                    "No cached news found. Run /get first.".into(),
                ))
            }
        }
    };

    if cached_events.is_empty() {
        return Err(AppError::Publisher(
            "Cached news is empty. Run /get again.".into(),
        ));
    }

    // 2️⃣ Select top 12 by priority
    let poll_candidates = select_top_events(cached_events, 12);

    if poll_candidates.is_empty() {
        return Err(AppError::Publisher(
            "No events available for poll.".into(),
        ));
    }

    // 3️⃣ Load Telegram config
    let tg = load_telegram_config()?;

    // 4️⃣ Clear previous poll (if any)
    {
        let mut state = STATE.lock().unwrap();
        state.poll = None;
    }

    // 5️⃣ Send poll
    send_events_poll(&tg, poll_candidates).await?;

    println!("✅ Poll created from cached daily events");

    Ok(())
}
