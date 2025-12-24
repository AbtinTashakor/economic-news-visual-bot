use crate::commands::get::execute_get;
use crate::commands::poll::execute_poll;
use crate::commands::render::execute_render;
use crate::error::AppError;
use crate::models::Impact;
use crate::publisher::telegram_poll::{get_selected_events, handle_poll_answer};
use crate::state::STATE;
use std::path::Path;

use crate::renderer::{
    render::render_png,
    types::{RenderEvent, RenderMeta},
};

pub async fn run() -> Result<(), AppError> {
    execute_get().await?;
    execute_poll().await?;

    let poll_id = {
        let state = STATE.lock().unwrap();
        match &state.poll {
            Some(poll) => poll.poll_id.clone(),
            None => {
                println!("❌ No active poll found in state");
                return Ok(());
            }
        }
    };

    // simulate admin vote
    handle_poll_answer(&poll_id, &[0, 2]);

    let selected = get_selected_events();

    println!("✅ Selected events:");
    for e in selected {
        println!("{} | {} | {:?}", e.currency, e.title, e.impact);
    }

    execute_render().await?;

    Ok(())
}
