# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml ./
COPY migration/Cargo.toml ./migration/

# Copy source code
COPY src ./src
COPY migration/src ./migration/src

# Build the application
RUN cargo build --release --bin rust_be

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app app

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder --chown=app:app /app/target/release/rust_be /app/rust_be

# Switch to app user
USER app

EXPOSE 3000

CMD ["./rust_be"]