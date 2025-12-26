use crate::publisher::TelegramConfig;
use crate::error::AppError;

use teloxide::prelude::*;         
use teloxide::types::InputFile;

pub async fn send_image(
    cfg: &TelegramConfig,
    image_path: &str,
    caption: &str,
) -> Result<(), AppError> {
    cfg.bot
        .send_photo(
            cfg.recipient.clone(),
            InputFile::file(image_path),
        )
        .caption(caption.to_string())
        .send()
        .await
        .map_err(|e| AppError::Publisher(e.to_string()))?;

    Ok(())
}

/// Send a plain text message to the configured recipient
pub async fn send_message(
    cfg: &TelegramConfig,
    text: impl AsRef<str>,
) -> Result<Message, AppError> {
    let msg = cfg
        .bot
        .send_message(cfg.recipient.clone(), text.as_ref())
        .send()
        .await
        .map_err(|e| AppError::Publisher(e.to_string()))?;

    Ok(msg)
}
