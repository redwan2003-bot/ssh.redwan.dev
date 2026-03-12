# ── Build stage ──────────────────────────────────────
FROM rust:1.88-bookworm AS builder

WORKDIR /app

# Install cmake for aws-lc-sys
RUN apt-get update && apt-get install -y cmake && rm -rf /var/lib/apt/lists/*

# Cache dependencies by building them first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build the actual binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ── Runtime stage ────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ssh-yannickh-dev /usr/local/bin/ssh-portfolio

ENV PORT=22
EXPOSE 22

CMD ["ssh-portfolio"]
