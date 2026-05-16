use crate::core::types::Candle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub trait Strategy {
    fn on_candle(&self, candle: &Candle) -> Signal;
}
