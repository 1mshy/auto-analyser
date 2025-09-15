#!/bin/bash

# Equity Analyser Setup and Run Script

set -e

echo "🏗️  Setting up Equity Analyser..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if PostgreSQL is running (either local or Docker)
echo "🔍 Checking database connection..."

if command -v docker &> /dev/null && docker-compose ps postgres | grep -q "Up"; then
    echo "✅ PostgreSQL running in Docker"
elif pg_isready -h localhost -p 5432 &> /dev/null; then
    echo "✅ PostgreSQL running locally"
else
    echo "🐘 Starting PostgreSQL with Docker Compose..."
    docker-compose up postgres -d
    echo "⏳ Waiting for PostgreSQL to be ready..."
    until pg_isready -h localhost -p 5432 &> /dev/null; do
        sleep 1
    done
    echo "✅ PostgreSQL is ready"
fi

# Install SQLx CLI if not already installed
if ! command -v sqlx &> /dev/null; then
    echo "🔧 Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Set up environment variables
if [ ! -f .env ]; then
    echo "📝 Creating .env file..."
    cp .env .env.backup 2>/dev/null || true
fi

# Run database migrations
echo "🗃️  Running database migrations..."
sqlx migrate run

echo "📦 Building application..."
cargo build

echo "🚀 Starting Equity Analyser..."
echo "📍 The application will be available at: http://localhost:3000"
echo "🔄 Market data updates every 5 minutes"
echo "⚠️  Alert evaluation every 1 minute"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

# Run the application
RUST_LOG=info cargo run
