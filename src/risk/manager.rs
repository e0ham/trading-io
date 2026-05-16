use crate::strategy::engine::Signal;

pub trait RiskManager {
    fn approve(&self, signal: Signal) -> bool;
}
