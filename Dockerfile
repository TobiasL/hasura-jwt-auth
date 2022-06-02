FROM rust:1.60 as build

RUN cargo install cargo-build-dependencies

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY ./migrations ./migrations
COPY ./src ./src

RUN cargo build --release

FROM debian:stable as runner

COPY --from=build /app/target/release/hasura-jwt-auth .

CMD ["./hasura-jwt-auth"]
