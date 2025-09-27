use auto_analyser::database::Database;
use auto_analyser::web_api::StockAnalysisResult;
use chrono::Utc;
use tempfile::tempdir;

#[tokio::test]
async fn test_database_initialization() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let database = Database::new(&db_url).await;
    assert!(database.is_ok());
    
    let db = database.unwrap();
    let result = db.initialize_tables().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_store_and_retrieve_analysis_result() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_store.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let db = Database::new(&db_url).await.unwrap();
    db.initialize_tables().await.unwrap();
    
    // Create a test result
    let test_result = StockAnalysisResult {
        ticker: "TEST".to_string(),
        name: "Test Company".to_string(),
        current_price: Some(100.0),
        rsi: Some(45.0),
        sma_20: Some(98.0),
        sma_50: Some(95.0),
        macd: Some(1.2),
        macd_signal: Some(1.0),
        macd_histogram: Some(0.2),
        volume: Some(1000000),
        pct_change: Some(2.5),
        market_cap: Some("$1B".to_string()),
        is_opportunity: false,
        signals: vec!["Test signal".to_string()],
        timestamp: Utc::now(),
    };
    
    // Store the result
    let store_result = db.store_analysis_result(&test_result, "test_session").await;
    assert!(store_result.is_ok());
    
    // Retrieve results
    let retrieved = db.get_latest_results(Some(10)).await.unwrap();
    assert_eq!(retrieved.len(), 1);
    assert_eq!(retrieved[0].ticker, "TEST");
    assert_eq!(retrieved[0].name, "Test Company");
}

#[tokio::test]
async fn test_get_analysis_stats() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_stats.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let db = Database::new(&db_url).await.unwrap();
    db.initialize_tables().await.unwrap();
    
    // Store multiple test results
    for i in 0..5 {
        let result = StockAnalysisResult {
            ticker: format!("TEST{}", i),
            name: format!("Test Company {}", i),
            current_price: Some(100.0 + i as f64),
            rsi: Some(30.0 + i as f64 * 10.0),
            sma_20: Some(98.0),
            sma_50: Some(95.0),
            macd: Some(1.2),
            macd_signal: Some(1.0),
            macd_histogram: Some(0.2),
            volume: Some(1000000),
            pct_change: Some(2.5),
            market_cap: Some("$1B".to_string()),
            is_opportunity: i % 2 == 0, // Every other one is an opportunity
            signals: vec![],
            timestamp: Utc::now(),
        };
        
        db.store_analysis_result(&result, "test_session").await.unwrap();
    }
    
    // Get statistics
    let stats = db.get_analysis_stats().await.unwrap();
    assert_eq!(stats.total_results, 5);
    assert_eq!(stats.unique_tickers, 5);
    assert_eq!(stats.opportunities, 3); // 0, 2, 4 are opportunities
    assert!(stats.avg_rsi.is_some());
}

#[tokio::test]
async fn test_cleanup_old_results() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_cleanup.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let db = Database::new(&db_url).await.unwrap();
    db.initialize_tables().await.unwrap();
    
    // Store a test result
    let result = StockAnalysisResult {
        ticker: "CLEANUP_TEST".to_string(),
        name: "Cleanup Test".to_string(),
        current_price: Some(100.0),
        rsi: Some(45.0),
        sma_20: Some(98.0),
        sma_50: Some(95.0),
        macd: Some(1.2),
        macd_signal: Some(1.0),
        macd_histogram: Some(0.2),
        volume: Some(1000000),
        pct_change: Some(2.5),
        market_cap: Some("$1B".to_string()),
        is_opportunity: false,
        signals: vec![],
        timestamp: Utc::now(),
    };
    
    db.store_analysis_result(&result, "cleanup_session").await.unwrap();
    
    // Verify result exists
    let results_before = db.get_latest_results(None).await.unwrap();
    assert_eq!(results_before.len(), 1);
    
    // Clean up very recent results (should not remove anything with 30 day threshold)
    let cleaned = db.cleanup_old_results(30).await.unwrap();
    
    // With 30 days threshold, no recent results should be cleaned
    let _results_after = db.get_latest_results(None).await.unwrap();
    // The cleanup function should run without error and return a count
    assert_eq!(cleaned, 0); // No results should be older than 30 days
}

#[tokio::test]
async fn test_get_results_by_session() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_session.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let db = Database::new(&db_url).await.unwrap();
    db.initialize_tables().await.unwrap();
    
    // Store results for different sessions
    let sessions = ["session1", "session2"];
    
    for (i, session) in sessions.iter().enumerate() {
        let result = StockAnalysisResult {
            ticker: format!("TEST_{}", i),
            name: format!("Test Company {}", i),
            current_price: Some(100.0),
            rsi: Some(45.0),
            sma_20: Some(98.0),
            sma_50: Some(95.0),
            macd: Some(1.2),
            macd_signal: Some(1.0),
            macd_histogram: Some(0.2),
            volume: Some(1000000),
            pct_change: Some(2.5),
            market_cap: Some("$1B".to_string()),
            is_opportunity: false,
            signals: vec![],
            timestamp: Utc::now(),
        };
        
        db.store_analysis_result(&result, session).await.unwrap();
    }
    
    // Retrieve results for session1
    let session1_results = db.get_results_by_session("session1").await.unwrap();
    assert_eq!(session1_results.len(), 1);
    assert_eq!(session1_results[0].ticker, "TEST_0");
    
    // Retrieve results for session2
    let session2_results = db.get_results_by_session("session2").await.unwrap();
    assert_eq!(session2_results.len(), 1);
    assert_eq!(session2_results[0].ticker, "TEST_1");
}

#[tokio::test]
async fn test_duplicate_ticker_handling() {
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_duplicate.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    
    let db = Database::new(&db_url).await.unwrap();
    db.initialize_tables().await.unwrap();
    
    let session = "duplicate_test";
    
    // Store initial result
    let result1 = StockAnalysisResult {
        ticker: "DUPLICATE".to_string(),
        name: "Duplicate Test".to_string(),
        current_price: Some(100.0),
        rsi: Some(45.0),
        sma_20: Some(98.0),
        sma_50: Some(95.0),
        macd: Some(1.2),
        macd_signal: Some(1.0),
        macd_histogram: Some(0.2),
        volume: Some(1000000),
        pct_change: Some(2.5),
        market_cap: Some("$1B".to_string()),
        is_opportunity: false,
        signals: vec![],
        timestamp: Utc::now(),
    };
    
    db.store_analysis_result(&result1, session).await.unwrap();
    
    // Store updated result with same ticker and session (should replace)
    let result2 = StockAnalysisResult {
        ticker: "DUPLICATE".to_string(),
        name: "Duplicate Test Updated".to_string(),
        current_price: Some(105.0),
        rsi: Some(50.0),
        sma_20: Some(99.0),
        sma_50: Some(96.0),
        macd: Some(1.3),
        macd_signal: Some(1.1),
        macd_histogram: Some(0.2),
        volume: Some(1100000),
        pct_change: Some(3.0),
        market_cap: Some("$1.1B".to_string()),
        is_opportunity: true,
        signals: vec!["Updated signal".to_string()],
        timestamp: Utc::now(),
    };
    
    db.store_analysis_result(&result2, session).await.unwrap();
    
    // Should only have one result (the updated one)
    let results = db.get_results_by_session(session).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Duplicate Test Updated");
    assert_eq!(results[0].current_price, Some(105.0));
    assert_eq!(results[0].is_opportunity, true);
}