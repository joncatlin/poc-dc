# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

# Install the tools needed to compile the rdkafka crate as the standard build 
# mechanism does not work. See installation instructions at https://crates.io/crates/rdkafka
RUN apt-get update && apt-get install musl-tools build-essential cmake -y && ln -s /usr/bin/g++ /bin/musl-g++

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/test_channel_sender

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/test_channel_sender*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

EXPOSE 8080

# Create user
RUN addgroup -g 1000 myapp
RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

WORKDIR /home/test_channel_sender/bin/

# Copy the built executable
COPY --from=cargo-build /usr/src/test_channel_sender/target/x86_64-unknown-linux-musl/release/test_channel_sender .

RUN chown myapp:myapp test_channel_sender

USER myapp

ENTRYPOINT ["./test_channel_sender"]
# CMD ["./test_channel_sender"]
