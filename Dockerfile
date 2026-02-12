FROM rust:1.88 AS builder

RUN apt-get update && apt-get install -y \
    curl git make gcc clang cmake build-essential \
    libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Go (no gvm)
ENV GO_VERSION=1.20.5
RUN curl -fsSL https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz \
    | tar -C /usr/local -xz
ENV PATH="/usr/local/go/bin:${PATH}"

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