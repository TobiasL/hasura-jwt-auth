FROM rust:1.60 as build

RUN cargo install cargo-build-dependencies

RUN USER=root cargo new --bin hasura-jwt-auth

WORKDIR /hasura-jwt-auth

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY ./migrations ./migrations
COPY ./src ./src

RUN cargo build --release --bin hasura-jwt-auth

FROM debian:stable as runner

COPY --from=build /hasura-jwt-auth/target/release/hasura-jwt-auth .

CMD ["./hasura-jwt-auth"]
