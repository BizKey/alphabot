use crate::application::services::MessageService;
use crate::domain::{BotCommand, KeyboardButton, MainKeyboard};
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use tracing::error;

#[derive(Clone)]
pub struct MessageHandler {
    service: Arc<dyn MessageService>,
}

impl MessageHandler {
    pub fn new(service: Arc<dyn MessageService>) -> Self {
        Self { service }
    }

    pub async fn handle(&self, bot: Bot, msg: Message) -> ResponseResult<()> {
        let text = match msg.text() {
            Some(text) => text,
            None => {
                error!("Empty message");
                return Ok(());
            }
        };

        let command = BotCommand::from_text(text);
        let chat_id = msg.chat.id;

        match command {
            BotCommand::Start | BotCommand::Menu => {
                let keyboard = Self::build_keyboard(MainKeyboard::create());
                if let Err(e) = bot
                    .send_message(chat_id, self.service.handle_command(&command))
                    .reply_markup(keyboard)
                    .await
                {
                    error!("Failed to send message: {}", e);
                }
            }
            BotCommand::Info => {
                if let Err(e) = bot
                    .send_message(chat_id, self.service.handle_command(&command))
                    .parse_mode(ParseMode::Html)
                    .await
                {
                    error!("Failed to send message: {}", e);
                }
            }
            _ => {
                if let Err(e) = bot
                    .send_message(chat_id, self.service.handle_command(&command))
                    .await
                {
                    error!("Failed to send message: {}", e);
                }
            }
        }

        Ok(())
    }

    fn build_keyboard(buttons: Vec<Vec<KeyboardButton>>) -> InlineKeyboardMarkup {
        let keyboard: Vec<Vec<InlineKeyboardButton>> = buttons
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|btn| InlineKeyboardButton::callback(&btn.label, &btn.callback_data))
                    .collect()
            })
            .collect();

        InlineKeyboardMarkup::new(keyboard)
    }
}

#[derive(Clone)]
pub struct CallbackHandler {
    service: Arc<dyn MessageService>,
}

impl CallbackHandler {
    pub fn new(service: Arc<dyn MessageService>) -> Self {
        Self { service }
    }

    pub async fn handle(&self, bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
        if let Some(data) = q.data {
            let chat_id = ChatId(q.from.id.0 as i64);
            let command = match data.as_str() {
                "status" => BotCommand::Status,
                "profit" => BotCommand::Profit,
                "trading" => BotCommand::Trading,
                "info" => BotCommand::Info,
                _ => BotCommand::Unknown(data),
            };

            let response = self.service.handle_command(&command);

            let mut message = bot.send_message(chat_id, response);
            if command == BotCommand::Info {
                message = message.parse_mode(ParseMode::Html);
            }

            if let Err(e) = message.await {
                error!("Failed to send callback message: {}", e);
            }

            if let Err(e) = bot.answer_callback_query(q.id).await {
                error!("Failed to answer callback: {}", e);
            }
        }

        Ok(())
    }
}
