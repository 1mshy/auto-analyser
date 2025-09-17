# Stock Symbol Filtering and Delisting Detection

This document describes the implementation of stock symbol filtering and automatic delisting detection features.

## Features Implemented

### 1. Symbol Filtering
- **Automatic filtering of symbols containing "^"**: These are typically indices or special financial instruments rather than individual stocks
- **Empty symbol filtering**: Prevents processing of empty or whitespace-only symbols
- **Centralized filtering logic**: The `StockListService::should_ignore_symbol()` method handles all filtering rules

### 2. Delisting Detection
- **Automatic detection of delisted stocks**: When fetching market data fails with specific error messages indicating delisting
- **Database tracking**: Delisted stocks are marked as inactive with reason and error details
- **Error patterns detected**:
  - "No data found"
  - "Symbol may be delisted"
  - "Invalid symbol"
  - "Not found"

### 3. Database Schema Updates
New fields added to the `stocks` table:
- `delisting_reason`: Categorizes why a stock was marked as delisted
- `last_error_at`: Timestamp of the last error encountered
- `last_error_message`: Details of the last error message

## Implementation Details

### Files Modified

1. **`src/services/stock_list.rs`**
   - Added `should_ignore_symbol()` function to filter symbols with "^"
   - Added `mark_stock_as_delisted()` and `mark_stock_as_delisted_with_reason()` methods
   - Updated stock fetching to apply filtering
   - Added tests for filtering logic

2. **`src/services/market_data.rs`**
   - Added `is_delisting_error()` function to detect delisting-related errors
   - Updated `fetch_current_quote()` and `fetch_historical_data()` to filter symbols with "^"
   - Added `*_with_delisting_check()` variants that automatically mark delisted stocks
   - Added tests for error detection

3. **`src/services/scheduler.rs`**
   - Updated market data scheduler to use delisting-aware methods
   - Added symbol filtering in the update process

4. **`src/handlers/market_data.rs`**
   - Updated to use delisting-aware market data methods

5. **`src/database.rs`**
   - Added `pool()` method to access the database pool
   - Updated `create_stock()` method to handle new delisting fields

6. **`src/models.rs`**
   - Updated `Stock` model to include new delisting tracking fields

7. **`migrations/003_add_delisting_tracking.sql`**
   - New migration to add delisting tracking fields to the database

## Usage Examples

### Checking if a symbol should be ignored
```rust
if StockListService::should_ignore_symbol("^VIX") {
    // This symbol will be ignored (contains "^")
}
```

### Fetching quotes with automatic delisting detection
```rust
let market_service = MarketDataService::new();
match market_service.fetch_current_quote_with_delisting_check("AAPL", &db_pool).await {
    Ok(quote) => {
        // Successfully fetched quote
    }
    Err(AppError::BadRequest(msg)) if msg.contains("delisted") => {
        // Stock was automatically marked as delisted
    }
    Err(e) => {
        // Other error
    }
}
```

## Benefits

1. **Reduced API calls**: Filtering out symbols with "^" prevents unnecessary API calls for indices
2. **Automatic cleanup**: Delisted stocks are automatically detected and marked as inactive
3. **Better data quality**: The system maintains a clean list of tradeable stocks
4. **Error resilience**: The system gracefully handles delisted stocks without breaking
5. **Audit trail**: Full tracking of when and why stocks were marked as delisted

## Testing

Tests have been added to verify:
- Symbol filtering correctly identifies symbols to ignore
- Delisting error detection works for various error message patterns
- Normal stocks are not incorrectly filtered

Run tests with:
```bash
cargo test
```

## Configuration

The filtering and delisting detection are enabled by default with no configuration required. The system will automatically:
- Filter symbols during initial stock list fetching
- Monitor for delisting errors during market data updates
- Maintain the database with current stock status

## Future Enhancements

Potential improvements could include:
- Configurable filtering rules
- Periodic re-checking of delisted stocks
- Additional delisting error patterns
- Notification system for newly delisted stocks
