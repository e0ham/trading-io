use crate::core::types::Candle;

pub trait MarketDataProvider {
    fn latest_candle(&self, symbol: &str) -> Option<Candle>;
}
