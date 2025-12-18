mod app;
mod config;
mod error;
mod models;
mod scraper;
mod filter;
mod image; 
mod publisher;

use crate::app::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
