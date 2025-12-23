use crate::commands::get::execute_get;
use crate::commands::poll::execute_poll;
use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {


    execute_get().await?;
    execute_poll().await?;

    Ok(())
}
