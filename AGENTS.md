# Repository Guidelines

## Project Structure & Module Organization
This is a single-crate Rust project (`trading-io`) organized by domain under `src/`.

- `src/main.rs`: binary entry point.
- `src/config`, `src/core`, `src/data`, `src/strategy`, `src/risk`, `src/execution`, `src/portfolio`, `src/backtest`, `src/utils`: domain modules (each with `mod.rs` and focused implementation files).
- `tests/`: integration tests (currently `tests/smoke_test.rs`).
- `Cargo.toml`: dependencies and crate metadata.

When adding code, place it in the closest domain module instead of growing `main.rs`.

## Build, Test, and Development Commands
- `cargo run`: build and run the bot locally.
- `cargo check`: fast type and borrow checking without producing a binary.
- `cargo test`: run unit and integration tests in `src/` and `tests/`.
- `cargo fmt`: format code using rustfmt defaults.
- `cargo clippy --all-targets --all-features -D warnings`: lint strictly and fail on warnings.

Run `cargo fmt`, `cargo clippy`, and `cargo test` before opening a PR.

## Coding Style & Naming Conventions
- Follow Rust 2021 idioms and rustfmt output (4-space indentation, trailing commas where formatter adds them).
- Use `snake_case` for functions/modules/files, `PascalCase` for structs/enums/traits, and `SCREAMING_SNAKE_CASE` for constants.
- Keep modules cohesive: strategy logic in `src/strategy`, risk checks in `src/risk`, etc.
- Prefer `anyhow::Result` at boundaries and specific error types internally when needed.

## Testing Guidelines
- Use `#[test]` for sync tests and `#[tokio::test]` for async paths.
- Put module-level unit tests near the code; cross-module behavior tests go in `tests/`.
- Name tests by behavior, e.g., `rejects_order_when_risk_limit_exceeded`.
- Cover new strategy, risk, and execution branches with at least one success and one failure case.

## Commit & Pull Request Guidelines
Current history uses short, imperative commit messages (for example, `project initial`). Continue with concise imperative subjects, ideally under 72 characters.

PRs should include:
- What changed and why.
- Linked issue/ticket (if applicable).
- Test evidence (`cargo test`, `cargo clippy`, `cargo fmt --check`).
- Notes on config or behavior changes.

## Security & Configuration Tips
Do not commit secrets, API keys, or exchange credentials. Keep runtime configuration environment-driven and validate inputs in `src/config/settings.rs`.
