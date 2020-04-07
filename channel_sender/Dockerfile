# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

RUN apt-get update

# Install the tools needed to compile the rdkafka crate as the standard build 
# mechanism does not work. See installation instructions at https://crates.io/crates/rdkafka
#RUN apt-get update && apt-get install musl-tools build-essential cmake -y && ln -s /usr/bin/g++ /bin/musl-g++
RUN apt-get update && apt-get install musl-tools build-essential cmake pkg-config libssl-dev -y && ln -s /usr/bin/g++ /bin/musl-g++

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/channel_sender

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/channel_sender*

COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

EXPOSE 8080

# Create user
RUN addgroup -g 1000 docker-user
RUN adduser -D -s /bin/sh -u 1000 -G docker-user docker-user

# RUN chown myapp:myapp channel_sender
# RUN chown myapp:myapp *.json

USER docker-user

# WORKDIR /home/channel_sender/bin/
WORKDIR /home/docker-user
RUN pwd

# Copy the built executable
COPY --from=cargo-build /usr/src/channel_sender/target/x86_64-unknown-linux-musl/release/channel_sender .
COPY *.json ./

# Create a volume for the templates
RUN mkdir ./templates
VOLUME ["/home/docker-user/templates"]

# Create a volume for the generated pdf files
RUN mkdir ./pdf-output
VOLUME ["/home/docker-user/pdf-output"]

# Create a dir for temp files generated in the pdf process
#RUN mkdir ./pdf-temp-files

CMD ["./channel_sender"]