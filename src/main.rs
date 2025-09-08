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
            "/info" => {
                let help_text = get_info_text();
                bot.send_message(msg.chat.id, help_text)
                    .parse_mode(teloxide::types::ParseMode::Html)
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

fn get_info_text() -> String {
    r#"
📊 <b>Доступные команды:</b>

🚀 <b>Основные команды:</b>
<code>/start</code> - Starts the trader.
<code>/stop</code> - Stops the trader.
<code>/stopentry</code> - Stop entering new trades.

📈 <b>Статус и мониторинг:</b>
<code>/status</code> [trade_id|table] - Lists all or specific open trades.
<code>/performance</code> - Show performance of each finished trade grouped by pair.
<code>/balance</code> - Show account balance per currency.

💰 <b>Прибыль и отчетность:</b>
<code>/profit</code> [n] - Cumulative profit from all finished trades (last n days).
<code>/profit_long</code> [n] - Cumulative profit from long trades (last n days).
<code>/profit_short</code> [n] - Cumulative profit from short trades (last n days).
<code>/daily</code> [n] - Profit/loss per day (last n days).

⚡ <b>Управление сделками:</b>
<code>/forceexit</code> trade_id|all - Instantly exits trade (ignore minimum_roi).
<code>/fx</code> trade_id|all - Alias to /forceexit.

❓ <b>Справка:</b>
<code>/help</code> - Show help message.
<code>/version</code> - Show version.
<code>/info</code> - Show this info message.

💡 <i>Используйте кнопки меню для быстрого доступа к командам!</i>
    "#
    .trim()
    .to_string()
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(data) = q.data {
        let chat_id = q.from.id;
        match data.as_str() {
            "info" => {
                let help_text = get_info_text();
                bot.send_message(chat_id, help_text)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await?;
            }
            "status" => {
                bot.send_message(chat_id, "Функция статуса будет реализована позже 📊")
                    .await?;
            }
            "profit" => {
                bot.send_message(chat_id, "Функция прибыли будет реализована позже 💰")
                    .await?;
            }
            "trading" => {
                bot.send_message(chat_id, "Функции торговли будут реализованы позже ⚡")
                    .await?;
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
            InlineKeyboardButton::callback("📊 Статус", "status"),
            InlineKeyboardButton::callback("💰 Прибыль", "profit"),
        ],
        vec![
            InlineKeyboardButton::callback("⚡ Торговля", "trading"),
            InlineKeyboardButton::callback("❓ Помощь", "info"),
        ],
    ];

    InlineKeyboardMarkup::new(keyboard)
}
