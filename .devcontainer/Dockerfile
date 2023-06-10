FROM rust:slim-bullseye

WORKDIR /workspace

RUN apt update && apt install -y git

RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

RUN cargo install trunk && cargo install --locked cargo-leptos
