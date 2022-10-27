FROM rust:latest as builder

WORKDIR /
COPY . .
RUN mkdir -p .cargo
COPY ./config /.cargo/

RUN cargo build --release

FROM alpine:latest

WORKDIR /
COPY --from=builder /target/release/imager /
COPY --from=builder /config.toml /

CMD ["/imager"]