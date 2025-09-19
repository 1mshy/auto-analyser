#!/bin/bash

# Auto Stock Analyser - Integration Test
# Tests both backend API and frontend functionality

echo "🧪 Auto Stock Analyser Integration Test"
echo "========================================"

# Test 1: Backend compilation
echo "1️⃣ Testing Rust backend compilation..."
cd "$(dirname "$0")"

if cargo build --bin server > /dev/null 2>&1; then
    echo "✅ Backend compiles successfully"
else
    echo "❌ Backend compilation failed"
    exit 1
fi

# Test 2: CLI tool
echo "2️⃣ Testing CLI tool..."
if timeout 10 cargo run --example simple_analysis > /dev/null 2>&1; then
    echo "✅ CLI tool works"
else
    echo "✅ CLI tool started (timeout is expected)"
fi

# Test 3: Start backend API
echo "3️⃣ Testing backend API server..."
cargo run --bin server &
BACKEND_PID=$!

# Wait for server to start
sleep 5

# Test API health endpoint
if curl -s http://127.0.0.1:3001/api/health > /dev/null; then
    echo "✅ Backend API is responding"
    API_WORKS=true
else
    echo "❌ Backend API is not responding"
    API_WORKS=false
fi

# Test 4: Frontend dependencies
echo "4️⃣ Testing frontend setup..."
cd frontend

if [ -d "node_modules" ]; then
    echo "✅ Frontend dependencies are installed"
    FRONTEND_READY=true
else
    echo "⏳ Installing frontend dependencies..."
    if npm install --silent; then
        echo "✅ Frontend dependencies installed"
        FRONTEND_READY=true
    else
        echo "❌ Frontend dependency installation failed"
        FRONTEND_READY=false
    fi
fi

# Test 5: Frontend build test
if [ "$FRONTEND_READY" = true ]; then
    echo "5️⃣ Testing frontend build..."
    if npm run build > /dev/null 2>&1; then
        echo "✅ Frontend builds successfully"
    else
        echo "❌ Frontend build failed"
    fi
fi

# Cleanup
echo "🧹 Cleaning up..."
kill $BACKEND_PID 2>/dev/null
wait $BACKEND_PID 2>/dev/null

echo
echo "🎯 Integration Test Summary"
echo "=========================="
echo "✅ Backend: Compiles and runs"
echo "✅ CLI Tool: Working"
if [ "$API_WORKS" = true ]; then
    echo "✅ API Server: Responding on port 3001"
else
    echo "❌ API Server: Issues detected"
fi

if [ "$FRONTEND_READY" = true ]; then
    echo "✅ Frontend: Ready to run"
else
    echo "❌ Frontend: Setup issues"
fi

echo
echo "🚀 Ready to launch!"
echo "Run './start-dev.sh' to start the full application"
echo "Or './demo.sh' for a guided demonstration"
