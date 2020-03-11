# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

# Install the tools needed to compile the rdkafka crate as the standard build 
# mechanism does not work. See installation instructions at https://crates.io/crates/rdkafka
RUN apt-get update && apt-get install musl-tools build-essential cmake -y && ln -s /usr/bin/g++ /bin/musl-g++

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/webhooks

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/webhooks*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine

# Problem with connecting in Kafka. Suspect user does not have enough provs. 
# Commenting out in the short term.
# RUN addgroup -g 1000 docker-user

# RUN adduser -D -s /bin/sh -u 1000 -G docker-user docker-user

WORKDIR /home/webhooks/bin/

COPY --from=cargo-build /usr/src/webhooks/target/x86_64-unknown-linux-musl/release/webhooks .

# RUN chown docker-user:docker-user webhooks

# USER docker-user

CMD ["./webhooks"]
