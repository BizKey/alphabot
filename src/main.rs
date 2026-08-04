mod application;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::logging::init_tracing;
use presentation::dispatcher::BotDispatcher;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    init_tracing();
    tracing::info!("Starting bot...");

    let bot = Bot::from_env();
    let dispatcher = BotDispatcher::new(bot);

    dispatcher.run().await;
}
