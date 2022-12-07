FROM ekidd/rust-musl-builder AS build

WORKDIR /app

ADD --chown=rust:rust . .

RUN sudo mkdir -p /.cargo \
 && sudo mv ./config /.cargo/config \
 && rustup target add x86_64-unknown-linux-musl \
 && cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest

WORKDIR /app

RUN mkdir conf

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/imager ./

VOLUME [ "/app/conf" ]

CMD ["/app/imager"]
