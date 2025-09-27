use auto_analyser::{StockData, StockAnalyzer, TechnicalIndicators};
use chrono::Utc;

#[test]
fn test_stock_data_creation() {
    let stock_data = StockData {
        symbol: "AAPL".to_string(),
        timestamp: Utc::now(),
        open: 150.0,
        high: 155.0,
        low: 149.0,
        close: 154.0,
        volume: 1000000,
    };

    assert_eq!(stock_data.symbol, "AAPL");
    assert_eq!(stock_data.open, 150.0);
    assert_eq!(stock_data.high, 155.0);
    assert_eq!(stock_data.low, 149.0);
    assert_eq!(stock_data.close, 154.0);
    assert_eq!(stock_data.volume, 1000000);
}

#[test]
fn test_technical_indicators_creation() {
    let indicators = TechnicalIndicators {
        sma_20: Some(152.0),
        sma_50: Some(150.0),
        rsi: Some(65.0),
        macd: Some((1.2, 1.0, 0.2)),
    };

    assert_eq!(indicators.sma_20, Some(152.0));
    assert_eq!(indicators.sma_50, Some(150.0));
    assert_eq!(indicators.rsi, Some(65.0));
    assert_eq!(indicators.macd, Some((1.2, 1.0, 0.2)));
}

#[test]
fn test_stock_analyzer_creation() {
    let _analyzer = StockAnalyzer::new();
    // Test that analyzer can be created without panicking
    assert!(true);
}

#[test]
fn test_stock_analyzer_with_cache_creation() {
    use auto_analyser::cache::CacheManager;
    
    let cache = CacheManager::new();
    let _analyzer = StockAnalyzer::new_with_cache(cache);
    // Test that analyzer with cache can be created without panicking
    assert!(true);
}

#[tokio::test]
async fn test_technical_indicators_calculation() {
    let mut analyzer = StockAnalyzer::new();
    
    // Create sample stock data
    let mut stock_data = Vec::new();
    let base_time = Utc::now();
    
    // Generate 50 days of sample data for proper SMA calculation
    for i in 0..50 {
        stock_data.push(StockData {
            symbol: "TEST".to_string(),
            timestamp: base_time + chrono::Duration::days(i as i64),
            open: 100.0 + i as f64,
            high: 105.0 + i as f64,
            low: 95.0 + i as f64,
            close: 102.0 + i as f64,
            volume: 1000000,
        });
    }
    
    let indicators = analyzer.calculate_indicators("TEST", &stock_data);
    
    // Should have calculated indicators for all data points
    assert_eq!(indicators.len(), 50);
    
    // The last indicator should have valid SMA values
    let last_indicator = indicators.last().unwrap();
    assert!(last_indicator.sma_20.is_some());
    assert!(last_indicator.sma_50.is_some());
    assert!(last_indicator.rsi.is_some());
    assert!(last_indicator.macd.is_some());
}

#[tokio::test]
async fn test_cache_functionality() {
    use auto_analyser::cache::CacheManager;
    
    let cache = CacheManager::new();
    
    // Test stock data caching
    let test_data = vec![StockData {
        symbol: "CACHE_TEST".to_string(),
        timestamp: Utc::now(),
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000000,
    }];
    
    let cache_key = "test_stock_data".to_string();
    
    // Cache the data
    cache.cache_stock_data(cache_key.clone(), test_data.clone()).await;
    
    // Retrieve from cache
    let cached_data = cache.get_stock_data(&cache_key).await;
    assert!(cached_data.is_some());
    assert_eq!(cached_data.unwrap().len(), 1);
}

#[test]
fn test_stock_filter_creation() {
    use auto_analyser::StockFilter;
    
    let filter = StockFilter::new()
        .with_market_cap_range(Some(1_000_000_000.0), Some(10_000_000_000.0))
        .with_price_range(Some(10.0), Some(100.0))
        .with_rsi_thresholds(Some(30.0), Some(70.0));
    
    assert_eq!(filter.min_market_cap, Some(1_000_000_000.0));
    assert_eq!(filter.max_market_cap, Some(10_000_000_000.0));
    assert_eq!(filter.min_price, Some(10.0));
    assert_eq!(filter.max_price, Some(100.0));
    assert_eq!(filter.oversold_rsi_threshold, Some(30.0));
    assert_eq!(filter.overbought_rsi_threshold, Some(70.0));
}

#[test]
fn test_analyze_signals() {
    let analyzer = StockAnalyzer::new();
    
    let stock_data = StockData {
        symbol: "SIGNAL_TEST".to_string(),
        timestamp: Utc::now(),
        open: 100.0,
        high: 105.0,
        low: 95.0,
        close: 102.0,
        volume: 1000000,
    };
    
    // Test oversold condition
    let oversold_indicators = TechnicalIndicators {
        sma_20: Some(100.0),
        sma_50: Some(98.0),
        rsi: Some(25.0), // Oversold
        macd: Some((0.5, 0.3, 0.2)),
    };
    
    let signals = analyzer.analyze_signals(&stock_data, &oversold_indicators);
    assert!(signals.iter().any(|s| s.contains("Oversold")));
    
    // Test overbought condition
    let overbought_indicators = TechnicalIndicators {
        sma_20: Some(100.0),
        sma_50: Some(98.0),
        rsi: Some(75.0), // Overbought
        macd: Some((0.5, 0.3, 0.2)),
    };
    
    let signals = analyzer.analyze_signals(&stock_data, &overbought_indicators);
    assert!(signals.iter().any(|s| s.contains("Overbought")));
}

#[tokio::test]
async fn test_rate_limiting() {
    use auto_analyser::cache::CacheManager;
    use std::time::Duration;
    
    let cache = CacheManager::new();
    
    // First request should not be rate limited
    assert!(!cache.should_rate_limit("test_symbol", Duration::from_millis(100)));
    
    // Immediate second request should be rate limited
    assert!(cache.should_rate_limit("test_symbol", Duration::from_millis(100)));
    
    // Wait for rate limit to expire
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Should not be rate limited anymore
    assert!(!cache.should_rate_limit("test_symbol", Duration::from_millis(100)));
}

#[test]
fn test_market_cap_parsing() {
    use auto_analyser::analyzer::StockAnalyzer;
    
    // Test billion parsing
    assert_eq!(StockAnalyzer::parse_market_cap("$1.5B").unwrap(), 1_500_000_000.0);
    
    // Test million parsing
    assert_eq!(StockAnalyzer::parse_market_cap("$500M").unwrap(), 500_000_000.0);
    
    // Test thousand parsing
    assert_eq!(StockAnalyzer::parse_market_cap("$1.2K").unwrap(), 1_200.0);
    
    // Test direct number
    assert_eq!(StockAnalyzer::parse_market_cap("$1000").unwrap(), 1000.0);
}

#[test]
fn test_percentage_parsing() {
    use auto_analyser::analyzer::StockAnalyzer;
    
    // Test positive percentage
    assert_eq!(StockAnalyzer::parse_percentage("5.5%").unwrap(), 5.5);
    
    // Test negative percentage
    assert_eq!(StockAnalyzer::parse_percentage("-2.3%").unwrap(), -2.3);
    
    // Test zero percentage
    assert_eq!(StockAnalyzer::parse_percentage("0.0%").unwrap(), 0.0);
}