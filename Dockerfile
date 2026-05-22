FROM rust:1.86-slim AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

RUN cargo build --release

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
