FROM rust:latest as builder

WORKDIR /
COPY . .
RUN mkdir -p .cargo


RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /target/x86_64-unknown-linux-musl/release/imager /
COPY --from=builder /config.toml /
COPY --from=builder /config /.cargo/

CMD ["./imager"]