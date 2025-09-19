#!/bin/bash

# Auto Stock Analyser - Demo Script
# Demonstrates the capabilities of both command-line and web interfaces

echo "🚀 Auto Stock Analyser Demo"
echo "==========================="
echo
echo "This demo will show you:"
echo "1. 💻 Command-line analysis"
echo "2. 🌐 Web dashboard setup"
echo "3. 📊 Real-time monitoring"
echo

read -p "Press Enter to start the demo..."

echo
echo "📋 Step 1: Command-Line Analysis"
echo "--------------------------------"
echo "Running a quick analysis with the CLI tool..."
echo

# Run a basic analysis
timeout 30 cargo run --example simple_analysis

echo
echo "✅ CLI analysis complete!"
echo
echo "📊 Step 2: Web Dashboard"
echo "------------------------"
echo "Now let's start the web interface for real-time analysis..."
echo

read -p "Ready to start the web dashboard? (y/n): " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🌐 Starting web dashboard..."
    echo
    echo "The dashboard will open with:"
    echo "- Real-time analysis progress"
    echo "- Interactive filters"
    echo "- Live charts and graphs"
    echo "- Mobile-responsive design"
    echo
    echo "Frontend: http://localhost:3000"
    echo "Backend API: http://127.0.0.1:3001"
    echo
    echo "Press Ctrl+C to stop all services when done"
    echo
    
    # Start the full stack
    ./start-dev.sh
else
    echo "Demo completed! To start the web dashboard later, run:"
    echo "./start-dev.sh"
fi

echo
echo "🎯 Demo Features Showcased:"
echo "- High-performance Rust backend"
echo "- Modern React frontend"
echo "- Real-time data streaming"
echo "- Technical indicator calculations"
echo "- Advanced filtering capabilities"
echo "- Trading signal detection"
echo
echo "Happy analyzing! 📈"
