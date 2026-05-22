use chrono::Utc;

use crate::core::types::{ArbitrageOpportunity, TradeExecution};

pub trait Broker: Send + Sync {
    fn execute_arbitrage(&self, opportunity: &ArbitrageOpportunity) -> anyhow::Result<TradeExecution>;
}

#[derive(Debug, Clone)]
pub struct PaperBroker;

impl Broker for PaperBroker {
    fn execute_arbitrage(&self, opportunity: &ArbitrageOpportunity) -> anyhow::Result<TradeExecution> {
        let ts = Utc::now().timestamp_millis();
        Ok(TradeExecution {
            buy_venue: opportunity.buy_venue.clone(),
            sell_venue: opportunity.sell_venue.clone(),
            amount_quote: opportunity.start_quote_amount,
            expected_pnl_quote: opportunity.net_pnl_quote,
            status: "simulated_fill".to_string(),
            tx_id: format!("paper-{}", ts),
            ts_ms: ts,
        })
    }
}
