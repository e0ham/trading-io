# trading-io

Rust-based trading bot foundation.

## Structure

- `src/config`: app configuration loading and validation
- `src/core`: shared domain types/events
- `src/data`: market data providers and normalization
- `src/strategy`: signal generation logic
- `src/risk`: risk checks and position sizing
- `src/execution`: order routing/exchange adapters
- `src/portfolio`: positions, balances, and PnL
- `src/backtest`: offline strategy simulation
- `src/trading`: runtime trading loop and controller
- `src/control`: Telegram bot control surface
- `src/utils`: common helpers
- `tests`: integration tests

## Run (Local)

```bash
cargo run
```

## Telegram Control

Set environment variables:

```bash
export TELEGRAM_ENABLED=true
export TELEGRAM_BOT_TOKEN="<your-bot-token>"
export TELEGRAM_ALLOWED_CHAT_IDS="123456789,-1001234567890"
export APP_MODE=paper
export BASE_TOKEN=SOL
export QUOTE_TOKEN=USDC
export TRADE_SIZE_QUOTE=100
export MIN_PROFIT_BPS=15
export MAX_SLIPPAGE_BPS=20
export MAX_QUOTE_AGE_MS=500
export LOOP_INTERVAL_MS=1000
export ESTIMATED_TX_COST_QUOTE=0.01
export LATENCY_BUFFER_QUOTE=0.005
export DEXES="Raydium,Orca+V2,Meteora+DLMM"
export JUPITER_QUOTE_URL="https://api.jup.ag/swap/v1/quote"
```

Then run:

```bash
cargo run
```

Available bot commands:
- `/help`
- `/starttrading`
- `/stoptrading`
- `/status`
- `/trades`

Security:
- Set `TELEGRAM_ALLOWED_CHAT_IDS` to a comma-separated list of allowed chat IDs.
- If empty, bot accepts commands from any chat.

## Run Scripts

Use helper scripts:

```bash
./scripts/start.sh
./scripts/stop.sh
```

Or with Make:

```bash
make start
make stop
make logs
```

## Run With Docker

1. Create `.env` from example and set your bot token:

```bash
cp .env.example .env
```

2. Build and start:

```bash
docker compose up -d --build
```

3. Check logs:

```bash
docker compose logs -f trading-io
```

4. Stop:

```bash
docker compose down
```

## Docker Dev Hot Reload

Use the dev profile to auto-restart the Rust app on code changes:

```bash
docker compose --profile dev up --build trading-io-dev
```

The dev service mounts the project directory and runs `cargo watch -x run`.
Cargo registry, git, and build target directories are persisted using named Docker volumes for faster incremental builds.

## Hetzner Deployment Notes

- Keep `TELEGRAM_BOT_TOKEN` only in `.env` (never commit it).
- Restrict VM ingress (SSH only; no public app port needed for polling bot mode).
- Set conservative trading/rate parameters first (`APP_MODE=paper`, low `TRADE_SIZE_QUOTE`).
- Monitor CPU/network and logs to avoid abusive traffic patterns.

## Architecture

- See `docs/ARCHITECTURE.md` for platform support and component design.
- Runtime now uses real Jupiter quotes for opportunity detection; execution remains paper/signal mode.
