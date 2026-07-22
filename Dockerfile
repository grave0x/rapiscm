# ── Builder stage ──────────────────────────────────────
FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY src/ src/

# Build with static linking, max optimisation
RUN cargo build --release --target x86_64-unknown-linux-musl \
    && strip target/x86_64-unknown-linux-musl/release/rapiscm

# ── Runtime stage ──────────────────────────────────────
FROM scratch

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/rapiscm /usr/local/bin/rapiscm
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

ENTRYPOINT ["/usr/local/bin/rapiscm"]
CMD ["--help"]
