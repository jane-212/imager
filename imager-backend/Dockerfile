FROM rust:lastest as builder

WORKDIR /
COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /target/x86_64-unknown-linux-musl/release/imager /
COPY --from=builder /config.toml /

CMD ["./imager"]