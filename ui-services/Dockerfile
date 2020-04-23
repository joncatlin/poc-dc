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

#RUN apt-get install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/ui-services

COPY Cargo.toml Cargo.toml


RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/ui-services*

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
RUN addgroup -g 1000 docker-user
RUN adduser -D -s /bin/sh -u 1000 -G docker-user docker-user

# RUN chown myapp:myapp channel_sender
# RUN chown myapp:myapp *.json

USER docker-user

WORKDIR /home/docker-user
RUN pwd

# Copy the built executable
COPY --from=cargo-build /usr/src/ui-services/target/x86_64-unknown-linux-musl/release/ui-services .

# Copy the environment file for the DB URL
COPY .env .

USER root

RUN chown docker-user:docker-user ui-services
RUN chown docker-user:docker-user .env

USER docker-user

# Create a volume for the template temp directory where files are uploaded 
RUN mkdir ./template_temp
VOLUME ["/home/docker-user/template_temp"]

RUN ls -la
CMD ["./ui-services"]