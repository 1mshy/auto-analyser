# Priority-Based Market Data System

This document explains the priority-based market data fetching system that automatically adjusts update frequencies based on stock importance.

## Overview

The system categorizes stocks into three priority levels:
- **High Priority**: Stocks on user watchlists (updated every 1 minute)
- **Medium Priority**: Actively traded stocks (updated every 5 minutes)  
- **Low Priority**: Other stocks (updated every 15 minutes)

## How It Works

### Automatic Priority Assignment

1. **Watchlist Stocks**: When a stock is added to any user's watchlist, it automatically becomes high priority
2. **Active Stocks**: Stocks with recent trading activity are assigned medium priority
3. **Other Stocks**: All remaining stocks default to low priority

### Database Triggers

The system uses PostgreSQL triggers to automatically manage priorities:
- Adding a stock to a watchlist → High priority
- Removing a stock from all watchlists → Medium priority (if not actively traded)

### Smart Scheduling

- Each priority level has its own update interval
- The scheduler checks every 30 seconds for stocks that need updates
- Batch processing with different sizes based on priority
- Rate limiting to avoid API overload

## Configuration

### Environment Variables

```bash
# Enable/disable priority scheduling (default: true)
ENABLE_PRIORITY_SCHEDULING=true

# Traditional fallback intervals (used when priority scheduling is disabled)
MARKET_DATA_INTERVAL_SECONDS=300
```

### Priority Intervals

The update intervals are hardcoded in the `StockPriority` enum:
- High: 60 seconds
- Medium: 300 seconds  
- Low: 900 seconds

## API Endpoints

### Monitor Priority Status

```bash
# Get all stocks grouped by priority
GET /api/stocks/priority

# Get stocks of specific priority
GET /api/stocks/priority?priority=high
GET /api/stocks/priority?priority=medium  
GET /api/stocks/priority?priority=low

# Get stocks that need updates
GET /api/stocks/priority/needing-update
GET /api/stocks/priority/needing-update?priority=high
```

### Example Response

```json
{
  "priorities": {
    "high": {
      "count": 15,
      "update_interval_seconds": 60,
      "symbols": ["AAPL", "GOOGL", "MSFT", ...]
    },
    "medium": {
      "count": 250,
      "update_interval_seconds": 300,
      "symbols": ["TSLA", "NVDA", ...]
    },
    "low": {
      "count": 4500,
      "update_interval_seconds": 900,
      "symbols": ["XYZ", "ABC", ...]
    }
  }
}
```

## Benefits

1. **Efficient Resource Usage**: High-priority stocks get frequent updates while low-priority stocks don't waste API calls
2. **Better User Experience**: Watchlist stocks always have fresh data
3. **Scalable**: System automatically adapts as users add/remove stocks from watchlists
4. **Cost Effective**: Reduces API usage for stocks that aren't actively monitored

## Implementation Details

### Database Schema

```sql
-- New columns added to stocks table
ALTER TABLE stocks 
ADD COLUMN priority stock_priority DEFAULT 'low',
ADD COLUMN last_price_update TIMESTAMP WITH TIME ZONE,
ADD COLUMN price_update_interval_seconds INTEGER DEFAULT 900;
```

### Key Components

1. **PriorityScheduler**: Main scheduling service that manages updates
2. **Database Triggers**: Automatically adjust priorities based on watchlist changes
3. **Watchlist Handlers**: Trigger immediate updates when stocks are added to watchlists
4. **Priority API Endpoints**: Monitor and debug the priority system

## Migration

To enable the priority system on an existing installation:

1. Run the migration script:
   ```bash
   ./setup_priority_system.sh
   ```

2. Restart the application with priority scheduling enabled:
   ```bash
   ENABLE_PRIORITY_SCHEDULING=true ./start-app.sh
   ```

3. Verify the system is working:
   ```bash
   curl http://localhost:3000/api/stocks/priority
   ```

## Troubleshooting

### Common Issues

1. **No High Priority Stocks**: Add some stocks to watchlists to see high priority in action
2. **Slow Updates**: Check if `ENABLE_PRIORITY_SCHEDULING=true` is set
3. **Database Errors**: Ensure the migration completed successfully

### Logging

The system logs priority update activity:
```
INFO: Starting priority-based market data scheduler
INFO: Updating 15 high priority stocks
INFO: High priority update completed: 15 successful, 0 failed
```

### Fallback Mode

If priority scheduling is disabled, the system falls back to the traditional uniform update interval for all stocks.
