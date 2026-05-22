# Architecture and Trading Platforms

## Platform Support Matrix

Current state (implemented now):

| Platform | Status | Notes |
|---|---|---|
| Solana | Partial (paper/signal mode) | Uses live Jupiter quote API for detection; execution is not on-chain yet. |
| Jupiter, Raydium, Orca, Meteora | Quote-integrated | Quotes come from Jupiter `swap/v1/quote` with per-DEX routing constraints. |
| Phoenix/OpenBook orderbooks | Planned | Not connected yet. |
| EVM DEXs (Uniswap, etc.) | Not supported | No EVM adapters in current codebase. |

## System Architecture

The runtime has been simplified into a direct flow with fewer abstractions.

1. Control layer (`src/control/telegram.rs`)
- Telegram commands start/stop trading and read status/trades.

2. Runtime layer (`src/trading/engine.rs`)
- Main loop: `quote -> detect opportunity -> risk check -> execute -> record trade`.

3. Data layer (`src/data/provider.rs`)
- `JupiterQuoteProvider` fetches real quotes from Jupiter.
- For each candidate pair, it gets:
  - Buy quote (`USDC -> SOL`) constrained to buy DEX
  - Sell quote (`SOL -> USDC`) constrained to sell DEX

4. Strategy layer (`src/strategy/engine.rs`)
- `SimpleArbitrageStrategy` evaluates 2-leg arbitrage profitability with fee/slippage/latency buffers.

5. Execution/risk (inside `src/trading/engine.rs`)
- Applies profitability threshold and quote-age checks.
- Records simulated fills in `paper` mode and signal-only events in `live` mode.

## Path to Live Solana Trading

To make Solana live-ready, replace mock components only:

- Keep `JupiterQuoteProvider` for pricing.
- Add a real broker for Solana transaction builder, signer, sender, and confirmer.
- Add wallet/key management, RPC failover, and retry policies.

The existing Telegram control and trading loop can remain unchanged.
