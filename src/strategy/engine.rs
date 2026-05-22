use crate::core::types::{ArbitrageOpportunity, Quote};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    ExecuteArbitrage,
    Hold,
}

pub trait ArbitrageStrategy: Send + Sync {
    fn detect(
        &self,
        quotes: &[Quote],
        min_profit_bps: f64,
        tx_cost_quote: f64,
        latency_buffer_quote: f64,
        max_quote_age_ms: u64,
    ) -> Option<ArbitrageOpportunity>;
}

#[derive(Debug, Default, Clone)]
pub struct SimpleArbitrageStrategy;

impl ArbitrageStrategy for SimpleArbitrageStrategy {
    fn detect(
        &self,
        quotes: &[Quote],
        min_profit_bps: f64,
        tx_cost_quote: f64,
        latency_buffer_quote: f64,
        max_quote_age_ms: u64,
    ) -> Option<ArbitrageOpportunity> {
        if quotes.len() < 2 {
            return None;
        }

        let now_ms = chrono::Utc::now().timestamp_millis();
        let fresh: Vec<&Quote> = quotes
            .iter()
            .filter(|q| now_ms.saturating_sub(q.ts_ms) as u64 <= max_quote_age_ms)
            .collect();

        if fresh.len() < 2 {
            return None;
        }

        let mut best: Option<ArbitrageOpportunity> = None;

        for buy in &fresh {
            for sell in &fresh {
                if buy.venue == sell.venue {
                    continue;
                }

                let start = buy.in_amount;
                let gross = (sell.out_amount - buy.in_amount).max(0.0);
                let fees = buy.fee_quote + sell.fee_quote;
                let slip = start * ((buy.slippage_bps + sell.slippage_bps) / 10_000.0);
                let net = gross - fees - tx_cost_quote - slip - latency_buffer_quote;
                let bps = (net / start) * 10_000.0;

                if bps < min_profit_bps {
                    continue;
                }

                let opp = ArbitrageOpportunity {
                    buy_venue: buy.venue.clone(),
                    sell_venue: sell.venue.clone(),
                    start_quote_amount: start,
                    end_quote_amount: start + gross,
                    gross_pnl_quote: gross,
                    net_pnl_quote: net,
                    expected_profit_bps: bps,
                    ts_ms: now_ms,
                };

                if best
                    .as_ref()
                    .map(|b| opp.net_pnl_quote > b.net_pnl_quote)
                    .unwrap_or(true)
                {
                    best = Some(opp);
                }
            }
        }

        best
    }
}
