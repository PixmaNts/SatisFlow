# Dockerfile
FROM rust:1.80 AS builder

WORKDIR /app
COPY Cargo.toml ./
COPY crates ./crates

# Generate a new lock file to avoid version conflicts
RUN cargo generate-lockfile

# Build the application
RUN cargo build --release --package satisflow-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false satisflow

# Copy the binary from builder stage
COPY --from=builder /app/target/release/satisflow-server /usr/local/bin/

# Change ownership to non-root user
RUN chown satisflow:satisflow /usr/local/bin/satisflow-server

# Switch to non-root user
USER satisflow

# Expose port (configurable via environment variable)
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info
ENV PORT=3000
ENV HOST=0.0.0.0
ENV ENVIRONMENT=production

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:${PORT:-3000}/health || exit 1

# Run the application
CMD ["satisflow-server"]