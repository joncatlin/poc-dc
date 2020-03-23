# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

# Install the tools needed to compile the rdkafka crate as the standard build 
# mechanism does not work. See installation instructions at https://crates.io/crates/rdkafka
RUN apt-get update && apt-get install musl-tools build-essential cmake -y && ln -s /usr/bin/g++ /bin/musl-g++

#RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/test-docker-file2

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/test-docker-file*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 myapp

RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

WORKDIR /home/test-docker-file2/bin/

COPY --from=cargo-build /usr/src/test-docker-file2/target/x86_64-unknown-linux-musl/release/test-docker-file2 .

RUN chown myapp:myapp test-docker-file2

USER myapp

CMD ["./test-docker-file2"]