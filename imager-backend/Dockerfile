FROM oldcai/rust-musl-builder:latest as builder
ADD . /
WORKDIR /

RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /target/x86_64-unknown-linux-musl/release/imager /
COPY --from=builder /config.toml /

CMD ["./imager"]