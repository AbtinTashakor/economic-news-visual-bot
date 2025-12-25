mod app;
mod commands;
mod config;
mod error;
mod filter;
mod image;
mod models;
mod normalize;
mod publisher;
mod renderer;
mod scraper;
mod sources;
mod state;

use std::time::Duration;

use crate::app::run;
use crate::publisher::load_telegram_config;

#[tokio::main]
async fn main() {
    loop {
        println!("🚀 Starting bot...");

        let cfg = match load_telegram_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("❌ Failed to load config: {}", e);
                sleep_and_retry().await;
                continue;
            }
        };

        match run(cfg).await {
            Ok(_) => {
                println!("⚠️ Bot stopped gracefully. Restarting...");
            }
            Err(e) => {
                eprintln!("❌ Bot crashed: {}", e);
            }
        }

        sleep_and_retry().await;
    }
}

async fn sleep_and_retry() {
    println!("⏳ Retrying in 10 seconds...");
    tokio::time::sleep(Duration::from_secs(10)).await;
}
