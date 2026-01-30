# Stage 1: Build
FROM rust:1.80-slim-bullseye as builder

WORKDIR /app

# Install build dependencies
# imagequant/webp often need C build tools
RUN apt-get update && apt-get install -y pkg-config libssl-dev gcc musl-tools

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir ./src && echo "fn main() {}" > ./src/main.rs
RUN cargo build --release
RUN rm ./src/main.rs

# Copy source code
COPY ./src ./src

# Build the actual application
# forcing a touch to ensure rebuild
RUN touch ./src/main.rs
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies (OpenSSL is often needed for reqwest/https)
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/image-compress-tool /usr/local/bin/image-compress-tool

# Default entrypoint
ENTRYPOINT ["image-compress-tool"]
CMD ["--help"]
