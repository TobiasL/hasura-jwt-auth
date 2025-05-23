FROM --platform=$BUILDPLATFORM rust:1.86 AS build

RUN apt-get update && apt-get install -y \
    g++-x86-64-linux-gnu libc6-dev-amd64-cross \
    g++-aarch64-linux-gnu libc6-dev-arm64-cross && \
    rm -rf /var/lib/apt/lists/*
    
RUN rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
    CC_x86_64_unknown_linux_gnu=x86_64-linux-gnu-gcc \
    CXX_x86_64_unknown_linux_gnu=x86_64-linux-gnu-g++ \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    CARGO_INCREMENTAL=0

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {print!(\"Placeholder for cargo build\");}" > src/main.rs

RUN cargo build --release

COPY ./migrations ./migrations
COPY ./src ./src

# Have to update the timestamp for Cargo to rebuild. See: https://github.com/rust-lang/cargo/issues/6529
RUN touch src/main.rs

RUN cargo install --target x86_64-unknown-linux-gnu --path .

RUN cargo install --target aarch64-unknown-linux-gnu --path .

FROM --platform=amd64 debian:stable AS final-amd64

COPY --from=build /app/target/x86_64-unknown-linux-gnu/release/hasura-jwt-auth .

FROM --platform=arm64 debian:stable AS final-arm64

COPY --from=build /app/target/aarch64-unknown-linux-gnu/release/hasura-jwt-auth .

FROM final-${TARGETARCH}

CMD ["./hasura-jwt-auth"]
