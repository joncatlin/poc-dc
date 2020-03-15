# This works
FROM rust:1.41

RUN apt update -yqq && apt install -yqq cmake g++

COPY ./ /ui-services
WORKDIR /ui-services
RUN rm .env && ls -l

RUN cargo clean
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

EXPOSE 8080

CMD ./target/release/ui-services


