#!/bin/bash

# Equity Analyser - Stop Script
echo "🛑 Stopping Equity Analyser Application"

# Stop backend if PID file exists
if [ -f .backend_pid ]; then
    BACKEND_PID=$(cat .backend_pid)
    echo "🦀 Stopping backend (PID: $BACKEND_PID)..."
    kill $BACKEND_PID 2>/dev/null
    rm .backend_pid
fi

# Stop frontend if PID file exists
if [ -f auto-front/.frontend_pid ]; then
    FRONTEND_PID=$(cat auto-front/.frontend_pid)
    echo "⚛️  Stopping frontend (PID: $FRONTEND_PID)..."
    kill $FRONTEND_PID 2>/dev/null
    rm auto-front/.frontend_pid
fi

# Stop PostgreSQL
echo "📊 Stopping PostgreSQL..."
docker-compose down

echo "✅ Equity Analyser stopped successfully!"
