# -------------------------------------
# Base Image + Dependencies
# -------------------------------------
FROM rust:latest AS base

ENV PATH="/app/.cargo/bin:${PATH}"

# Create a new empty shell project
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
# Copy the Cargo.toml and Cargo.lock files
COPY ./core/Cargo.toml ./core/Cargo.lock ./
# Copy the source code
COPY ./core/src ./src
# COPY ./core/static ./static

# -------------------------------------
# Development Target
# -------------------------------------
FROM base AS dev

# Copy everything (including static assets)
COPY ./core .

# Install watchexec
RUN apt-get update && \
    cargo install watchexec-cli

ENV PATH="/app/.cargo/bin:${PATH}"

# Run Watchexec (This may be overwritten in compose.yml)
CMD ["watchexec", "--restart", "--watch", "src", "--exts", "rs", "cargo", "run"]

# -------------------------------------
# Production Target
# -------------------------------------
FROM base AS builder
# Build Application
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/core /app/core
# COPY ./core/static ./static
EXPOSE 8000
CMD ["/app/core"]
