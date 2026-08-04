use crate::application::services::MessageServiceImpl;
use crate::presentation::handlers::{CallbackHandler, MessageHandler};
use std::sync::Arc;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::*;

pub struct BotDispatcher {
    bot: Bot,
}

impl BotDispatcher {
    pub fn new(bot: Bot) -> Self {
        Self { bot }
    }

    pub async fn run(&self) {
        let service = Arc::new(MessageServiceImpl);

        let message_handler = MessageHandler::new(service.clone());
        let callback_handler = CallbackHandler::new(service);

        let handler = dptree::entry()
            .branch(Update::filter_message().endpoint({
                let handler = message_handler;
                move |bot: Bot, msg: Message| {
                    let handler = handler.clone();
                    async move { handler.handle(bot, msg).await }
                }
            }))
            .branch(Update::filter_callback_query().endpoint({
                let handler = callback_handler;
                move |bot: Bot, q: CallbackQuery| {
                    let handler = handler.clone();
                    async move { handler.handle(bot, q).await }
                }
            }));

        teloxide::dispatching::Dispatcher::builder(self.bot.clone(), handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
