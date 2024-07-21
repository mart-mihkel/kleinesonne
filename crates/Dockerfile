FROM rust:1.79-alpine AS build
WORKDIR /usr/src/app

COPY . .
RUN apk add musl-dev
RUN cargo build
RUN cargo build --release --package api --bin api



FROM build AS dev

ENTRYPOINT cargo run api



FROM alpine AS prod
COPY --from=build /usr/src/app/target/release/api /usr/local/bin/api

EXPOSE 3000
ENTRYPOINT api
