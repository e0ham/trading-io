mod backtest;
mod config;
mod core;
mod data;
mod execution;
mod portfolio;
mod risk;
mod strategy;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    utils::logging::init();
    let cfg = config::settings::Settings::load()?;

    tracing::info!(mode = %cfg.app.mode, "starting trading application");
    tracing::info!("project scaffold is ready");

    Ok(())
}
