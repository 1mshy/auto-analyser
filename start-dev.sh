#!/bin/bash

# Auto Stock Analyser - Development Startup Script
# This script starts both the Rust backend API and React frontend

echo "🚀 Starting Auto Stock Analyser Development Environment"
echo "======================================================="

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo (Rust) is not installed. Please install Rust first:"
    echo "   https://rustup.rs/"
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install Node.js first:"
    echo "   https://nodejs.org/"
    exit 1
fi

# Function to cleanup background processes
cleanup() {
    echo
    echo "🛑 Shutting down services..."
    jobs -p | xargs -r kill
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Start the Rust backend API server
echo "🔧 Building and starting Rust backend API..."
cargo build --bin server

if [ $? -ne 0 ]; then
    echo "❌ Failed to build Rust backend"
    exit 1
fi

# Start backend in background
cargo run --bin server &
BACKEND_PID=$!

# Wait a moment for backend to start
sleep 3

# Check if backend is running
if ! kill -0 $BACKEND_PID 2>/dev/null; then
    echo "❌ Failed to start backend server"
    exit 1
fi

echo "✅ Backend API running on http://127.0.0.1:3001"

# Install frontend dependencies if node_modules doesn't exist
if [ ! -d "frontend/node_modules" ]; then
    echo "📦 Installing frontend dependencies..."
    cd frontend
    npm install
    cd ..
fi

# Start the React frontend
echo "🎨 Starting React frontend..."
cd frontend
npm start &
FRONTEND_PID=$!

# Wait a moment for frontend to start
sleep 5

echo
echo "✅ Development environment ready!"
echo "📊 Frontend Dashboard: http://localhost:3000"
echo "🔧 Backend API: http://127.0.0.1:3001"
echo
echo "Press Ctrl+C to stop all services"

# Wait for background processes
wait
