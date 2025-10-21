# Multi-stage build for minimal final image
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
# Touch main.rs to force rebuild of our code only
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage - minimal image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/maximize /app/maximize

# Copy configuration example
COPY config.example.json /app/config.example.json

# Create directory for token storage
RUN mkdir -p /app/.maximize

# Expose port
EXPOSE 8081

# Set environment variables
ENV RUST_LOG=info
ENV BIND_ADDRESS=0.0.0.0
ENV PORT=8081

# Run the binary
ENTRYPOINT ["/app/maximize"]
