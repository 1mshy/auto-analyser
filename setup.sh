#!/bin/bash

# Complete setup script for Equity Analyser
set -e

echo "🚀 Starting Equity Analyser Complete Setup"
echo "========================================="

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command_exists cargo; then
    echo "❌ Rust/Cargo not found. Please install from https://rustup.rs/"
    exit 1
fi
echo "✅ Rust/Cargo found"

if ! command_exists docker; then
    echo "❌ Docker not found. Please install Docker Desktop"
    exit 1
fi
echo "✅ Docker found"

if ! command_exists docker-compose; then
    echo "❌ Docker Compose not found. Please install Docker Compose"
    exit 1
fi
echo "✅ Docker Compose found"

# Start PostgreSQL
echo "🐘 Starting PostgreSQL database..."
docker-compose up postgres -d

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
until docker-compose exec postgres pg_isready -U postgres > /dev/null 2>&1; do
    sleep 1
done
echo "✅ PostgreSQL is ready"

# Install SQLx CLI if needed
if ! command_exists sqlx; then
    echo "🔧 Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
fi
echo "✅ SQLx CLI ready"

# Run migrations
echo "🗃️  Running database migrations..."
sqlx migrate run

# Build the application
echo "🔨 Building application..."
cargo build

echo ""
echo "🎉 Setup completed successfully!"
echo ""
echo "You can now run the application with:"
echo "  ./run.sh"
echo ""
echo "Or manually with:"
echo "  cargo run"
echo ""
echo "The application will be available at: http://localhost:3000"
echo ""
echo "📚 Features:"
echo "  - Market data fetching from Yahoo Finance"
echo "  - Technical indicators (SMA, EMA, RSI, MACD, Bollinger Bands)"
echo "  - User authentication and watchlists"
echo "  - Price and technical indicator alerts"
echo "  - Modern web interface"
echo ""
echo "🔄 Background processes:"
echo "  - Market data updates every 5 minutes"
echo "  - Alert evaluation every 1 minute"
