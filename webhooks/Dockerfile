FROM rust:1.40 as builder
# WORKDIR /usr/src/webhooks
# COPY . .
# RUN cargo install --path .

# FROM debian:buster-slim
# #RUN apt-get update && apt-get install -y extra-runtime-dependencies
# COPY --from=builder /usr/local/cargo/bin/webhooks /usr/local/bin/webhooks
# CMD ["webhooks"]









RUN cargo install cargo-build-deps
RUN cd /tmp && USER=root cargo new --bin webhooks
WORKDIR /tmp/webhooks
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release
COPY src /tmp/webhooks/src
RUN cargo build  --release
EXPOSE 80
#COPY ./target/release /usr/local/bin/webhooks
CMD ["./target/release/webhooks"]
