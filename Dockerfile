FROM rust:1.86-slim AS base
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

FROM base AS deps
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src tests \
    && printf 'fn main() {}\n' > src/main.rs \
    && cargo build --release \
    && rm -rf src tests

FROM base AS builder
COPY --from=deps /app/target /app/target
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

RUN cargo build --release

FROM base AS dev
RUN cargo install cargo-watch --locked
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 10001 botuser

COPY --from=builder /app/target/release/trading-io /usr/local/bin/trading-io

USER botuser
ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/trading-io"]
