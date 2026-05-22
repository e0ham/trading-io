use crate::core::types::ArbitrageOpportunity;

pub trait RiskManager: Send + Sync {
    fn approve(&self, opportunity: &ArbitrageOpportunity) -> bool;
}

#[derive(Debug, Clone)]
pub struct BasicRiskManager {
    pub max_notional_quote: f64,
    pub max_slippage_bps: f64,
}

impl RiskManager for BasicRiskManager {
    fn approve(&self, opportunity: &ArbitrageOpportunity) -> bool {
        if opportunity.start_quote_amount > self.max_notional_quote {
            return false;
        }

        if opportunity.expected_profit_bps <= 0.0 || self.max_slippage_bps <= 0.0 {
            return false;
        }

        true
    }
}
