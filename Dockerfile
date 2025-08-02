# Build stage
FROM rust:1.88-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/qr-api /app/qr-api

EXPOSE 3000

ENV HOST=0.0.0.0
ENV PORT=3000
ENV LOG_LEVEL=info

CMD ["./qr-api"]