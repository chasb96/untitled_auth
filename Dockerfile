FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin auth
WORKDIR /src/auth

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/auth*

COPY ./src ./src
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

WORKDIR /src

COPY --from=build_host /src/auth/target/release/auth ./auth

CMD ["./auth"]
