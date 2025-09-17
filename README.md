# Auto Stock Analyser

A Rust-based stock market analysis tool that fetches real-time and historical data using Yahoo Finance API and calculates technical indicators.

## Features

- 📈 **Real-time Stock Data**: Fetch current quotes for any stock symbol
- 📊 **Historical Data**: Retrieve historical stock data for analysis
- 🎯 **Ticker Collection**: Fetch all available tickers from Nasdaq API with filtering capabilities
- 🔍 **Technical Indicators**: Calculate popular technical indicators including:
  - Simple Moving Averages (SMA 20, SMA 50)
  - Relative Strength Index (RSI)
  - Moving Average Convergence Divergence (MACD)
- 🚨 **Trading Signals**: Basic signal detection for overbought/oversold conditions and trend analysis
- 📋 **Multi-Symbol Analysis**: Analyze multiple stocks in a single run
- 🏢 **Smart Filtering**: Filter stocks by sector, market cap, country, and performance
- 🏆 **Top Performers**: Automatically identify best performing stocks

## Dependencies

- `yahoo_finance_api`: For fetching stock data from Yahoo Finance
- `ta`: Technical analysis indicators library
- `tokio`: Async runtime for handling API requests
- `chrono` & `time`: Date and time handling
- `anyhow`: Error handling

## Quick Start

```bash
# Clone and setup
git clone <your-repo-url>
cd auto-analyser
./setup.sh

# Run complete analysis pipeline
cargo run --example complete_analysis

# Explore 7000+ available tickers
cargo run --example ticker_collection

# Simple single-stock analysis
cargo run --example simple_analysis
```

## Usage

### Basic Example

```bash
cargo run
```

This will fetch today's top performers from Nasdaq and analyze them along with some stable large-cap stocks.

### Ticker Collection

```bash
cargo run --example ticker_collection
```

This will:
- Fetch all available tickers from Nasdaq API
- Show top performers by percentage change
- Filter by sector (Technology, Healthcare, etc.)
- Filter by market cap
- Display sector distribution

### Single Stock Analysis

```bash
cargo run --example simple_analysis
```

This will analyze Apple (AAPL) with 30 days of data and show trend analysis.

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

### New Ticker Collection Features

1. **Fetch All Tickers**:
```rust
let tickers = StockAnalyzer::fetch_all_tickers().await?;
```

2. **Filter by Criteria**:
```rust
let tech_stocks = StockAnalyzer::filter_tickers(
    &tickers,
    Some("Technology"),  // Sector filter
    Some(1_000_000_000.0), // Min market cap ($1B)
    Some("United States"), // Country filter
);
```

3. **Get Top Performers**:
```rust
let top_10 = StockAnalyzer::get_top_performers(&tickers, 10);
```

4. **Display Formatted Results**:
```rust
StockAnalyzer::print_tickers(&top_10, "Top Performers");
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