# Yahoo Finance API Rate Limit Test

This example script tests the Yahoo Finance API to determine rate limiting behavior and optimal request patterns.

## What the Script Tests

1. **Request Limits**: How many requests can be made before getting blocked
2. **Error Types**: What kinds of errors/blocks are encountered (429, 403, timeouts, etc.)
3. **Response Times**: Average, min, and max response times
4. **Recovery Time**: How long rate limit blocks last
5. **Different Endpoints**: Tests multiple API endpoints (quotes, historical data, search)

## How to Run

```bash
# Run with default configuration (100 requests, 50ms delay)
cargo run --example yahoo_api_rate_limit_test

# Or compile first then run
cargo build --example yahoo_api_rate_limit_test
./target/debug/examples/yahoo_api_rate_limit_test
```

## Configuration Options

You can modify the `TestConfig` in the `main()` function to customize the test:

```rust
let config = TestConfig {
    delay_between_requests_ms: 50,  // Delay between requests in milliseconds
    max_requests: 100,              // Maximum number of requests to make
    test_different_endpoints: true,  // Test multiple API endpoints
    test_symbols: vec![             // Stock symbols to test with
        "AAPL".to_string(), 
        "GOOGL".to_string(), 
        // ... add more symbols
    ],
    retry_on_error: false,          // Whether to retry failed requests
    retry_attempts: 3,              // Number of retry attempts
};
```

## Test Scenarios

### Scenario 1: Find Basic Rate Limit
```rust
TestConfig {
    delay_between_requests_ms: 0,   // No delay - fastest possible
    max_requests: 1000,
    test_different_endpoints: false, // Single endpoint only
    ..Default::default()
}
```

### Scenario 2: Test Sustainable Rate
```rust
TestConfig {
    delay_between_requests_ms: 100, // 100ms delay
    max_requests: 500,
    test_different_endpoints: true,
    ..Default::default()
}
```

### Scenario 3: Stress Test
```rust
TestConfig {
    delay_between_requests_ms: 10,  // Very fast requests
    max_requests: 2000,
    test_different_endpoints: true,
    ..Default::default()
}
```

## Expected Results

The script will output:
- Progress updates every 10 requests
- Real-time error reporting
- Final statistics including success rate and response times
- Error breakdown by type
- Recommendations for optimal request timing

## Interpreting Results

- **Success Rate > 95%**: Current rate is sustainable
- **Success Rate 80-95%**: Some rate limiting detected, consider slower requests
- **Success Rate < 80%**: Significant rate limiting, increase delays
- **High response times (>1000ms)**: Server under load
- **429 errors**: Explicit rate limiting
- **403 errors**: Access forbidden (may indicate IP blocking)

## Safety Notes

- The script starts conservatively to avoid immediate blocking
- It includes automatic detection of temporary vs permanent blocks
- Consider running during off-peak hours to get more representative results
- Yahoo Finance may have different limits for different API endpoints

## Understanding Yahoo Finance Limits

Based on typical behavior:
- **Free tier**: Usually allows 100-2000 requests per hour
- **Rate limits**: Often enforced per IP address
- **Burst limits**: May allow rapid requests initially, then throttle
- **Recovery time**: Blocks typically last 15-60 minutes
