FROM rust:latest as builder

RUN USER=root cargo new --bin sub_log
WORKDIR /sub_log

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY . ./

RUN rm ./target/release/deps/sub_log*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /sub_log/target/release/sub_log .

CMD ["./sub_log"]
