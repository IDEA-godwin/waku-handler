FROM rust:1.88 AS builder

RUN apt-get update && apt-get install -y \
    curl git make gcc clang cmake build-essential \
    libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Provide Go 1.20 from the official golang image (multi-arch, deterministic)
COPY --from=golang:1.20 /usr/local/go /usr/local/go
ENV PATH="/usr/local/go/bin:${PATH}"
RUN go version
WORKDIR /app

COPY . .
RUN cargo fetch
RUN cargo build --release && strip target/release/waku-handler

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/waku-handler /usr/local/bin/waku-handler

CMD ["waku-handler"]