.PHONY: help start stop restart logs status local check test fmt clippy

help:
	@echo "Available targets:"
	@echo "  make start    - start via docker compose"
	@echo "  make stop     - stop via docker compose"
	@echo "  make restart  - restart service"
	@echo "  make logs     - follow service logs"
	@echo "  make status   - show compose status"
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
