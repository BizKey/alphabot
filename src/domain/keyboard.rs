#[derive(Clone)]
pub struct KeyboardButton {
    pub label: String,
    pub callback_data: String,
}

impl KeyboardButton {
    pub fn new(label: &str, callback_data: &str) -> Self {
        Self {
            label: label.to_string(),
            callback_data: callback_data.to_string(),
        }
    }
}

pub struct MainKeyboard;

impl MainKeyboard {
    pub fn create() -> Vec<Vec<KeyboardButton>> {
        vec![
            vec![
                KeyboardButton::new("📊 Статус", "status"),
                KeyboardButton::new("💰 Прибыль", "profit"),
            ],
            vec![
                KeyboardButton::new("⚡ Торговля", "trading"),
                KeyboardButton::new("❓ Помощь", "info"),
            ],
        ]
    }
}
