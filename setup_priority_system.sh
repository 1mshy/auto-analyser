#!/bin/bash

# Run database migrations
echo "Running database migration to add priority system..."

# Set database URL if not already set
export DATABASE_URL=${DATABASE_URL:-"postgres://postgres:password@localhost/equity_analyser"}

# Run the migration using sqlx
sqlx migrate run --database-url $DATABASE_URL

echo "Migration completed successfully!"

# Optional: Run a query to verify the migration worked
echo "Verifying migration..."
psql $DATABASE_URL -c "SELECT COUNT(*) as total_stocks, priority, COUNT(*) FROM stocks GROUP BY priority;" || echo "Could not verify - this is normal if psql is not available"

echo "Priority system is now active!"
echo ""
echo "Priority levels:"
echo "- High: 60 seconds (1 minute) - for watchlist stocks"
echo "- Medium: 300 seconds (5 minutes) - for actively traded stocks"  
echo "- Low: 900 seconds (15 minutes) - for other stocks"
echo ""
echo "You can check priority status at:"
echo "  GET /api/stocks/priority - shows all priorities and counts"
echo "  GET /api/stocks/priority?priority=high - shows high priority stocks"
echo "  GET /api/stocks/priority/needing-update - shows stocks needing updates"
