FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/my_server*

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from:builder /app/target/release/my_server /app/server
EXPOSE 8080

CMD ["./server"]
