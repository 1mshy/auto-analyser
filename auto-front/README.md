# Equity Analyser Frontend

A modern React application built with Vite, HeroUI, and Tailwind CSS that provides a user-friendly interface for the Equity Analyser stock analysis platform.

## Features

- **Authentication**: Secure user registration and login
- **Real-time Market Data**: Get current quotes and technical indicators for any stock
- **Watchlist Management**: Track your favorite stocks with real-time updates
- **Smart Alerts**: Set price and technical indicator-based alerts
- **Responsive Design**: Works seamlessly on desktop and mobile devices
- **Modern UI**: Clean, professional interface using HeroUI components

## Tech Stack

- **React 18** - Modern React with hooks and functional components
- **TypeScript** - Type-safe development
- **Vite** - Fast build tool and development server
- **HeroUI** - Beautiful, accessible UI components
- **Tailwind CSS** - Utility-first CSS framework
- **React Router** - Client-side routing

## Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn
- Running Equity Analyser backend (Rust service on port 3000)

### Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev
```

The app will be available at `http://localhost:5173` (or another port if 5173 is busy).

## Project Structure

```
src/
├── components/           # Reusable UI components
│   ├── auth/            # Authentication components
│   ├── AlertsManager.tsx # Alert management interface
│   ├── Dashboard.tsx     # Main dashboard layout
│   ├── StockQuote.tsx   # Stock quote display
│   └── Watchlist.tsx    # Watchlist management
├── contexts/            # React contexts
│   └── AuthContext.tsx  # Authentication state management
├── services/            # API services
│   └── api.ts           # Backend API integration
├── App.tsx              # Main app component
└── main.tsx             # App entry point
```

## Key Features

### Authentication Flow

- User registration and login with JWT tokens
- Automatic token persistence and validation
- Protected routes based on authentication state

### Market Data

- Real-time stock quotes with current price and change percentage
- Technical indicators including SMA, EMA, RSI, MACD, and Bollinger Bands
- Search functionality for any stock symbol

### Watchlist Management

- Add/remove stocks from personal watchlist
- Real-time price updates for all watched symbols
- Quick overview of portfolio performance

### Alert System

- Price-based alerts (above/below thresholds)
- Technical indicator alerts (RSI overbought/oversold)
- Enable/disable alerts with toggle switches
- Alert history and management

## API Integration

Connects to the Rust backend API at `http://localhost:3000` with endpoints for:

- Authentication (`/api/auth/*`)
- Market data (`/api/market/*`)
- Watchlist management (`/api/watchlist`)
- Alert management (`/api/alerts`)

## Development

```bash
# Run linting
npm run lint

# Build for production
npm run build

# Preview production build
npm run preview
```
