# syntax=docker/dockerfile:1
FROM rust:latest AS FETCH_THE_EFFIN_RUST
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/lib.rs ./src/lib.rs
RUN rustup default nightly
RUN cargo fetch
COPY ./src ./src
RUN cargo build --release --bin fast-simple-testies
RUN cargo install --path .

FROM debian:latest
WORKDIR /app
RUN apt update && apt install -y ca-certificates
COPY --from=FETCH_THE_EFFIN_RUST /usr/local/cargo/bin/fast-simple-testies /app
CMD ["sh", "-c", "/app/fast-simple-testies", "100"]



