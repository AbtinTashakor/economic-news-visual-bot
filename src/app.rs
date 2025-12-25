use std::sync::Arc;

use crate::commands::command::Command;
use crate::commands::{get::execute_get, poll::execute_poll, render::execute_render};
use crate::error::AppError;
use crate::publisher::TELEGRAM;
use crate::publisher::TelegramConfig;
use crate::publisher::telegram::send_message;
use crate::publisher::telegram_poll::handle_poll_answer;
use chrono::NaiveDate;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

/// Bot runtime (dispatcher)
pub async fn run(cfg: TelegramConfig) -> Result<(), AppError> {
    let bot = cfg.bot.clone();

    println!("🤖 Bot is running...");

    // --------------------------------------------------
    // Get bot username (for parsing /cmd@BotName)
    // --------------------------------------------------
    let me = bot.get_me().await?;
    let bot_name = Arc::new(me.username.clone().expect("Bot must have a username"));

    // --------------------------------------------------
    // DM / Group messages handler
    // --------------------------------------------------
    let message_handler = Update::filter_message()
        .filter_map(|msg: Message| msg.text().map(|text| (msg.chat.id, text.to_string())))
        .endpoint({
            let bot_name = Arc::clone(&bot_name);
            move |(chat_id, text): (ChatId, String)| {
                let bot_name = Arc::clone(&bot_name);
                async move {
                    handle_command(&text, &bot_name, chat_id).await;
                    Ok::<(), AppError>(())
                }
            }
        });

    // --------------------------------------------------
    // Channel posts handler
    // --------------------------------------------------
    let channel_handler = Update::filter_channel_post()
        .filter_map(|msg: Message| msg.text().map(|text| (msg.chat.id, text.to_string())))
        .endpoint({
            let bot_name = Arc::clone(&bot_name);
            move |(chat_id, text): (ChatId, String)| {
                let bot_name = Arc::clone(&bot_name);
                async move {
                    handle_command(&text, &bot_name, chat_id).await;
                    Ok::<(), AppError>(())
                }
            }
        });

    // --------------------------------------------------
    // Poll answer handler
    // --------------------------------------------------
    let poll_handler = Update::filter_poll_answer().endpoint(|answer: PollAnswer| async move {
        println!("📥 PollAnswer received: {:?}", answer);

        let poll_id = answer.poll_id.0.as_str();
        let option_ids: Vec<u32> = answer.option_ids.iter().map(|&id| id as u32).collect();

        handle_poll_answer(poll_id, &option_ids);

        // 🔥 auto render after vote
        if let Err(e) = execute_render().await {
            eprintln!("auto render failed: {}", e);
        }

        Ok::<(), AppError>(())
    });

    // --------------------------------------------------
    // Dispatcher tree
    // --------------------------------------------------
    let handler = dptree::entry()
        .branch(message_handler)
        .branch(channel_handler)
        .branch(poll_handler);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

/// Shared command handler for all message sources
async fn handle_command(text: &str, bot_name: &str, _chat_id: ChatId) {
    let cmd = match Command::parse(text, bot_name) {
        Ok(cmd) => cmd,
        Err(_) => return, // not a command
    };

    match cmd {
        Command::Get { date } => {
            println!("fetching data for date: {:?}", date);
            let parsed_date = NaiveDate::parse_from_str(&date, "%m/%d/%Y").ok();
            let status = send_message(
                &TELEGRAM,
                "⏳ Fetching economic calendar data… Please wait.",
            )
            .await
            .ok();

            execute_get(parsed_date).await.ok();
            execute_poll().await.ok();

            if let Some(msg) = status {
                let _ = TELEGRAM
                    .bot
                    .delete_message(TELEGRAM.recipient.clone(), msg.id)
                    .await;
            }
        }

        Command::Poll => {
            execute_poll().await.ok();
        }

        Command::Render => {
            execute_render().await.ok();
        }
    }
}
