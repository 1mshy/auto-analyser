# Multi-stage build for Rust backend
FROM rust:1.70 as backend-builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY migrations/ ./migrations/

# Build the application
RUN cargo build --release --bin server

# Build frontend
FROM node:18-alpine as frontend-builder

WORKDIR /app/frontend

# Copy package files
COPY frontend/package*.json ./
RUN npm ci

# Copy frontend source
COPY frontend/ ./
RUN npm run build

# Final runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/server ./server

# Copy frontend build
COPY --from=frontend-builder /app/frontend/build ./frontend/build

# Copy migrations
COPY migrations/ ./migrations/

# Create data directory for SQLite database
RUN mkdir -p /app/data

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:/app/data/analysis.db

# Expose ports
EXPOSE 3001

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/api/health || exit 1

# Start the server
CMD ["./server"]