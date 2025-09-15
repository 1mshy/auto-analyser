#!/bin/bash

# Equity Analyser - Full Stack Startup Script
echo "🚀 Starting Equity Analyser Full Stack Application"

# Check if PostgreSQL is running
echo "📊 Checking PostgreSQL..."
if ! docker ps | grep -q postgres; then
    echo "🔧 Starting PostgreSQL with Docker Compose..."
    docker-compose up -d postgres
    echo "⏳ Waiting for PostgreSQL to be ready..."
    sleep 5
fi

# Start the Rust backend
echo "🦀 Starting Rust backend server..."
cd /Users/lucalapenna/Documents/GitHub/auto-analyser
cargo build --release
nohup cargo run --release > backend.log 2>&1 &
BACKEND_PID=$!
echo "✅ Backend started with PID: $BACKEND_PID"

# Wait for backend to be ready
echo "⏳ Waiting for backend to be ready..."
sleep 10

# Start the React frontend
echo "⚛️  Starting React frontend..."
cd auto-front
npm run build
nohup npm run preview > frontend.log 2>&1 &
FRONTEND_PID=$!
echo "✅ Frontend started with PID: $FRONTEND_PID"

echo ""
echo "🎉 Equity Analyser is now running!"
echo "📊 Backend API: http://localhost:3000"
echo "🌐 Frontend App: http://localhost:4173"
echo ""
echo "📝 Logs:"
echo "   Backend: /Users/lucalapenna/Documents/GitHub/auto-analyser/backend.log"
echo "   Frontend: /Users/lucalapenna/Documents/GitHub/auto-analyser/auto-front/frontend.log"
echo ""
echo "🛑 To stop the application:"
echo "   kill $BACKEND_PID $FRONTEND_PID"
echo "   docker-compose down"
echo ""

# Save PIDs for easy cleanup
echo "$BACKEND_PID" > .backend_pid
echo "$FRONTEND_PID" > auto-front/.frontend_pid

echo "🎯 Application ready! Open http://localhost:4173 in your browser"
