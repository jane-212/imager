FROM rust:latest as builder

COPY . .
RUN mkdir -p .cargo
COPY ./config /.cargo/config

WORKDIR /
RUN cargo build --release

FROM alpine:latest

COPY --from=builder /target/release/imager /imager
COPY --from=builder /config.toml /config.toml
WORKDIR /

CMD ["./imager"]