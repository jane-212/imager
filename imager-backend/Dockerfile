FROM rust:latest as builder

WORKDIR /build
COPY . .
RUN mkdir -p /.cargo
COPY ./config /.cargo/config

RUN cargo build --release

FROM alpine:latest

WORKDIR /app
COPY --from=builder /build/target/release/imager ./imager
COPY --from=builder /build/config.toml ./config.toml

CMD ["./imager"]