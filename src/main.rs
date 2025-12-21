mod app;
mod config;
mod error;
mod models;
mod scraper;
mod filter;
mod image; 
mod publisher;

use crate::app::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
