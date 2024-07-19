FROM rust:1.79-alpine AS build
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release --package api --bin api



FROM alpine AS prod
COPY --from=build /usr/src/app/target/release/api /usr/local/bin/api

USER api
EXPOSE 3000/tcp
ENTRYPOINT [ "api" ]
