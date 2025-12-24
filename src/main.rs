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

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
