FROM rust:latest as builder

RUN USER=root cargo new --bin dead_poet_society
WORKDIR /dead_poet_society

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY . ./

RUN rm ./target/release/deps/dead_poet_society*
RUN cargo build --release

FROM debian:buster-slim

RUN apt update
RUN apt install -y libpq5

COPY --from=builder /dead_poet_society/target/release/dead_poet_society .

CMD ["./dead_poet_society"]
