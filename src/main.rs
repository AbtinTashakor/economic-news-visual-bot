mod app;
mod config;
mod error;
mod models;
mod scraper;
mod filter;
mod image; 
mod publisher;
mod sources;
mod normalize;
mod commands;
mod state;
mod renderer;

use crate::app::run;
use crate::publisher::load_telegram_config;

#[tokio::main]
async fn main() {
    let cfg = match load_telegram_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load telegram config: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(cfg).await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
