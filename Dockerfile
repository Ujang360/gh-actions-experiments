FROM alpine:3.13

WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/sayhello sayhello

RUN ln -s /app/sayhello /usr/bin/sayhello
