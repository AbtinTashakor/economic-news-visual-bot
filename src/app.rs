use crate::config::AppConfig;
use crate::error::AppError;
use crate::scraper;
use crate::filter;
use crate::image;
use crate::publisher;

pub async fn run() -> Result<(), AppError> {
    let config = AppConfig::load("config/default.yaml")?;

    //println!("Loaded config: {:?}", config);

    let events = scraper::fetch_events().await?;
    //println!("Fetched {} events",events.len());

    let filtered = filter::filter_events(events, &config);
    //println!("Filtered {} events",filtered.len());

    image::render_image(&filtered, "output.png")?;
    println!("Image generated: output.png");

    publisher::send_image(
        "output.png",
        "📊 Economic Events Today",
    )
    .await?;

    println!("Image sent to Telegram");

    

    Ok(())
}
