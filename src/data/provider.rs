use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::Deserialize;

use crate::core::types::Quote;

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

#[derive(Clone)]
pub struct JupiterQuoteProvider {
    client: Client,
    base_url: String,
    slippage_bps: u16,
}

impl JupiterQuoteProvider {
    pub fn new(base_url: Option<String>, slippage_bps: u16) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.unwrap_or_else(|| "https://api.jup.ag/swap/v1/quote".to_string()),
            slippage_bps,
        }
    }

    pub async fn quote_exact_in(
        &self,
        dex: &str,
        input_symbol: &str,
        output_symbol: &str,
        input_amount_ui: f64,
    ) -> Result<Quote> {
        let input_mint = mint_for_symbol(input_symbol)?;
        let output_mint = mint_for_symbol(output_symbol)?;
        let input_decimals = decimals_for_symbol(input_symbol)?;
        let output_decimals = decimals_for_symbol(output_symbol)?;

        let raw_amount = to_raw_amount(input_amount_ui, input_decimals)?;

        let response = self
            .client
            .get(&self.base_url)
            .query(&[
                ("inputMint", input_mint),
                ("outputMint", output_mint),
                ("amount", &raw_amount.to_string()),
                ("slippageBps", &self.slippage_bps.to_string()),
                ("swapMode", "ExactIn"),
                ("dexes", dex),
                ("restrictIntermediateTokens", "true"),
            ])
            .send()
            .await
            .context("failed to call Jupiter quote API")?
            .error_for_status()
            .context("Jupiter quote API returned error status")?
            .json::<JupiterQuoteResponse>()
            .await
            .context("failed to decode Jupiter quote response")?;

        let in_raw = response
            .in_amount
            .parse::<u64>()
            .context("invalid inAmount in Jupiter response")?;
        let out_raw = response
            .out_amount
            .parse::<u64>()
            .context("invalid outAmount in Jupiter response")?;

        let in_ui = from_raw_amount(in_raw, input_decimals);
        let out_ui = from_raw_amount(out_raw, output_decimals);

        let price_impact_bps = response
            .price_impact_pct
            .parse::<f64>()
            .unwrap_or(0.0)
            * 10_000.0;

        Ok(Quote {
            venue: response
                .route_plan
                .as_ref()
                .and_then(|p| p.first())
                .map(|r| r.swap_info.label.clone())
                .unwrap_or_else(|| dex.to_string()),
            in_mint: input_symbol.to_string(),
            out_mint: output_symbol.to_string(),
            in_amount: in_ui,
            out_amount: out_ui,
            fee_quote: 0.0,
            slippage_bps: price_impact_bps,
            ts_ms: chrono::Utc::now().timestamp_millis(),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JupiterQuoteResponse {
    in_amount: String,
    out_amount: String,
    price_impact_pct: String,
    route_plan: Option<Vec<RoutePlanItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoutePlanItem {
    swap_info: SwapInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwapInfo {
    label: String,
}

fn mint_for_symbol(symbol: &str) -> Result<&'static str> {
    match symbol {
        "SOL" => Ok(SOL_MINT),
        "USDC" => Ok(USDC_MINT),
        _ => Err(anyhow!("unsupported token symbol: {symbol}")),
    }
}

fn decimals_for_symbol(symbol: &str) -> Result<u32> {
    match symbol {
        "SOL" => Ok(9),
        "USDC" => Ok(6),
        _ => Err(anyhow!("unsupported token symbol: {symbol}")),
    }
}

fn to_raw_amount(amount_ui: f64, decimals: u32) -> Result<u64> {
    if amount_ui <= 0.0 {
        return Err(anyhow!("input amount must be > 0"));
    }
    let scale = 10u64.pow(decimals) as f64;
    Ok((amount_ui * scale).round() as u64)
}

fn from_raw_amount(amount_raw: u64, decimals: u32) -> f64 {
    let scale = 10u64.pow(decimals) as f64;
    amount_raw as f64 / scale
}
