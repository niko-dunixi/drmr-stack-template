# syntax=docker/dockerfile:1.3
ARG RUST_VERSION=1.56.1
FROM rust:${RUST_VERSION} as build
# FROM ekidd/rust-musl-builder:nightly-2021-02-13 as build
# RUN rustup default nightly
# ENV TARGET=musl \
#     PKG_CONFIG_ALLOW_CROSS=true \
#     PKG_CONFIG_ALL_STATIC=true
# RUN ln -s "/usr/bin/g++" "/usr/bin/musl-g++"
# RUN rustup target add x86_64-unknown-linux-musl
COPY . .
# RUN --mount=type=cache,target=$CARGO_HOME/registry --mount=type=cache,target=target \
#     rm -rf ./target/* && exit 1
RUN --mount=type=cache,target=$CARGO_HOME/registry --mount=type=cache,target=target \
    cargo test
RUN --mount=type=cache,target=$CARGO_HOME/registry --mount=type=cache,target=target \
    cargo build --release && cp target/release/server /server
EXPOSE 8080
ENTRYPOINT [ "./server" ]
CMD [ ]
# cargo build --release --target x86_64-unknown-linux-musl && cp target/release/server /server
# cargo build --release && cp target/release/server ./server
# cargo build --release && cp target/release/server /server && (ls -alh ./target/release/ && exit 1)
# cargo build --release --target x86_64-unknown-linux-musl && cp target/release/server /server

# FROM alpine:3 as main
# FROM scratch as main
FROM rust:${RUST_VERSION} as main
WORKDIR /opt/srv
# RUN apk add --no-cache bash
COPY --from=build server server
COPY Rocket.toml Rocket.toml
EXPOSE 8080
ENTRYPOINT [ "./server" ]
CMD [ ]