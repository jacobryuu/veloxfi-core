# Build stage
FROM rust:1.83 as builder

WORKDIR /app

# Copy project files
COPY . .

# Build release binary (with Swagger enabled)
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/veloxfi-core /app/veloxfi-core

# Create data directory
RUN mkdir -p /app/data

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Run application
CMD ["/app/veloxfi-core"]
