use crate::error::AppError;
use teloxide::{prelude::*, types::InputFile};
use std::env;

pub async fn send_image(
    image_path: &str,
    caption: &str,
) -> Result<(), AppError> {
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| AppError::Publisher("Missing TELEGRAM_BOT_TOKEN".into()))?;

    let chat_id_raw = env::var("TELEGRAM_CHAT_ID")
        .map_err(|_| AppError::Publisher("Missing TELEGRAM_CHAT_ID".into()))?;

    let bot = Bot::new(bot_token);

    if let Ok(id) = chat_id_raw.parse::<i64>() {
        // حالت عددی
        bot.send_photo(ChatId(id), InputFile::file(image_path))
            .caption(caption.to_string())
            .send()
            .await
            .map_err(|e| AppError::Publisher(e.to_string()))?;
    } else if chat_id_raw.starts_with('@') {
        //  نکته مهم: String بده، نه &str
        bot.send_photo(chat_id_raw, InputFile::file(image_path))
            .caption(caption.to_string())
            .send()
            .await
            .map_err(|e| AppError::Publisher(e.to_string()))?;
    } else {
        return Err(AppError::Publisher(
            "Invalid TELEGRAM_CHAT_ID (must be numeric or @channel)".into(),
        ));
    }

    Ok(())
}
