# 📈 Equity Analyser

A comprehensive equity analysis service that ingests market data, computes technical indicators, and triggers user-defined alerts for US and Canadian equities.

## Features

- **Real-time Market Data**: Pulls quote and historical data from Yahoo Finance
- **Comprehensive Stock Database**: Automatically fetches and maintains a database of all US stocks from NASDAQ and NYSE
- **Technical Indicators**: Computes SMA, EMA, RSI, MACD, Bollinger Bands using the `ta` crate
- **Alert System**: User-defined alert rules with automatic triggering
- **Web UI**: Clean, modern interface for watchlists, alerts, and market signals
- **REST API**: Full RESTful API for all operations
- **Background Processing**: Automated market data updates and alert evaluation
- **Scalable Data Collection**: Fetches market data for all known US stocks automatically

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Market Data**: Yahoo Finance API via `yahoo_finance_api` crate
- **Technical Analysis**: `ta` crate for indicator calculations
- **Authentication**: JWT-based authentication with bcrypt password hashing
- **Frontend**: Vanilla JavaScript with modern CSS

## Quick Start

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- Docker & Docker Compose (optional, for database)

### 1. Database Setup

Using Docker Compose (recommended):
```bash
docker-compose up postgres -d
```

Or install PostgreSQL manually and create the database:
```sql
CREATE DATABASE equity_analyser;
```

### 2. Environment Configuration

Copy the example environment file:
```bash
cp .env.example .env
```

Update the `.env` file with your configuration:
```env
DATABASE_URL=postgres://postgres:password@localhost:5432/equity_analyser
PORT=3000
JWT_SECRET=your-secure-secret-key
MARKET_DATA_INTERVAL_SECONDS=300  # 5 minutes
ALERT_CHECK_INTERVAL_SECONDS=60   # 1 minute
```

### 3. Install Dependencies and Run

```bash
# Install SQLx CLI for database migrations
cargo install sqlx-cli

# Run database migrations
sqlx migrate run

# Build and run the application
cargo run
```

The application will be available at `http://localhost:3000`

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login user
- `GET /api/auth/me` - Get current user info

### Market Data
- `GET /api/market/quote/{symbol}` - Get current quote for a symbol
- `GET /api/market/historical/{symbol}` - Get historical data
- `GET /api/market/indicators/{symbol}` - Get technical indicators

### Watchlist
- `GET /api/watchlist` - Get user's watchlist
- `POST /api/watchlist` - Add symbol to watchlist
- `POST /api/watchlist/{symbol}` - Remove symbol from watchlist

### Alerts
- `GET /api/alerts` - Get user's alerts
- `POST /api/alerts` - Create new alert
- `POST /api/alerts/{id}` - Update alert
- `POST /api/alerts/{id}` - Delete alert

### Stocks
- `GET /api/stocks` - Get list of all known US stocks (paginated)
- `POST /api/stocks/refresh` - Refresh the stock database from external sources

## Database Schema

The application uses the following main tables:

- `users` - User accounts
- `watchlist` - User watchlists
- `stocks` - Complete database of all US stocks with metadata
- `alerts` - User-defined alert rules
- `alert_triggers` - Alert trigger history
- `market_data` - Historical market data cache

## Technical Indicators

The service calculates the following technical indicators:

- **Simple Moving Average (SMA)**: 20-day and 50-day
- **Exponential Moving Average (EMA)**: 12-day and 26-day
- **Relative Strength Index (RSI)**: 14-day
- **MACD**: Moving Average Convergence Divergence with signal line
- **Bollinger Bands**: 20-day with 2 standard deviations

## Alert Types

Supported alert conditions:

- `price_above` - Trigger when price goes above threshold
- `price_below` - Trigger when price goes below threshold
- `rsi_above` - Trigger when RSI goes above threshold
- `rsi_below` - Trigger when RSI goes below threshold

## Development

### Project Structure

```
src/
├── main.rs              # Application entry point
├── api.rs               # Application state
├── config.rs            # Configuration management
├── database.rs          # Database operations
├── models.rs            # Data models and DTOs
├── handlers/            # HTTP request handlers
│   ├── auth.rs          # Authentication handlers
│   ├── market_data.rs   # Market data handlers
│   ├── watchlist.rs     # Watchlist handlers
│   └── alerts.rs        # Alert handlers
├── services/            # Business logic services
│   ├── market_data.rs   # Market data fetching
│   ├── indicators.rs    # Technical indicator calculations
│   ├── scheduler.rs     # Background data updates
│   └── alerts.rs        # Alert evaluation
└── utils/
    └── errors.rs        # Error handling
```

### Adding New Features

1. **New Alert Types**: Add conditions in `services/alerts.rs`
2. **New Indicators**: Extend `services/indicators.rs`
3. **New API Endpoints**: Add handlers and routes in respective modules

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Production Deployment

### Environment Variables

Set the following environment variables for production:

```env
DATABASE_URL=postgres://user:password@host:5432/database
JWT_SECRET=your-production-secret-key
PORT=3000
RUST_LOG=info
```

### Database Migrations

Run migrations in production:
```bash
sqlx migrate run --database-url $DATABASE_URL
```

### Performance Considerations

- The market data scheduler runs every 5 minutes by default
- Alert evaluation runs every minute
- Consider rate limiting for the Yahoo Finance API
- Use connection pooling for database connections (already configured)

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Support

For issues and feature requests, please use the GitHub issues tracker.