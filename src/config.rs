use serde::Deserialize;
use crate::error::AppError;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TargetTimezone {
    NY,
    IR,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub impact: Vec<String>,
    pub currency: Vec<String>,
    pub timezone: TargetTimezone,
    pub language: String,
}

impl AppConfig {
    pub fn load(path: &str) -> Result<Self, AppError> {
        let content = fs::read_to_string(path)
            .map_err(|e| AppError::Config(e.to_string()))?;

        let config: AppConfig = serde_yaml::from_str(&content)
            .map_err(|e| AppError::Config(e.to_string()))?;

        Ok(config)
    }
}
