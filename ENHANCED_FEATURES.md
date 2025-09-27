# 📋 API Documentation

## New Enhanced Endpoints

### Health & Status
- `GET /api/health` - Application health check
- `GET /api/continuous-status` - Real-time continuous analysis status  
- `POST /api/filtered-results` - Get filtered stock analysis results

### System Monitoring
- `GET /api/cache-stats` - Cache performance metrics
- `GET /api/database-stats` - Database analytics and statistics
- `POST /api/clear-cache` - Clear application cache

### Analysis Operations
- `POST /api/analysis` - Start new analysis session
- `GET /api/analysis/:id` - Get analysis session status
- `GET /api/analysis/:id/results` - Get analysis results
- `POST /api/filter-stats` - Get filter statistics

### WebSocket
- `WS /ws` - Real-time updates for continuous analysis

## Enhanced Features

### 🚀 Performance Improvements
- **Multi-layer Caching**: Stock data, indicators, and API responses cached
- **Database Persistence**: SQLite storage with automatic schema migrations
- **Rate Limiting**: Intelligent API throttling to prevent rate limit violations
- **Connection Pooling**: Optimized database connections

### 📊 Advanced Analytics
- **RSI Distribution Charts**: Visual representation of market conditions  
- **Opportunity Detection**: Automated identification of trading signals
- **Historical Trend Analysis**: Long-term stock performance tracking
- **Smart Filtering**: Multi-dimensional stock filtering capabilities

### 🛡️ Production Features
- **Structured Logging**: Configurable log levels with tracing
- **Error Boundaries**: Graceful error handling and recovery
- **Health Monitoring**: Comprehensive system status monitoring
- **Docker Support**: Full containerization for easy deployment

## Testing Suite

### Run Tests
```bash
# Run all tests
cargo test

# Run specific test suites
cargo test analyzer_tests
cargo test database_tests

# Run with output
cargo test -- --nocapture
```

### Test Coverage
- ✅ Unit tests for all core components
- ✅ Integration tests for API endpoints  
- ✅ Database operations testing
- ✅ Cache functionality verification
- ✅ Error handling scenarios

## Deployment Options

### 🐳 Docker (Production)
```bash
# Quick start
docker-compose up -d

# View logs
docker-compose logs -f

# Scale services
docker-compose up -d --scale auto-analyser=3
```

### 🛠️ Local Development
```bash
# Enhanced deployment script
./deploy.sh

# Manual setup
cargo run --bin server
```

### ☁️ Cloud Deployment
- Ready for AWS ECS/EKS, Google Cloud Run, Azure Container Instances
- Environment variables for configuration
- Health checks for load balancers
- Horizontal scaling support

## Configuration

### Environment Variables
- `RUST_LOG`: Log level (error, warn, info, debug, trace)
- `DATABASE_URL`: SQLite database location
- `PORT`: Server port (default: 3001)
- `CACHE_SIZE`: Maximum cache entries per type

### Feature Flags
- Caching can be disabled for development
- Database persistence is optional
- Rate limiting thresholds configurable

## Monitoring & Observability

### Metrics Available
- Cache hit/miss ratios
- Database query performance
- API request rates
- Analysis processing times
- Memory usage statistics

### Logging
- Structured JSON logging
- Request/response tracing
- Error stack traces
- Performance metrics