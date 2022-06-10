FROM rust:1.60 as build

RUN cargo install cargo-build-dependencies

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {print!(\"Placeholder for cargo build\");}" > src/main.rs

RUN cargo build --release

COPY ./migrations ./migrations
COPY ./src ./src

# Have to update the timestamp for Cargo to rebuild. See: https://github.com/rust-lang/cargo/issues/6529
RUN touch src/main.rs

RUN cargo build --release

FROM debian:stable as runner

COPY --from=build /app/target/release/hasura-jwt-auth .

CMD ["./hasura-jwt-auth"]
