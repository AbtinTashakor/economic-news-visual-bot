use std::path::Path;

use image::DynamicImage;

use crate::error::AppError;

pub struct RenderAssets {
    pub template: DynamicImage,
    pub impact_high: DynamicImage,
    pub impact_medium: DynamicImage,
    pub impact_low: DynamicImage,
    pub impact_holiday: DynamicImage,
}

impl RenderAssets {
    pub fn load(assets_dir: &Path) -> Result<Self, AppError> {
        let template_path = assets_dir.join("template.png");
        let high_path = assets_dir.join("impact_high.png");
        let med_path = assets_dir.join("impact_medium.png");
        let low_path = assets_dir.join("impact_low.png");
        let holiday_path = assets_dir.join("impact_holiday.png");

        let template = image::open(&template_path)
            .map_err(|e| AppError::Image(format!("failed to open template {:?}: {}", template_path, e)))?;

        let impact_high = image::open(&high_path)
            .map_err(|e| AppError::Image(format!("failed to open impact_high {:?}: {}", high_path, e)))?;

        let impact_medium = image::open(&med_path)
            .map_err(|e| AppError::Image(format!("failed to open impact_medium {:?}: {}", med_path, e)))?;

        let impact_low = image::open(&low_path)
            .map_err(|e| AppError::Image(format!("failed to open impact_low {:?}: {}", low_path, e)))?;

        let impact_holiday = image::open(&holiday_path)
            .map_err(|e| AppError::Image(format!("failed to open impact_holiday {:?}: {}", holiday_path, e)))?;

        Ok(Self {
            template,
            impact_high,
            impact_medium,
            impact_low,
            impact_holiday,
        })
    }
}
