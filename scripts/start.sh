#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if [[ ! -f .env ]]; then
  echo "[start] .env not found. Creating from .env.example"
  cp .env.example .env
  echo "[start] Set TELEGRAM_BOT_TOKEN in .env before starting."
fi

if ! command -v docker >/dev/null 2>&1; then
  echo "[start] docker is not installed"
  exit 1
fi

if ! docker compose version >/dev/null 2>&1; then
  echo "[start] docker compose is not available"
  exit 1
fi

echo "[start] Starting trading-io with docker compose"
docker compose up -d --build

echo "[start] Service started. Use: docker compose logs -f trading-io"
