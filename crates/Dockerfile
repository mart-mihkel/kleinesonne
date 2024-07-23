FROM rust:1.79-alpine AS build
WORKDIR /usr/src/app

COPY . .
RUN apk add musl-dev
RUN cargo build --release --package api --bin api



FROM alpine
ENV UPLOAD_DIR=/usr/src/app/uploads
COPY --from=build /usr/src/app/target/release/api /usr/local/bin/api

EXPOSE 3000
ENTRYPOINT [ "api" ]
