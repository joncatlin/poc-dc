# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

# Install the tools needed to compile the rdkafka crate as the standard build 
# mechanism does not work. See installation instructions at https://crates.io/crates/rdkafka
RUN apt-get update && apt-get install musl-tools build-essential cmake -y && ln -s /usr/bin/g++ /bin/musl-g++

# Needed for Diesel
RUN apt-get install -y libpq-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/event-store

COPY Cargo.toml Cargo.toml


RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/event-store*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

# Add the postgres driver
RUN apk update
RUN apk add libpq

EXPOSE 8080

# Create user
RUN addgroup -g 1000 myapp
RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

WORKDIR /home/event-store/bin/

# Copy the built executable
COPY --from=cargo-build /usr/src/event-store/target/x86_64-unknown-linux-musl/release/event-store .

# Copy the environment file for the DB URL
COPY .env .

RUN chown myapp:myapp event-store
RUN chown myapp:myapp .env

USER myapp

CMD ["./event-store"]




# FROM rust:1.41

# RUN apt update -yqq && apt install -yqq cmake g++

# COPY ./ /event-store
# WORKDIR /event-store
# RUN rm .env && ls -l

# RUN cargo clean
# RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# CMD ./target/release/event-store







