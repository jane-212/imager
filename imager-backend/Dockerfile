FROM rust:latest as builder

WORKDIR /build
COPY . .
RUN mkdir -p /.cargo
COPY ./config /.cargo/config

RUN cargo build --release

FROM alpine:latest

WORKDIR /app
COPY --from=builder /build/target/release/imager /app/imager
COPY --from=builder /config.toml /app/config.toml

CMD ["/app/imager"]