FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin crabcraft

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

EXPOSE 25565
VOLUME ["/app"]
WORKDIR /app

COPY --from=builder /app/target/release/crabcraft /usr/local/bin/crabcraft

ENTRYPOINT ["/usr/local/bin/crabcraft"]