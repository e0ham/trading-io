use crate::strategy::engine::Signal;

pub trait Broker {
    fn execute(&self, symbol: &str, signal: Signal) -> anyhow::Result<()>;
}
