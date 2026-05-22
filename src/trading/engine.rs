use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::{
    sync::{Mutex, RwLock},
    task::JoinHandle,
    time::{sleep, Duration},
};

use crate::{
    config::settings::Settings,
    core::types::{ArbitrageOpportunity, TradeExecution, TradingStatus},
    data::provider::JupiterQuoteProvider,
};

#[derive(Clone)]
pub struct TradingController {
    settings: Settings,
    status: Arc<RwLock<TradingStatus>>,
    trades: Arc<Mutex<Vec<TradeExecution>>>,
    running: Arc<AtomicBool>,
    task: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl TradingController {
    pub fn new(settings: Settings) -> Self {
        let status = TradingStatus::new(settings.app.mode.clone());
        Self {
            settings,
            status: Arc::new(RwLock::new(status)),
            trades: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(AtomicBool::new(false)),
            task: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(&self) -> String {
        if self.running.swap(true, Ordering::SeqCst) {
            return "trading is already running".to_string();
        }

        {
            let mut status = self.status.write().await;
            status.running = true;
            status.last_error = None;
        }

        let settings = self.settings.clone();
        let status = Arc::clone(&self.status);
        let trades = Arc::clone(&self.trades);
        let running = Arc::clone(&self.running);

        let handle = tokio::spawn(async move {
            let provider = JupiterQuoteProvider::new(
                settings.trading.jupiter_quote_url.clone(),
                settings.trading.max_slippage_bps as u16,
            );
            while running.load(Ordering::SeqCst) {
                let result = tick(&settings, &provider, &status, &trades).await;
                if let Err(err) = result {
                    let mut st = status.write().await;
                    st.last_error = Some(err.to_string());
                }
                sleep(Duration::from_millis(settings.trading.loop_interval_ms)).await;
            }
        });

        let mut task_lock = self.task.lock().await;
        *task_lock = Some(handle);

        "trading started".to_string()
    }

    pub async fn stop(&self) -> String {
        if !self.running.swap(false, Ordering::SeqCst) {
            return "trading is already stopped".to_string();
        }

        {
            let mut status = self.status.write().await;
            status.running = false;
        }

        if let Some(handle) = self.task.lock().await.take() {
            handle.abort();
        }

        "trading stopped".to_string()
    }

    pub async fn status_text(&self) -> String {
        let st = self.status.read().await;
        format!(
            "running: {}\nmode: {}\nscans: {}\nopportunities: {}\nexecuted: {}\nrejected_by_risk: {}\nlast_scan_ms: {}\nlast_error: {}",
            st.running,
            st.mode,
            st.scans,
            st.opportunities,
            st.executed,
            st.rejected_by_risk,
            st.last_scan_ms
                .map(|v| v.to_string())
                .unwrap_or_else(|| "n/a".to_string()),
            st.last_error.clone().unwrap_or_else(|| "none".to_string())
        )
    }

    pub async fn recent_trades_text(&self, limit: usize) -> String {
        let trades = self.trades.lock().await;
        if trades.is_empty() {
            return "no trades yet".to_string();
        }

        let lines: Vec<String> = trades
            .iter()
            .rev()
            .take(limit)
            .map(|t| {
                format!(
                    "{} -> {} | amount {:.2} | pnl {:.4} | {} | {}",
                    t.buy_venue, t.sell_venue, t.amount_quote, t.expected_pnl_quote, t.status, t.tx_id
                )
            })
            .collect();

        lines.join("\n")
    }
}

async fn tick(
    settings: &Settings,
    provider: &JupiterQuoteProvider,
    status: &Arc<RwLock<TradingStatus>>,
    trades: &Arc<Mutex<Vec<TradeExecution>>>,
) -> anyhow::Result<()> {
    let dexes = if settings.trading.dexes.is_empty() {
        vec!["Raydium".to_string(), "Orca+V2".to_string(), "Meteora+DLMM".to_string()]
    } else {
        settings.trading.dexes.clone()
    };

    {
        let mut st = status.write().await;
        st.scans += 1;
        st.last_scan_ms = Some(chrono::Utc::now().timestamp_millis());
    }

    let mut best: Option<ArbitrageOpportunity> = None;

    for buy_dex in &dexes {
        let buy_quote = match provider
            .quote_exact_in(
                buy_dex,
                &settings.trading.quote_token,
                &settings.trading.base_token,
                settings.trading.trade_size_quote,
            )
            .await
        {
            Ok(q) => q,
            Err(_) => continue,
        };

        for sell_dex in &dexes {
            if sell_dex == buy_dex {
                continue;
            }

            let sell_quote = match provider
                .quote_exact_in(
                    sell_dex,
                    &settings.trading.base_token,
                    &settings.trading.quote_token,
                    buy_quote.out_amount,
                )
                .await
            {
                Ok(q) => q,
                Err(_) => continue,
            };

            let now_ms = chrono::Utc::now().timestamp_millis();
            let buy_age_ms = now_ms.saturating_sub(buy_quote.ts_ms) as u64;
            let sell_age_ms = now_ms.saturating_sub(sell_quote.ts_ms) as u64;
            if buy_age_ms > settings.trading.max_quote_age_ms
                || sell_age_ms > settings.trading.max_quote_age_ms
            {
                continue;
            }

            let candidate = evaluate_opportunity(settings, buy_dex, sell_dex, &buy_quote, &sell_quote);
            if candidate.net_pnl_quote <= 0.0 {
                continue;
            }

            if best
                .as_ref()
                .map(|b| candidate.net_pnl_quote > b.net_pnl_quote)
                .unwrap_or(true)
            {
                best = Some(candidate);
            }
        }
    }

    let Some(opportunity) = best else {
        return Ok(());
    };

    {
        let mut st = status.write().await;
        st.opportunities += 1;
    }

    if opportunity.expected_profit_bps < settings.trading.min_profit_bps {
        let mut st = status.write().await;
        st.rejected_by_risk += 1;
        return Ok(());
    }

    let trade = TradeExecution {
        buy_venue: opportunity.buy_venue.clone(),
        sell_venue: opportunity.sell_venue.clone(),
        amount_quote: opportunity.start_quote_amount,
        expected_pnl_quote: opportunity.net_pnl_quote,
        status: if settings.app.mode == "live" {
            "signal_only".to_string()
        } else {
            "simulated_fill".to_string()
        },
        tx_id: format!("{}-{}", settings.app.mode, chrono::Utc::now().timestamp_millis()),
        ts_ms: chrono::Utc::now().timestamp_millis(),
    };

    {
        let mut st = status.write().await;
        st.executed += 1;
    }

    let mut log = trades.lock().await;
    log.push(trade);
    if log.len() > 100 {
        let excess = log.len() - 100;
        log.drain(0..excess);
    }

    Ok(())
}

fn evaluate_opportunity(
    settings: &Settings,
    buy_dex: &str,
    sell_dex: &str,
    buy_quote: &crate::core::types::Quote,
    sell_quote: &crate::core::types::Quote,
) -> ArbitrageOpportunity {
    let start = settings.trading.trade_size_quote;
    let end_quote = sell_quote.out_amount;
    let gross_pnl = end_quote - start;

    let estimated_slippage_cost = start * ((buy_quote.slippage_bps + sell_quote.slippage_bps) / 10_000.0);
    let tx_cost = settings.trading.estimated_tx_cost_quote;
    let latency_cost = settings.trading.latency_buffer_quote;

    let net = gross_pnl - estimated_slippage_cost - tx_cost - latency_cost;
    let bps = if start > 0.0 { (net / start) * 10_000.0 } else { 0.0 };

    ArbitrageOpportunity {
        buy_venue: buy_dex.to_string(),
        sell_venue: sell_dex.to_string(),
        start_quote_amount: start,
        end_quote_amount: end_quote,
        gross_pnl_quote: gross_pnl,
        net_pnl_quote: net,
        expected_profit_bps: bps,
        ts_ms: chrono::Utc::now().timestamp_millis(),
    }
}
