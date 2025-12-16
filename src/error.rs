use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Config(String),
    Scraper(String),
    Image(String),
    Publisher(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Config(e) => write!(f, "Config error: {}", e),
            AppError::Scraper(e) => write!(f, "Scraper error: {}", e),
            AppError::Image(e) => write!(f, "Image error: {}", e),
            AppError::Publisher(e) => write!(f, "Publisher error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
