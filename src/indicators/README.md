# Technical Indicators Module

This module contains custom implementations and wrappers for technical analysis indicators used in the auto-analyser.

## Structure

```
indicators/
├── mod.rs          # Module exports
├── rsi.rs          # Relative Strength Index (Custom TradingView-compatible implementation)
├── sma.rs          # Simple Moving Average (Wrapper around ta crate)
├── macd.rs         # MACD (Wrapper around ta crate)
└── README.md       # This file
```

## Indicators

### RSI (Relative Strength Index)
- **File**: `rsi.rs`
- **Implementation**: Custom implementation that matches TradingView's calculation
- **Method**: Uses Wilder's smoothing (exponential moving average with α = 1/period)
- **Usage**: Default 14-period RSI
- **Range**: 0-100 (overbought >70, oversold <30)

### SMA (Simple Moving Average)
- **File**: `sma.rs`
- **Implementation**: Wrapper around the `ta` crate's SimpleMovingAverage
- **Purpose**: Provides a consistent interface and room for future customization
- **Usage**: Commonly used with 20 and 50 period windows

### MACD (Moving Average Convergence Divergence)
- **File**: `macd.rs`
- **Implementation**: Wrapper around the `ta` crate's MACD
- **Output**: Custom `MACDOutput` struct with `macd`, `signal`, and `histogram` fields
- **Default Parameters**: 12-period fast EMA, 26-period slow EMA, 9-period signal line

## Design Philosophy

1. **Modularity**: Each indicator is in its own file for better organization
2. **Consistency**: All indicators follow similar patterns for instantiation and usage
3. **Extensibility**: Wrappers allow for future customization without changing the main analyzer code
4. **Accuracy**: Custom implementations (like RSI) ensure compatibility with popular trading platforms

## Adding New Indicators

To add a new indicator:

1. Create a new `.rs` file in the `indicators/` directory
2. Implement the indicator with `new()`, `next()`, and `reset()` methods
3. Add appropriate tests
4. Export the indicator in `mod.rs`
5. Update this README

## Testing

Each indicator includes unit tests to verify:
- Proper instantiation
- Basic calculation functionality
- Reset functionality
- Edge cases where applicable

Run tests with:
```bash
cargo test indicators
```
