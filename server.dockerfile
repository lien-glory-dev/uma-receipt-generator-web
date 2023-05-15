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

EXPOSE 80
CMD ["cargo run --release"]
