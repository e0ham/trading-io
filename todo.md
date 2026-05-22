# TODO: Solana DEX Arbitrage Bot

## 1) Configuration
- [ ] Define RPC, wallet, and network settings in `src/config/settings.rs`
- [ ] Add env-based config loading for keys and endpoints
- [ ] Add validation for required settings (RPC URL, keypair path, min profit bps)

## 2) Data Layer (`src/data`)
- [ ] Implement quote adapters for target Solana venues (Jupiter/Raydium/Orca)
- [ ] Normalize quote format (in/out amount, fees, slippage estimate, timestamp)
- [ ] Add periodic and event-driven market snapshot updates

## 3) Strategy (`src/strategy`)
- [ ] Implement 2-leg arbitrage detector across venues
- [ ] Implement triangular path scanner (`A -> B -> C -> A`)
- [ ] Add profit model: gross spread - trading fees - estimated tx cost - slippage buffer
- [ ] Emit actionable trade signals with route metadata

## 4) Risk (`src/risk`)
- [ ] Add max notional and per-token exposure limits
- [ ] Add min liquidity and max slippage guards
- [ ] Add daily loss cap and consecutive-failure circuit breaker

## 5) Execution (`src/execution`)
- [ ] Build Solana transaction composer for selected route
- [ ] Add signing, send, confirm, retry, and timeout handling
- [ ] Add priority fee/compute budget controls
- [ ] Store tx signatures and execution results

## 6) Portfolio & State (`src/portfolio`)
- [ ] Track balances, open exposure, realized/unrealized PnL
- [ ] Track fills and per-strategy performance metrics

## 7) Backtesting & Simulation (`src/backtest`)
- [ ] Create replay runner from historical quote snapshots
- [ ] Evaluate strategy performance with realistic fees/slippage
- [ ] Add paper-trading mode before live deploy

## 8) Testing
- [ ] Unit tests for quote normalization and profit math
- [ ] Integration tests for full signal-to-execution path
- [ ] Failure-path tests (RPC errors, stale quotes, partial fills)

## 9) Ops & Safety
- [ ] Structured logging via `tracing` with per-trade correlation IDs
- [ ] Add metrics (attempts, success rate, avg pnl, rejection reasons)
- [ ] Add kill switch and safe shutdown handler

## 10) MVP Milestones
- [ ] M1: Quote ingestion + 2-leg detector
- [ ] M2: Dry-run execution and risk gating
- [ ] M3: Paper trading on devnet/mainnet-beta (small size)
- [ ] M4: Controlled live rollout with strict limits
