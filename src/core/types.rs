use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub venue: String,
    pub in_mint: String,
    pub out_mint: String,
    pub in_amount: f64,
    pub out_amount: f64,
    pub fee_quote: f64,
    pub slippage_bps: f64,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub buy_venue: String,
    pub sell_venue: String,
    pub start_quote_amount: f64,
    pub end_quote_amount: f64,
    pub gross_pnl_quote: f64,
    pub net_pnl_quote: f64,
    pub expected_profit_bps: f64,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    pub buy_venue: String,
    pub sell_venue: String,
    pub amount_quote: f64,
    pub expected_pnl_quote: f64,
    pub status: String,
    pub tx_id: String,
    pub ts_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingStatus {
    pub running: bool,
    pub mode: String,
    pub last_scan_ms: Option<i64>,
    pub scans: u64,
    pub opportunities: u64,
    pub executed: u64,
    pub rejected_by_risk: u64,
    pub last_error: Option<String>,
}

impl TradingStatus {
    pub fn new(mode: String) -> Self {
        Self {
            running: false,
            mode,
            last_scan_ms: None,
            scans: 0,
            opportunities: 0,
            executed: 0,
            rejected_by_risk: 0,
            last_error: None,
        }
    }
}
