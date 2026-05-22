mod config;
mod control;
mod core;
mod data;
mod trading;
mod utils;

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    utils::logging::init();
    let cfg = config::settings::Settings::load()?;

    tracing::info!(mode = %cfg.app.mode, "starting trading application");

    let controller = trading::engine::TradingController::new(cfg.clone());

    if cfg.telegram.enabled {
        let token = cfg
            .telegram
            .bot_token
            .clone()
            .context("TELEGRAM_ENABLED=true but TELEGRAM_BOT_TOKEN is missing")?;

        tracing::info!("telegram control enabled");
        if let Err(err) =
            control::telegram::run(controller, &token, cfg.telegram.allowed_chat_ids.clone()).await
        {
            tracing::error!(error = %err, "telegram bot failed; falling back to passive mode");
        }
    } else {
        tracing::info!("telegram disabled; set TELEGRAM_ENABLED=true to enable bot control");
    }

    tracing::info!("application idle; waiting for ctrl-c");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
