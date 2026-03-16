# ---------- Builder ----------
FROM rust:1.92 AS builder

WORKDIR /app

COPY . .

# Build the example binary
RUN cargo build --release -p hft-term --example usage


# ---------- Runtime ----------
FROM debian:bookworm

WORKDIR /app

# Copy compiled example binary
COPY --from=builder /app/target/release/examples/usage /usr/local/bin/usage

ENTRYPOINT ["usage"]


