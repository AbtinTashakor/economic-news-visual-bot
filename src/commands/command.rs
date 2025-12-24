use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "get latest or dated data")]
    Get {
        /// optional date: YYYY-MM-DD
        #[command(default)]
        date: String,
    },

    #[command(description = "create poll")]
    Poll,

    #[command(description = "render output")]
    Render,
}
