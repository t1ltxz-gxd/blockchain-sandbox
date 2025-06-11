ARG APP_NAME=blockchain-sandbox

FROM rust:slim-bullseye AS builder
ARG APP_NAME

WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static
ARG APP_NAME

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/${APP_NAME} /usr/local/bin/app

ENTRYPOINT ["/usr/local/bin/app"]