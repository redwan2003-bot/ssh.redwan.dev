# ── Build stage ──────────────────────────────────────
FROM rust:1.88-bookworm AS builder

WORKDIR /app

# Install cmake for aws-lc-sys and git for potential dependencies
RUN apt-get update && apt-get install -y cmake git && rm -rf /var/lib/apt/lists/*

# Cache dependencies by building them first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build the actual binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ── Runtime stage ────────────────────────────────────
FROM debian:bookworm-slim

# Install necessary packages for Railway environment
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user for security
RUN groupadd -r portfolio && useradd -r -g portfolio portfolio

COPY --from=builder /app/target/release/ssh-redwan-dev /usr/local/bin/ssh-portfolio

# Set proper permissions
RUN chmod +x /usr/local/bin/ssh-portfolio

# Use port 2222 to match the Rust app's default, but allow override via PORT env var
ENV PORT=2222
EXPOSE 2222

# Run as non-root user
USER portfolio

CMD ["ssh-portfolio"]
