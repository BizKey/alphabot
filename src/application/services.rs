use crate::domain::BotCommand;

pub trait MessageService: Send + Sync {
    fn handle_command(&self, command: &BotCommand) -> String;
    fn get_info_text(&self) -> String;
}

#[derive(Clone)]
pub struct MessageServiceImpl;

impl MessageService for MessageServiceImpl {
    fn handle_command(&self, command: &BotCommand) -> String {
        match command {
            BotCommand::Start | BotCommand::Menu => "Главное меню:".to_string(),
            BotCommand::Info => self.get_info_text(),
            BotCommand::Status => "📊 Функция статуса будет реализована позже".to_string(),
            BotCommand::Profit => "💰 Функция прибыли будет реализована позже".to_string(),
            BotCommand::Trading => "⚡ Функции торговли будут реализованы позже".to_string(),
            BotCommand::Unknown(cmd) => {
                format!("Неизвестная команда: {}. Используйте /menu", cmd)
            }
        }
    }

    fn get_info_text(&self) -> String {
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
}
