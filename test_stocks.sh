#!/bin/bash

# Test script to verify the stock fetching functionality
echo "Testing stock fetching functionality..."

# Start the application in the background
echo "Starting the application..."
cd /Users/lucalapenna/Documents/GitHub/auto-analyser
cargo run &
APP_PID=$!

# Wait for the app to start
echo "Waiting for application to start..."
sleep 15

# Test the health endpoint
echo "Testing health endpoint..."
curl -s http://localhost:3000/health

# Test the stock list endpoint (you may need to authenticate first)
echo -e "\n\nTesting stock list endpoint..."
curl -s "http://localhost:3000/api/stocks?limit=10" | jq '.' || echo "Need authentication or jq not installed"

# Test the stock refresh endpoint
echo -e "\n\nTesting stock refresh endpoint..."
curl -s -X POST "http://localhost:3000/api/stocks/refresh?force=true" | jq '.' || echo "Need authentication or jq not installed"

# Stop the application
echo -e "\n\nStopping application..."
kill $APP_PID 2>/dev/null
wait $APP_PID 2>/dev/null

echo "Test completed!"
