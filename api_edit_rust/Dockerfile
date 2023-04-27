FROM rust:latest as builder

RUN USER=root cargo new --bin api_edit_rust
WORKDIR /api_edit_rust

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY . ./

RUN rm ./target/release/deps/api_edit_rust*
RUN cargo build --release

FROM debian:buster-slim

RUN apt update
RUN apt install -y libpq5

COPY --from=builder /api_edit_rust/target/release/api_edit_rust .

CMD ["./api_edit_rust"]
