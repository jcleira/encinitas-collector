FROM rust:latest as builder

RUN apt-get update && apt-get install -y musl-tools

# Set up for static build
RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /usr/src/encinitas-collector
COPY . .

RUN cargo build --release --target aarch64-unknown-linux-musl

FROM debian:buster-slim
COPY --from=builder /usr/src/encinitas-collector/target/aarch64-unknown-linux-musl/release/encinitas-collector /usr/local/bin/encinitas-collector

CMD ["encinitas-collector"]

