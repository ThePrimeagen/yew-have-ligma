# syntax=docker/dockerfile:1
FROM rust:latest AS FETCH_THE_EFFIN_RUST
WORKDIR /app
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./src/lib.rs /app/src/lib.rs
COPY ./dist /app/dist
RUN rustup default nightly
RUN cargo fetch

COPY ./src ./src
COPY ./index.html /app/index.html
RUN cargo build --release --bin react-sucks
RUN cargo install --path .

FROM debian:latest
WORKDIR /app
RUN apt update && apt install -y ca-certificates
COPY --from=FETCH_THE_EFFIN_RUST /usr/local/cargo/bin/react-sucks /app
COPY --from=FETCH_THE_EFFIN_RUST /app/dist /app
COPY --from=FETCH_THE_EFFIN_RUST /app/index.html /app
RUN set -x
RUN ls -la .
CMD ["./react-sucks", "index.html"]


