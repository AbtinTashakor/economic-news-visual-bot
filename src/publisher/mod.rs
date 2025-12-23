use crate::error::AppError;
use std::env;
use teloxide::prelude::*;
use teloxide::types::Recipient;

pub mod telegram;
pub mod telegram_poll;

/// Telegram runtime configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct TelegramConfig {
    pub bot: Bot,
    pub recipient: Recipient,
}

/// Loads Telegram configuration from environment variables
pub fn load_telegram_config() -> Result<TelegramConfig, AppError> {
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| AppError::Publisher("Missing TELEGRAM_BOT_TOKEN".into()))?;

    let chat_raw = env::var("TELEGRAM_CHAT_ID")
        .map_err(|_| AppError::Publisher("Missing TELEGRAM_CHAT_ID".into()))?;

    let bot = Bot::new(bot_token);

    let recipient = if let Ok(id) = chat_raw.parse::<i64>() {
        Recipient::Id(ChatId(id))
    } else if chat_raw.starts_with('@') {
        Recipient::ChannelUsername(chat_raw)
    } else {
        return Err(AppError::Publisher(
            "Invalid TELEGRAM_CHAT_ID (must be numeric or @channel)".into(),
        ));
    };

    Ok(TelegramConfig { bot, recipient })
}
