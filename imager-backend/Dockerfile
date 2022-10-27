FROM rust:latest as builder

WORKDIR /
COPY . .


RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /target/x86_64-unknown-linux-musl/release/imager /
COPY --from=builder /config.toml /
RUN mkdir -p .cargo
COPY --from=builder /config /.cargo/

CMD ["./imager"]