.PHONY: help start stop restart logs status start-dev stop-dev restart-dev logs-dev status-dev local check test fmt clippy

help:
	@echo "Available targets:"
	@echo "  make start    - start via docker compose"
	@echo "  make stop     - stop via docker compose"
	@echo "  make restart  - restart service"
	@echo "  make logs     - follow service logs"
	@echo "  make status   - show compose status"
	@echo "  make start-dev   - start dev hot reload service"
	@echo "  make stop-dev    - stop dev hot reload service"
	@echo "  make restart-dev - restart dev hot reload service"
	@echo "  make logs-dev    - follow dev service logs"
	@echo "  make status-dev  - show dev compose status"
	@echo "  make local    - run locally with cargo"
	@echo "  make check    - cargo check"
	@echo "  make test     - cargo test"
	@echo "  make fmt      - cargo fmt --check"
	@echo "  make clippy   - cargo clippy strict"

start:
	./scripts/start.sh

stop:
	./scripts/stop.sh

restart: stop start

logs:
	docker compose logs -f trading-io

status:
	docker compose ps

start-dev:
	docker compose --profile dev up -d --build trading-io-dev

stop-dev:
	docker compose --profile dev stop trading-io-dev

restart-dev: stop-dev start-dev

logs-dev:
	docker compose --profile dev logs -f trading-io-dev

status-dev:
	docker compose --profile dev ps

local:
	cargo run

check:
	cargo check

test:
	cargo test

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all-targets --all-features -D warnings
