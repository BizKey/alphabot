#[derive(Debug, Clone, PartialEq)]
pub enum BotCommand {
    Start,
    Menu,
    Info,
    Status,
    Profit,
    Trading,
    Unknown(String),
}

impl BotCommand {
    pub fn from_text(text: &str) -> Self {
        match text {
            "/start" => BotCommand::Start,
            "/menu" => BotCommand::Menu,
            "/info" => BotCommand::Info,
            "/status" => BotCommand::Status,
            "/profit" => BotCommand::Profit,
            "/trading" => BotCommand::Trading,
            _ if text.starts_with('/') => BotCommand::Unknown(text.to_string()),
            _ => BotCommand::Unknown(text.to_string()),
        }
    }
}
