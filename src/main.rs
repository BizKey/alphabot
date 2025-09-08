use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup};
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(text) = msg.text() {
        match text {
            "/start" | "/menu" => {
                let keyboard = make_main_keyboard();
                bot.send_message(msg.chat.id, "Главное меню:")
                    .reply_markup(keyboard)
                    .await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Используйте /menu для отображения меню")
                    .await?;
            }
        }
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data {
        let chat_id = q.from.id;
        match data.as_str() {
            "btn1" => {
                bot.send_message(chat_id, "Вы нажали кнопку 1! ✅").await?;
            }
            "btn2" => {
                bot.send_message(chat_id, "Вы нажали кнопку 2! 🚀").await?;
            }
            "btn3" => {
                bot.send_message(chat_id, "Вы нажали кнопку 3! 💡").await?;
            }
            _ => {
                bot.send_message(chat_id, "Неизвестное действие").await?;
            }
        }

        bot.answer_callback_query(q.id).await?;
    }
    Ok(())
}

fn make_main_keyboard() -> InlineKeyboardMarkup {
    let keyboard = vec![
        vec![
            InlineKeyboardButton::callback("Кнопка 1 ✅", "btn1"),
            InlineKeyboardButton::callback("Кнопка 2 🚀", "btn2"),
        ],
        vec![InlineKeyboardButton::callback("Кнопка 3 💡", "btn3")],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
