# syntax=docker/dockerfile:1
FROM rust:latest AS FETCH_THE_EFFIN_RUST
RUN rustup default nightly
WORKDIR /app
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./src/lib.rs /app/src/lib.rs
RUN cargo fetch

COPY ./src /app/src
RUN cargo build --release --bin testies
COPY ./run-client /app
CMD ["./run-client"]
