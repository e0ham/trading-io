use anyhow::Result;
use teloxide::{
    prelude::*,
    repls::CommandReplExt,
    requests::Requester,
    utils::command::BotCommands,
};

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

pub async fn run(controller: TradingController, bot_token: &str) -> Result<()> {
    let bot = Bot::new(bot_token);
    // Ensure polling mode is clean if webhook mode was previously configured.
    let _ = bot.delete_webhook().drop_pending_updates(true).send().await;

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let controller = controller.clone();
        async move {
            let reply = match cmd {
                Command::Help => Command::descriptions().to_string(),
                Command::StartTrading => controller.start().await,
                Command::StopTrading => controller.stop().await,
                Command::Status => controller.status_text().await,
                Command::Trades => controller.recent_trades_text(10).await,
            };

            bot.send_message(msg.chat.id, reply)
                .await?;

            Ok(())
        }
    })
    .await;

    Ok(())
}
