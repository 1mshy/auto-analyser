# Auto Stock Analyser

A Rust-based stock market analysis tool that fetches real-time and historical data using Yahoo Finance API and calculates technical indicators.

## Features

- 📈 **Real-time Stock Data**: Fetch current quotes for any stock symbol
- 📊 **Historical Data**: Retrieve historical stock data for analysis
- 🔍 **Technical Indicators**: Calculate popular technical indicators including:
  - Simple Moving Averages (SMA 20, SMA 50)
  - Relative Strength Index (RSI)
  - Moving Average Convergence Divergence (MACD)
- 🚨 **Trading Signals**: Basic signal detection for overbought/oversold conditions and trend analysis
- 📋 **Multi-Symbol Analysis**: Analyze multiple stocks in a single run

## Dependencies

- `yahoo_finance_api`: For fetching stock data from Yahoo Finance
- `ta`: Technical analysis indicators library
- `tokio`: Async runtime for handling API requests
- `chrono` & `time`: Date and time handling
- `anyhow`: Error handling

## Usage

### Basic Example

```bash
cargo run
```

This will analyze the default stocks (AAPL, GOOGL, MSFT, TSLA) with the last 100 days of data.

### Customizing Analysis

You can modify the `main()` function to:

1. **Change symbols**:
```rust
let symbols = vec!["NVDA", "AMD", "INTC"];
```

2. **Adjust time range**:
```rust
let end = Utc::now();
let start = end - Duration::days(365); // 1 year of data
```

3. **Add custom indicators**:
```rust
// In the IndicatorSet struct, add new indicators
struct IndicatorSet {
    sma_20: SimpleMovingAverage,
    sma_50: SimpleMovingAverage,
    sma_200: SimpleMovingAverage, // Add 200-day SMA
    rsi: RelativeStrengthIndex,
    macd: MovingAverageConvergenceDivergence,
}
```

## Code Structure

### Core Components

1. **`StockData`**: Represents a single stock data point with OHLCV data
2. **`TechnicalIndicators`**: Container for calculated technical indicators
3. **`StockAnalyzer`**: Main analysis engine that:
   - Fetches data from Yahoo Finance
   - Calculates technical indicators
   - Generates trading signals
   - Formats analysis output

### Key Methods

- `fetch_stock_data()`: Retrieves historical stock data
- `get_latest_quote()`: Gets the most recent stock quote
- `calculate_indicators()`: Computes technical indicators for stock data
- `analyze_signals()`: Generates trading signals based on indicators
- `print_analysis()`: Displays formatted analysis results

## Sample Output

```
==================================================

=== Stock Analysis for AAPL ===
Latest Data (2025-09-17):
  Price: $175.23
  Volume: 45234567

Technical Indicators:
  SMA(20): $172.45
  SMA(50): $168.90
  RSI(14): 65.23
  MACD: 0.1234, Signal: 0.0987, Histogram: 0.0247

Signals:
  • Bullish: Price above SMA20 > SMA50
  • MACD Bullish: MACD above Signal

Latest Quote Update:
  Time: 2025-09-17 20:00:00 UTC
  Price: $175.23
```

## Extending the Analysis

### Adding New Indicators

1. Import the indicator from the `ta` crate
2. Add it to the `IndicatorSet` struct
3. Initialize it in `initialize_indicators()`
4. Calculate it in `calculate_indicators()`
5. Add it to `TechnicalIndicators` struct
6. Update signal analysis in `analyze_signals()`

### Adding Custom Signals

Modify the `analyze_signals()` method to include your custom trading logic:

```rust
// Example: Bollinger Bands squeeze detection
if let (Some(bb_upper), Some(bb_lower)) = (indicators.bb_upper, indicators.bb_lower) {
    let bb_width = (bb_upper - bb_lower) / data.close;
    if bb_width < 0.05 { // 5% width threshold
        signals.push("Bollinger Bands Squeeze Detected".to_string());
    }
}
```

## Error Handling

The application includes comprehensive error handling for:
- Network connectivity issues
- Invalid stock symbols
- Missing data points
- API rate limiting

## Rate Limiting

Yahoo Finance API has rate limits. For production use, consider:
- Adding delays between requests
- Implementing retry logic
- Caching frequently requested data
- Using alternative data sources for high-frequency analysis

## Contributing

Feel free to contribute by:
- Adding new technical indicators
- Improving signal detection algorithms
- Adding data visualization
- Implementing backtesting capabilities
- Adding support for other data sources

## License

This project is open source and available under the MIT License.