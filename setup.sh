#!/bin/bash

echo "🚀 Auto Stock Analyser Setup & Demo"
echo "=================================="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

echo "✅ Cargo found"

# Build the project
echo "🔨 Building project..."
cargo build

if [ $? -eq 0 ]; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# Run tests
echo "🧪 Running tests..."
cargo test

if [ $? -eq 0 ]; then
    echo "✅ All tests passed"
else
    echo "❌ Some tests failed"
    exit 1
fi

echo ""
echo "📊 Available commands:"
echo "  cargo run                           # Run main analysis (AAPL, GOOGL, MSFT, TSLA)"
echo "  cargo run --example simple_analysis # Run simple example (AAPL only)"
echo "  cargo run --example basic_analysis  # Run basic example"
echo "  cargo test                          # Run unit tests"
echo ""
echo "🎯 Ready to analyze stocks! Try running one of the commands above."
