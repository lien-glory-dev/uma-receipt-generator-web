FROM rust:1.69-bullseye as development

WORKDIR /app
RUN apt-get update \
 && apt-get install -y \
      libopencv-dev \
      clang \
      libclang-dev \
 && apt-get -y clean

RUN cargo install cargo-watch
COPY ./ .


FROM development as builder

RUN cargo build --release

FROM rust:1.69-slim-bullseye

COPY --from=builder /app/server/target/release/server /usr/local/bin/server
EXPOSE 80
CMD ["/usr/local/bin/server"]
