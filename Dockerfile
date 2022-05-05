FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin dirstat-rs-exporter
WORKDIR ./dirstat-rs-exporter
USER root
ADD . ./
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN ls -al /home/rust/src/dirstat-rs-exporter/target/x86_64-unknown-linux-musl/release

FROM alpine:3.15

RUN apk add libressl-dev
COPY --from=builder /home/rust/src/dirstat-rs-exporter/target/x86_64-unknown-linux-musl/release/dirstat-rs-exporter /usr/bin/dirstat-rs-exporter

ENTRYPOINT  [ "dirstat-rs-exporter"  ]
