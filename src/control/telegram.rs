use std::collections::HashSet;

use anyhow::Result;
use teloxide::{prelude::*, repls::repl, requests::Requester, utils::command::BotCommands};
use tracing::{info, warn};

use crate::trading::engine::TradingController;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Trading controls")]
enum Command {
    #[command(description = "show this help")]
    Help,
    #[command(description = "start the trading loop")]
    StartTrading,
    #[command(description = "stop the trading loop")]
    StopTrading,
    #[command(description = "show runtime status")]
    Status,
    #[command(description = "show recent trades")]
    Trades,
}

pub async fn run(
    controller: TradingController,
    bot_token: &str,
    allowed_chat_ids: Vec<i64>,
) -> Result<()> {
    let bot = Bot::new(bot_token);
    let allowed_set: HashSet<i64> = allowed_chat_ids.into_iter().collect();

    // Ensure polling mode is clean if webhook mode was previously configured.
    let _ = bot.delete_webhook().drop_pending_updates(true).send().await;

    repl(bot, move |bot: Bot, msg: Message| {
        let controller = controller.clone();
        let allowed_set = allowed_set.clone();
        async move {
            let chat_id = msg.chat.id.0;
            let text = msg.text().unwrap_or("");

            info!(chat_id, text = %text, "telegram message received");

            if !allowed_set.is_empty() && !allowed_set.contains(&chat_id) {
                warn!(chat_id, "telegram message rejected from unauthorized chat");
                bot.send_message(msg.chat.id, "unauthorized chat").await?;
                return Ok(());
            }

            let Some(cmd) = parse_command(text) else {
                bot.send_message(
                    msg.chat.id,
                    "unknown command. Use /help, /starttrading, /stoptrading, /status, or /trades",
                )
                .await?;
                return Ok(());
            };

            let reply = match cmd {
                Command::Help => Command::descriptions().to_string(),
                Command::StartTrading => controller.start().await,
                Command::StopTrading => controller.stop().await,
                Command::Status => controller.status_text().await,
                Command::Trades => controller.recent_trades_text(10).await,
            };

            bot.send_message(msg.chat.id, reply).await?;

            Ok(())
        }
    })
    .await;

    Ok(())
}

fn parse_command(text: &str) -> Option<Command> {
    let normalized = text.trim().split_whitespace().next()?;
    let command = normalized.split('@').next()?.to_ascii_lowercase();

    match command.as_str() {
        "/help" => Some(Command::Help),
        "/starttrading" => Some(Command::StartTrading),
        "/stoptrading" => Some(Command::StopTrading),
        "/status" => Some(Command::Status),
        "/trades" => Some(Command::Trades),
        _ => None,
    }
}
