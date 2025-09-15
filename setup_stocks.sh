#!/bin/bash

# Update dependencies first
echo "Building the project..."
cargo build

if [ $? -ne 0 ]; then
    echo "Build failed. Please fix the compilation errors first."
    exit 1
fi

# Check if Docker is running and start the database
echo "Starting PostgreSQL database with Docker..."
docker-compose up -d postgres

# Wait for postgres to be ready
echo "Waiting for PostgreSQL to be ready..."
sleep 5

# Run database migrations
echo "Running database migrations..."
DATABASE_URL="postgres://postgres:password@localhost/equity_analyser" cargo run --bin auto-analyser &
APP_PID=$!

# Give the app a moment to start and run migrations
sleep 10

# Kill the app for now - we just wanted to run migrations
kill $APP_PID 2>/dev/null

echo "Setup complete! You can now run the application with:"
echo "cargo run"
