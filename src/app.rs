use crate::config::AppConfig;
use crate::error::AppError;

pub fn run() -> Result<(), AppError> {
    let config = AppConfig::load("config/default.yaml")?;

    println!("Loaded config: {:?}", config);

    Ok(())
}
