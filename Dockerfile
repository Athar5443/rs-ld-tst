FROM rust:1.75-slim-bookworm AS builder
RUN apt-get update && apt-get install -y \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN git clone https://github.com/Athar5443/rs-ld-tst .

RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/my_server /app/server

ENV PORT=8080
EXPOSE ${PORT}

# Menjalankan server
CMD ["./server"]
