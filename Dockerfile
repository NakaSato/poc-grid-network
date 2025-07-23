# Multi-stage build for Thai Energy Trading Blockchain

# Stage 1: Builder stage
FROM rust:latest as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r thai-energy && useradd -r -g thai-energy thai-energy

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/thai-energy-trading-blockchain .

# Copy configuration files
COPY config/ ./config/

# Create necessary directories
RUN mkdir -p logs data && \
    chown -R thai-energy:thai-energy /app

# Switch to non-root user
USER thai-energy

# Expose ports
EXPOSE 8080 9090 9944

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ./thai-energy-trading-blockchain --health-check || exit 1

# Start the application
CMD ["./thai-energy-trading-blockchain"]
