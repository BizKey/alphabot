use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup};
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(dptree::endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler).build().dispatch().await;
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if msg.text() == Some("/start") {
        let keyboard = make_inline_keyboard();
        bot.send_message(msg.chat.id, "Выберите действие:")
            .reply_markup(keyboard)
            .await?;
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data {
        let chat_id = q.message.unwrap().chat().id;

        match data.as_str() {
            "action_1" => {
                bot.send_message(chat_id, "Вы выбрали действие 1!").await?;
            }
            "action_2" => {
                bot.send_message(chat_id, "Вы выбрали действие 2!").await?;
            }
            _ => {
                bot.send_message(chat_id, "Неизвестное действие").await?;
            }
        }

        // Ответ на callback (убирает "часики" у кнопки)
        bot.answer_callback_query(q.id).await?;
    }
    Ok(())
}

fn make_inline_keyboard() -> InlineKeyboardMarkup {
    let keyboard = vec![vec![
        InlineKeyboardButton::callback("Действие 1", "action_1"),
        InlineKeyboardButton::callback("Действие 2", "action_2"),
    ]];

    InlineKeyboardMarkup::new(keyboard)
}
