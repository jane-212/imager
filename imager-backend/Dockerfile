FROM rust:latest as builder

WORKDIR /
COPY . .
RUN mkdir -p .cargo
COPY ./config /.cargo/config

RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /target/release/imager /imager
COPY --from=builder /config.toml /config.toml

CMD ["./imager"]