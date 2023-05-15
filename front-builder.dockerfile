FROM rust:1.69-bullseye as development

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-watch
RUN cargo install trunk

COPY ./front ./front


FROM development as builder

RUN trunk build
