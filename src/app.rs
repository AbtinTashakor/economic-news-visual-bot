//use crate::commands::get::execute_get;
//use crate::commands::poll::execute_poll;
use crate::error::AppError;
use crate::models::Impact;
use crate::publisher::telegram_poll::{get_selected_events, handle_poll_answer};
//use crate::state::STATE;
use std::path::Path;

use crate::renderer::{
    render::render_png,
    types::{RenderEvent, RenderMeta},
};

pub async fn run() -> Result<(), AppError> {
    // execute_get().await?;
    // execute_poll().await?;

    // let poll_id = {
    //     let state = STATE.lock().unwrap();
    //     match &state.poll {
    //         Some(poll) => poll.poll_id.clone(),
    //         None => {
    //             println!("❌ No active poll found in state");
    //             return Ok(());
    //         }
    //     }
    // };

    // // simulate admin vote
    // handle_poll_answer(&poll_id, &[0, 2]);

    // let selected = get_selected_events();

    // println!("✅ Selected events:");
    // for e in selected {
    //     println!("{} | {} | {:?}", e.currency, e.title, e.impact);
    // }

    let meta = RenderMeta {
        day_name: "Monday".to_string(),
        date_label: "December 15".to_string(),
    };

    let events = vec![
        RenderEvent {
            title: "Canada CPI m/m".to_string(),
            currency: "CAD".to_string(),
            time_label: "08:30".to_string(),
            impact: Impact::High,
        },
        RenderEvent {
            title: "US Retail Sales m/m".to_string(),
            currency: "USD".to_string(),
            time_label: "10:00".to_string(),
            impact: Impact::Medium,
        },
    ];

    let png = render_png(Path::new("assets"), meta, &events)?;

    std::fs::write("tmp/render_test.png", png)
        .map_err(|e| AppError::Image(format!("failed to write render_test.png: {}", e)))?;

    println!("✅ render_test.png generated");
    Ok(())


}
