#!/bin/bash

# Auto Stock Analyser - Enhanced Deployment Script
# This script sets up and runs the enhanced auto-analyser with all new features

set -e

echo "🚀 Auto Stock Analyser - Enhanced Setup"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Docker is available
if command -v docker &> /dev/null && command -v docker-compose &> /dev/null; then
    echo -e "${BLUE}🐳 Docker detected. Offering containerized deployment...${NC}"
    read -p "Use Docker deployment? (y/n): " use_docker
    
    if [[ $use_docker =~ ^[Yy]$ ]]; then
        echo -e "${GREEN}Starting containerized deployment...${NC}"
        
        # Create data directory
        mkdir -p data
        
        # Build and start containers
        docker-compose down --remove-orphans
        docker-compose build
        docker-compose up -d
        
        echo -e "${GREEN}✅ Application started successfully!${NC}"
        echo -e "${BLUE}📊 Dashboard: http://localhost${NC}"
        echo -e "${BLUE}🔌 API: http://localhost:3001${NC}"
        echo -e "${BLUE}📋 Health: http://localhost:3001/api/health${NC}"
        
        echo -e "${YELLOW}📝 To view logs: docker-compose logs -f${NC}"
        echo -e "${YELLOW}🛑 To stop: docker-compose down${NC}"
        
        exit 0
    fi
fi

# Local development setup
echo -e "${YELLOW}Setting up local development environment...${NC}"

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust not found. Please install Rust from https://rustup.rs/${NC}"
    exit 1
fi

# Check Node.js installation
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Please install Node.js from https://nodejs.org/${NC}"
    exit 1
fi

# Create data directory for SQLite database
echo -e "${BLUE}📁 Creating data directory...${NC}"
mkdir -p data

# Install Rust dependencies and build
echo -e "${BLUE}🦀 Building Rust backend...${NC}"
cargo build --release --bin server

# Install frontend dependencies
echo -e "${BLUE}⚛️ Installing frontend dependencies...${NC}"
cd frontend
npm install

# Build frontend
echo -e "${BLUE}🏗️ Building frontend...${NC}"
npm run build
cd ..

# Function to start backend
start_backend() {
    echo -e "${GREEN}🚀 Starting Rust backend server...${NC}"
    RUST_LOG=info DATABASE_URL=sqlite:./data/analysis.db ./target/release/server
}

# Function to start frontend in development mode
start_frontend_dev() {
    echo -e "${GREEN}⚛️ Starting React frontend in development mode...${NC}"
    cd frontend
    npm start
}

# Ask user for deployment mode
echo -e "${YELLOW}Choose deployment mode:${NC}"
echo "1. Production (built frontend served by Rust backend)"
echo "2. Development (separate frontend and backend servers)"
read -p "Enter choice (1 or 2): " mode

case $mode in
    1)
        echo -e "${GREEN}🏭 Starting in production mode...${NC}"
        echo -e "${BLUE}📊 Dashboard will be available at: http://localhost:3001${NC}"
        echo -e "${BLUE}🔌 API available at: http://localhost:3001/api${NC}"
        echo -e "${BLUE}📋 Health check: http://localhost:3001/api/health${NC}"
        echo -e "${BLUE}📊 System stats: http://localhost:3001/api/cache-stats${NC}"
        echo ""
        echo -e "${YELLOW}Press Ctrl+C to stop the server${NC}"
        start_backend
        ;;
    2)
        echo -e "${GREEN}🛠️ Starting in development mode...${NC}"
        echo -e "${BLUE}📊 Frontend will be available at: http://localhost:3000${NC}"
        echo -e "${BLUE}🔌 Backend will be available at: http://localhost:3001${NC}"
        echo ""
        echo -e "${YELLOW}Starting backend server in background...${NC}"
        
        # Start backend in background
        RUST_LOG=info DATABASE_URL=sqlite:./data/analysis.db ./target/release/server &
        BACKEND_PID=$!
        
        # Wait for backend to start
        sleep 3
        
        echo -e "${YELLOW}Starting frontend development server...${NC}"
        echo -e "${YELLOW}Press Ctrl+C to stop both servers${NC}"
        
        # Function to cleanup on exit
        cleanup() {
            echo -e "\n${YELLOW}🛑 Stopping servers...${NC}"
            kill $BACKEND_PID 2>/dev/null || true
            exit 0
        }
        
        # Set trap for cleanup
        trap cleanup SIGINT SIGTERM
        
        # Start frontend (this will block)
        start_frontend_dev
        ;;
    *)
        echo -e "${RED}❌ Invalid choice. Exiting.${NC}"
        exit 1
        ;;
esac