use crate::commands::get::execute_get;
use crate::error::AppError;

pub async fn run() -> Result<(), AppError> {


    execute_get().await?;


    Ok(())
}
