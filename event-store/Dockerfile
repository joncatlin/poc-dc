FROM rust:1.41

RUN apt update -yqq && apt install -yqq cmake g++

COPY ./ /event-store
WORKDIR /event-store
RUN rm .env && ls -l

RUN cargo clean
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

CMD ./target/release/event-store
