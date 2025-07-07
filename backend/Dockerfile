# Build stage
FROM rust:1.88-bookworm as chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim AS runtime

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/.containerized.env ./.env

CMD ["./backend"]
