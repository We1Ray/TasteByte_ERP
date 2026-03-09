# ============================================================
# TasteByte ERP — Rust/Axum Backend
# Multi-stage build for minimal production image
# ============================================================

# Stage 1: Builder
FROM rust:1.84-bookworm AS builder
WORKDIR /app

# Copy manifests first to cache dependency builds
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to compile dependencies
RUN mkdir src && echo 'fn main(){}' > src/main.rs
RUN cargo build --release && rm -rf src

# Copy actual source and migrations
COPY src ./src
COPY migrations ./migrations

# Touch main.rs so cargo detects the change and rebuilds
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime — minimal Debian slim image
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --system appgroup && useradd --system --gid appgroup appuser

WORKDIR /app

COPY --from=builder /app/target/release/backend ./backend
COPY --from=builder /app/migrations ./migrations

# Create uploads directory
RUN mkdir -p uploads && chown -R appuser:appgroup /app

USER appuser

EXPOSE 8000

ENV RUST_LOG=info

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

CMD ["./backend"]
