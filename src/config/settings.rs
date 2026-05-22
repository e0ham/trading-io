use std::env;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub mode: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradingConfig {
    pub base_token: String,
    pub quote_token: String,
    pub trade_size_quote: f64,
    pub min_profit_bps: f64,
    pub max_slippage_bps: f64,
    pub max_quote_age_ms: u64,
    pub loop_interval_ms: u64,
    pub estimated_tx_cost_quote: f64,
    pub latency_buffer_quote: f64,
    pub dexes: Vec<String>,
    pub jupiter_quote_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramConfig {
    pub enabled: bool,
    pub bot_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub app: AppConfig,
    pub trading: TradingConfig,
    pub telegram: TelegramConfig,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let app = AppConfig {
            mode: env::var("APP_MODE").unwrap_or_else(|_| "paper".to_string()),
        };

        let dexes = env::var("DEXES")
            .unwrap_or_else(|_| "Raydium,Orca+V2,Meteora+DLMM".to_string())
            .split(',')
            .map(str::trim)
            .filter(|v| !v.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        let trading = TradingConfig {
            base_token: env::var("BASE_TOKEN").unwrap_or_else(|_| "SOL".to_string()),
            quote_token: env::var("QUOTE_TOKEN").unwrap_or_else(|_| "USDC".to_string()),
            trade_size_quote: parse_env_f64("TRADE_SIZE_QUOTE", 100.0)?,
            min_profit_bps: parse_env_f64("MIN_PROFIT_BPS", 15.0)?,
            max_slippage_bps: parse_env_f64("MAX_SLIPPAGE_BPS", 20.0)?,
            max_quote_age_ms: parse_env_u64("MAX_QUOTE_AGE_MS", 500)?,
            loop_interval_ms: parse_env_u64("LOOP_INTERVAL_MS", 1000)?,
            estimated_tx_cost_quote: parse_env_f64("ESTIMATED_TX_COST_QUOTE", 0.01)?,
            latency_buffer_quote: parse_env_f64("LATENCY_BUFFER_QUOTE", 0.005)?,
            dexes,
            jupiter_quote_url: env::var("JUPITER_QUOTE_URL").ok(),
        };

        let bot_token = env::var("TELEGRAM_BOT_TOKEN").ok();
        let telegram = TelegramConfig {
            enabled: env::var("TELEGRAM_ENABLED")
                .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "True"))
                .unwrap_or(false),
            bot_token,
        };

        Ok(Self {
            app,
            trading,
            telegram,
        })
    }
}

fn parse_env_f64(name: &str, default: f64) -> Result<f64> {
    match env::var(name) {
        Ok(v) => v
            .parse::<f64>()
            .with_context(|| format!("failed to parse {name} as f64")),
        Err(_) => Ok(default),
    }
}

fn parse_env_u64(name: &str, default: u64) -> Result<u64> {
    match env::var(name) {
        Ok(v) => v
            .parse::<u64>()
            .with_context(|| format!("failed to parse {name} as u64")),
        Err(_) => Ok(default),
    }
}
