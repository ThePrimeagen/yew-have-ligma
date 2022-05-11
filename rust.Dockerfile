# syntax=docker/dockerfile:1
FROM rust:latest AS FETCH_THE_EFFIN_RUST
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/lib.rs ./src/lib.rs
RUN rustup default nightly
RUN cargo fetch
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

COPY ./src ./src
COPY ./index.html /app/index.html
RUN trunk build --dist dist
RUN cargo build --release --bin fast-simple-testies
RUN cargo install --path .

FROM debian:latest
WORKDIR /app
RUN apt update && apt install -y ca-certificates
COPY --from=FETCH_THE_EFFIN_RUST /usr/local/cargo/bin/fast-simple-testies /app
COPY --from=FETCH_THE_EFFIN_RUST /app/dist /app
COPY --from=FETCH_THE_EFFIN_RUST /app/index.html /app
RUN set -x
RUN ls -la .
CMD ["./fast-simple-testies", "100"]

