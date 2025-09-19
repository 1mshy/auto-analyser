# Stock Filtering System

The Auto Stock Analyser now includes a comprehensive filtering system that allows you to customize which stocks are analyzed based on various criteria.

## How to Use

### Basic Usage

Modify the `create_custom_filter()` function in `src/main.rs` to set your desired criteria:

```rust
fn create_custom_filter() -> StockFilter {
    StockFilter::new()
        .with_market_cap_range(Some(100_000_000.0), Some(50_000_000_000.0))  // $100M to $50B
        .with_price_range(Some(5.0), Some(200.0))                            // $5 to $200
        .with_volume_range(Some(100_000), None)                              // Min 100K volume
        // ... add more filters as needed
}
```

## Available Filters

### Market Capitalization
```rust
.with_market_cap_range(Some(min), Some(max))
```
- Filter stocks by market cap range
- Values in USD (e.g., `1_000_000_000.0` for $1B)
- Use `None` for no limit

### Stock Price
```rust
.with_price_range(Some(min), Some(max))
```
- Filter by current stock price
- Values in USD (e.g., `10.0` for $10)

### Trading Volume
```rust
.with_volume_range(Some(min), Some(max))
```
- Filter by daily trading volume
- Values as number of shares

### Percentage Change
```rust
.with_pct_change_range(Some(min), Some(max))
```
- Filter by daily percentage change
- Values as percentages (e.g., `5.0` for +5%, `-10.0` for -10%)

### RSI (Relative Strength Index)
```rust
.with_rsi_range(Some(min), Some(max))
.with_rsi_thresholds(Some(oversold), Some(overbought))
```
- Filter by RSI values (0-100)
- `with_rsi_thresholds` specifically looks for oversold/overbought conditions

### Sectors
```rust
.with_sectors(vec![
    "Technology".to_string(),
    "Healthcare".to_string(),
    "Financial".to_string(),
])
```
- Filter by business sectors
- Common sectors: Technology, Healthcare, Financial, Consumer, Energy, Industrial, etc.

### Countries
```rust
.with_countries(vec!["United States".to_string(), "Canada".to_string()])
```
- Filter by company headquarters country

### Industries
```rust
.with_industries(vec!["Software".to_string(), "Biotechnology".to_string()])
```
- Filter by specific industries within sectors

### IPO Year
```rust
.with_ipo_year_range(Some(2010), Some(2023))
```
- Filter by when the company went public
- Useful for finding established vs. newer companies

## Example Configurations

### 1. Conservative Large Cap
```rust
StockFilter::new()
    .with_market_cap_range(Some(10_000_000_000.0), None)     // $10B+
    .with_price_range(Some(50.0), Some(300.0))               // $50-$300
    .with_volume_range(Some(1_000_000), None)                // 1M+ volume
    .with_pct_change_range(Some(-5.0), Some(5.0))            // Max 5% change
    .with_countries(vec!["United States".to_string()])
```

### 2. Growth Stocks
```rust
StockFilter::new()
    .with_market_cap_range(Some(500_000_000.0), Some(20_000_000_000.0))  // $500M-$20B
    .with_pct_change_range(Some(2.0), Some(20.0))                        // 2-20% gains
    .with_ipo_year_range(Some(2015), None)                               // Recent IPOs
    .with_sectors(vec!["Technology".to_string(), "Healthcare".to_string()])
```

### 3. Value/Oversold Stocks
```rust
StockFilter::new()
    .with_market_cap_range(Some(1_000_000_000.0), None)      // $1B+
    .with_pct_change_range(Some(-15.0), Some(-2.0))          // Recent decline
    .with_rsi_thresholds(Some(25.0), Some(35.0))             // Oversold
```

### 4. Momentum Stocks
```rust
StockFilter::new()
    .with_pct_change_range(Some(5.0), Some(25.0))            // Strong gains
    .with_rsi_range(Some(45.0), Some(65.0))                  // Not extreme RSI
    .with_volume_range(Some(1_000_000), None)                // High volume
```

## Tips

1. **Start Broad**: Begin with loose criteria and gradually narrow down
2. **Market Conditions**: Adjust filters based on current market conditions
3. **Backtesting**: Test your filters on historical data when possible
4. **Performance**: More restrictive filters = faster analysis but fewer opportunities
5. **Combine Filters**: Use multiple criteria for more sophisticated screening

## Running Examples

```bash
# Run the main application with your custom filters
cargo run

# See example filter configurations
cargo run --example custom_filters

# Test different ticker collections
cargo run --example ticker_collection
```

## Common Market Cap Ranges

- **Nano Cap**: Under $50M
- **Micro Cap**: $50M - $300M  
- **Small Cap**: $300M - $2B
- **Mid Cap**: $2B - $10B
- **Large Cap**: $10B - $200B
- **Mega Cap**: Over $200B

Use these as guidelines for setting your market cap filters.
