#!/bin/bash

echo "Setting up the Auto Analyser with Stock Database functionality..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start Docker and try again."
    exit 1
fi

# Start PostgreSQL with Docker Compose
echo "Starting PostgreSQL database..."
docker-compose up -d postgres

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to start..."
sleep 10

# Check if PostgreSQL is ready
until docker exec $(docker-compose ps -q postgres) pg_isready -U postgres > /dev/null 2>&1; do
    echo "Waiting for PostgreSQL to be ready..."
    sleep 2
done

echo "PostgreSQL is ready!"

# Set environment variables
export DATABASE_URL="postgres://postgres:password@localhost:5432/equity_analyser"
export JWT_SECRET="your-secure-secret-key-change-in-production"
export PORT="3000"
export MARKET_DATA_INTERVAL_SECONDS="300"
export ALERT_CHECK_INTERVAL_SECONDS="60"

# Install SQLx CLI if not already installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Create the database if it doesn't exist
echo "Creating database..."
sqlx database create --database-url $DATABASE_URL 2>/dev/null || echo "Database already exists"

# Run migrations
echo "Running database migrations..."
sqlx migrate run --database-url $DATABASE_URL

echo "Setup complete!"
echo ""
echo "You can now run the application with:"
echo "cargo run"
echo ""
echo "Or test the stock functionality with:"
echo "./test_stocks.sh"
echo ""
echo "The application will:"
echo "1. Automatically fetch all US stocks from NASDAQ and NYSE on startup"
echo "2. Update market data for ALL stocks (not just watchlist items)"
echo "3. Run background tasks to keep stock data fresh"
echo ""
echo "API endpoints available:"
echo "- GET /api/stocks - List all stocks (paginated)"
echo "- POST /api/stocks/refresh - Manually refresh stock database"
echo "- All existing endpoints for market data, watchlists, alerts, etc."
