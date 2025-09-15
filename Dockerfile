# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this step is cached unless Cargo.toml changes)
RUN cargo build --release

# Remove dummy main.rs
RUN rm src/main.rs

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY static ./static

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1001 appuser

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/auto-analyser ./
COPY --from=builder /app/static ./static

# Change ownership to appuser
RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

CMD ["./auto-analyser"]
