# Auto Stock Analyser

A comprehensive stock market analysis tool with a Rust backend for high-performance data processing and a modern React frontend for real-time visualization. Fetches data from Yahoo Finance API and calculates technical indicators with advanced customizable filtering.

## 🚀 New React Frontend Dashboard

**Experience real-time stock analysis with our beautiful web interface!**

- 📊 **Live Dashboard**: Watch analysis progress in real-time
- 🎯 **Interactive Filters**: Customize analysis criteria with instant feedback
- 📈 **Visual Charts**: RSI distribution and market trend visualization  
- 🔍 **Smart Filtering**: Sort and filter results by opportunities, RSI, volume, and more
- 📱 **Responsive Design**: Works perfectly on desktop, tablet, and mobile

### Quick Start with Web Interface

```bash
# Start both backend API and React frontend
./start-dev.sh
```

Then open your browser to:
- **Dashboard**: http://localhost:3000
- **API**: http://127.0.0.1:3001

## Installation & Setup

### Prerequisites

- **Rust** (latest stable version): [Install Rust](https://rustup.rs/)
- **Node.js** 16+ and npm: [Install Node.js](https://nodejs.org/)
- **Git**: For cloning the repository

### Quick Setup

```bash
# Clone the repository
git clone <your-repo-url>
cd auto-analyser

# Start the full application (backend + frontend)
./start-dev.sh
```

The startup script will:
1. Build and start the Rust backend API on port 3001
2. Install frontend dependencies (if needed)
3. Start the React development server on port 3000
4. Open the dashboard at http://localhost:3000

### Manual Setup

If you prefer to run components separately:

#### Backend Only
```bash
# Run the command-line analyzer
cargo run

# Or start the web API server
cargo run --bin server
```

#### Frontend Only
```bash
cd frontend
npm install
npm start
```

## Usage Modes

### 🌐 Web Dashboard (Recommended)

The React frontend provides the most intuitive experience:

1. **Configure Analysis**: Use the filter panel to set your criteria
2. **Start Analysis**: Click "Start Analysis" to begin real-time processing
3. **Monitor Progress**: Watch live updates as stocks are analyzed
4. **View Results**: Explore opportunities with interactive tables and charts

Features:
- Real-time progress tracking
- Interactive filtering and sorting
- RSI distribution charts
- Mobile-responsive design
- Trading signal alerts

### 💻 Command Line Interface

For traditional terminal-based usage:

- **Basic Analysis**: Just run `cargo run` to fetch and analyze today's top stocks.
- **Custom Scripts**: Use provided examples or create your own in the `src/bin` directory.
- **Ticker Collection**: Explore thousands of tickers with `cargo run --example ticker_collection`.

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
- 🏢 **Advanced Filtering**: Customizable filters for:
  - Market capitalization ranges
  - Price ranges
  - Trading volume
  - Percentage change (daily performance)
  - RSI-based conditions (oversold/overbought)
  - Sector and industry focus
  - Geographic filters
  - IPO year ranges
- 🏆 **Opportunity Detection**: Smart identification of investment opportunities based on your criteria
- ⚡ **Performance Optimized**: Rate-limited analysis to prevent API throttling

## Quick Start

### Basic Usage
```bash
# Clone and run with default settings
git clone <repository-url>
cd auto-analyser
cargo run
```

### Customizing Your Analysis
Edit the `create_custom_filter()` function in `src/main.rs` to set your criteria:
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

## Customizing Your Analysis
Edit the `create_custom_filter()` function in `src/main.rs` to set your criteria:

```rust
fn create_custom_filter() -> StockFilter {
    StockFilter::new()
        // Market cap range: $1B to $50B
        .with_market_cap_range(Some(1_000_000_000.0), Some(50_000_000_000.0))
        
        // Price range: $10 to $300
        .with_price_range(Some(10.0), Some(300.0))
        
        // Look for recent gainers (2% to 15% daily change)
        .with_pct_change_range(Some(2.0), Some(15.0))
        
        // Find oversold opportunities (RSI < 30)
        .with_rsi_thresholds(Some(30.0), Some(70.0))
}
```

### Example Configurations
See `cargo run --example filter_examples` for pre-built filter configurations:

- **Large Cap Value**: Focus on established companies showing recent declines
- **Growth Momentum**: High-growth potential stocks with positive trends  
- **Conservative Large**: Stable mega-cap stocks with low volatility
- **Oversold Recovery**: Undervalued stocks with recovery potential

```bash
# Run main analysis with your custom filters
cargo run

# Explore different filter configurations  
cargo run --example filter_examples

# See all available tickers and data structure
cargo run --example debug_tickers

# Test custom filtering configurations
cargo run --example custom_filters
```

## Filter Criteria Available

- **Market Capitalization**: Set min/max market cap ranges
- **Stock Price**: Filter by current stock price
- **Trading Volume**: Minimum volume requirements
- **Daily Performance**: Percentage change filters
- **RSI Conditions**: Oversold/overbought detection
- **Sector Focus**: Target specific business sectors
- **Geographic**: Filter by company location
- **Company Age**: IPO year-based filtering

## Investment Strategies Supported

1. **Value Investing**: Find undervalued stocks with strong fundamentals
2. **Growth Investing**: Identify stocks with strong momentum and growth potential
3. **Momentum Trading**: Spot stocks with recent positive performance trends
4. **Contrarian Investing**: Find oversold stocks ready for recovery
5. **Large Cap Safety**: Focus on established, stable companies

## Performance Notes

- Analysis is rate-limited to prevent API throttling
- Processes up to 100 stocks per run by default
- Real-time RSI calculation for opportunity detection
- Intelligent prioritization of top-performing stocks

## Contributing

Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest new features.

## Disclaimer

This tool is for educational and research purposes only. Not financial advice. Always do your own research before making investment decisions.