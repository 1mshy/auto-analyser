use anyhow::Result;
/// # Yahoo Finance API Rate Limit Test
///
/// This example tests the Yahoo Finance API to determine:
/// 1. How many requests can be made before getting blocked
/// 2. What types of errors/blocks are encountered
/// 3. How long blocks last
/// 4. Optimal request timing to avoid blocks
///
/// Run this example with: `cargo run --example yahoo_api_rate_limit_test`
use auto_analyser::StockAnalyzer;
use chrono::{Duration, Utc};
use std::time::{Duration as StdDuration, Instant};
use tokio::time::sleep;

#[derive(Debug, Clone)]
struct RequestResult {
    request_number: u32,
    symbol: String,
    success: bool,
    error_type: Option<String>,
    response_time_ms: u64,
    timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug)]
struct TestConfig {
    test_symbols: Vec<String>,
    delay_between_requests_ms: u64,
    max_requests: u32,
    test_different_endpoints: bool,
    retry_on_error: bool,
    retry_attempts: u32,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            test_symbols: vec![
                "AAPL".to_string(),
                "GOOGL".to_string(),
                "MSFT".to_string(),
                "TSLA".to_string(),
                "AMZN".to_string(),
                "META".to_string(),
                "NVDA".to_string(),
                "NFLX".to_string(),
                "AMD".to_string(),
                "INTC".to_string(),
            ],
            delay_between_requests_ms: 100, // Start with 100ms delay
            max_requests: 1000,
            test_different_endpoints: true,
            retry_on_error: false,
            retry_attempts: 3,
        }
    }
}

struct RateLimitTester {
    analyzer: StockAnalyzer,
    config: TestConfig,
    results: Vec<RequestResult>,
    start_time: Instant,
}

impl RateLimitTester {
    fn new(config: TestConfig) -> Self {
        Self {
            analyzer: StockAnalyzer::new(),
            config,
            results: Vec::new(),
            start_time: Instant::now(),
        }
    }

    async fn run_test(&mut self) -> Result<()> {
        println!("🧪 Yahoo Finance API Rate Limit Test");
        println!("{}", "=".repeat(60));
        println!("📋 Test Configuration:");
        println!("   • Max requests: {}", self.config.max_requests);
        println!(
            "   • Delay between requests: {}ms",
            self.config.delay_between_requests_ms
        );
        println!("   • Test symbols: {:?}", self.config.test_symbols);
        println!(
            "   • Test different endpoints: {}",
            self.config.test_different_endpoints
        );
        println!("   • Retry on error: {}", self.config.retry_on_error);
        println!();

        let mut consecutive_failures = 0;
        let max_consecutive_failures = 10;

        for request_num in 1..=self.config.max_requests {
            // Choose symbol (cycle through the list)
            let symbol_index = (request_num - 1) as usize % self.config.test_symbols.len();
            let symbol = &self.config.test_symbols[symbol_index];

            // Alternate between different API endpoints to test each one
            let result = if self.config.test_different_endpoints {
                match request_num % 3 {
                    0 => self.test_latest_quote(request_num, symbol).await,
                    1 => self.test_historical_data(request_num, symbol).await,
                    _ => self.test_search_ticker(request_num, symbol).await,
                }
            } else {
                self.test_latest_quote(request_num, symbol).await
            };

            self.results.push(result.clone());

            // Print progress every 10 requests
            if request_num % 10 == 0 {
                self.print_progress_update(request_num).await;
            }

            // Track consecutive failures
            if !result.success {
                consecutive_failures += 1;
                println!("❌ Request {} failed: {:?}", request_num, result.error_type);

                // If we hit too many consecutive failures, we might be blocked
                if consecutive_failures >= max_consecutive_failures {
                    println!(
                        "🚫 Hit {} consecutive failures. Likely rate limited!",
                        max_consecutive_failures
                    );

                    // Try waiting longer and test if block is temporary
                    println!("⏳ Testing if block is temporary by waiting 60 seconds...");
                    sleep(StdDuration::from_secs(60)).await;

                    let test_result = self.test_latest_quote(request_num + 1000, "AAPL").await;
                    if test_result.success {
                        println!("✅ Block appears to be temporary! Continuing test...");
                        consecutive_failures = 0;
                    } else {
                        println!("❌ Still blocked after 60 seconds. Ending test.");
                        break;
                    }
                }
            } else {
                consecutive_failures = 0;
            }

            // Delay between requests
            if self.config.delay_between_requests_ms > 0 {
                sleep(StdDuration::from_millis(
                    self.config.delay_between_requests_ms,
                ))
                .await;
            }
        }

        self.print_final_results().await;
        Ok(())
    }

    async fn test_latest_quote(&self, request_num: u32, symbol: &str) -> RequestResult {
        let start = Instant::now();
        let timestamp = Utc::now();

        match self.analyzer.get_latest_quote(symbol).await {
            Ok(_) => RequestResult {
                request_number: request_num,
                symbol: symbol.to_string(),
                success: true,
                error_type: None,
                response_time_ms: start.elapsed().as_millis() as u64,
                timestamp,
            },
            Err(e) => RequestResult {
                request_number: request_num,
                symbol: symbol.to_string(),
                success: false,
                error_type: Some(self.categorize_error(&e)),
                response_time_ms: start.elapsed().as_millis() as u64,
                timestamp,
            },
        }
    }

    async fn test_historical_data(&self, request_num: u32, symbol: &str) -> RequestResult {
        let start = Instant::now();
        let timestamp = Utc::now();
        let end = Utc::now();
        let start_date = end - Duration::days(7); // Last 7 days

        match self
            .analyzer
            .fetch_stock_data(symbol, start_date, end)
            .await
        {
            Ok(_) => RequestResult {
                request_number: request_num,
                symbol: symbol.to_string(),
                success: true,
                error_type: None,
                response_time_ms: start.elapsed().as_millis() as u64,
                timestamp,
            },
            Err(e) => RequestResult {
                request_number: request_num,
                symbol: symbol.to_string(),
                success: false,
                error_type: Some(self.categorize_error(&e)),
                response_time_ms: start.elapsed().as_millis() as u64,
                timestamp,
            },
        }
    }

    async fn test_search_ticker(&self, request_num: u32, symbol: &str) -> RequestResult {
        let start = Instant::now();
        let timestamp = Utc::now();

        // This simulates a search/lookup operation by getting a quote
        // In a real scenario, you might use a search endpoint if available
        match self.analyzer.get_latest_quote(symbol).await {
            Ok(_) => RequestResult {
                request_number: request_num,
                symbol: symbol.to_string(),
                success: true,
                error_type: None,
                response_time_ms: start.elapsed().as_millis() as u64,
                timestamp,
            },
            Err(e) => RequestResult {
                request_number: request_num,
                symbol: symbol.to_string(),
                success: false,
                error_type: Some(self.categorize_error(&e)),
                response_time_ms: start.elapsed().as_millis() as u64,
                timestamp,
            },
        }
    }

    fn categorize_error(&self, error: &anyhow::Error) -> String {
        let error_str = error.to_string().to_lowercase();

        if error_str.contains("429") || error_str.contains("too many requests") {
            "RATE_LIMIT_429".to_string()
        } else if error_str.contains("403") || error_str.contains("forbidden") {
            "FORBIDDEN_403".to_string()
        } else if error_str.contains("401") || error_str.contains("unauthorized") {
            "UNAUTHORIZED_401".to_string()
        } else if error_str.contains("timeout") {
            "TIMEOUT".to_string()
        } else if error_str.contains("connection") {
            "CONNECTION_ERROR".to_string()
        } else if error_str.contains("502")
            || error_str.contains("503")
            || error_str.contains("504")
        {
            "SERVER_ERROR_5XX".to_string()
        } else {
            format!("OTHER: {}", error_str)
        }
    }

    async fn print_progress_update(&self, current_request: u32) {
        let success_count = self.results.iter().filter(|r| r.success).count();
        let failure_count = self.results.len() - success_count;
        let success_rate = (success_count as f64 / self.results.len() as f64) * 100.0;

        let avg_response_time = if !self.results.is_empty() {
            self.results.iter().map(|r| r.response_time_ms).sum::<u64>() / self.results.len() as u64
        } else {
            0
        };

        println!(
            "📊 Progress: {}/{} requests | ✅ {:.1}% success | ⏱️ {}ms avg | ❌ {} failures",
            current_request,
            self.config.max_requests,
            success_rate,
            avg_response_time,
            failure_count
        );
    }

    async fn print_final_results(&self) {
        println!("\n🏁 Final Test Results");
        println!("{}", "=".repeat(60));

        let total_requests = self.results.len();
        let successful_requests = self.results.iter().filter(|r| r.success).count();
        let failed_requests = total_requests - successful_requests;
        let success_rate = (successful_requests as f64 / total_requests as f64) * 100.0;
        let total_time = self.start_time.elapsed();

        println!("📊 Overall Statistics:");
        println!("   • Total requests: {}", total_requests);
        println!("   • Successful requests: {}", successful_requests);
        println!("   • Failed requests: {}", failed_requests);
        println!("   • Success rate: {:.2}%", success_rate);
        println!("   • Total test time: {:.2}s", total_time.as_secs_f64());
        println!(
            "   • Requests per second: {:.2}",
            total_requests as f64 / total_time.as_secs_f64()
        );

        // Response time statistics
        if !self.results.is_empty() {
            let response_times: Vec<u64> =
                self.results.iter().map(|r| r.response_time_ms).collect();
            let avg_response_time =
                response_times.iter().sum::<u64>() / response_times.len() as u64;
            let min_response_time = *response_times.iter().min().unwrap();
            let max_response_time = *response_times.iter().max().unwrap();

            println!("\n⏱️ Response Time Statistics:");
            println!("   • Average: {}ms", avg_response_time);
            println!("   • Minimum: {}ms", min_response_time);
            println!("   • Maximum: {}ms", max_response_time);

            // Check for high response times
            if avg_response_time > 1000 {
                println!("   • High response times detected - server may be under load");
            }
        }

        // Error analysis
        let mut error_counts = std::collections::HashMap::new();
        for result in &self.results {
            if !result.success {
                if let Some(error_type) = &result.error_type {
                    *error_counts.entry(error_type.clone()).or_insert(0) += 1;
                }
            }
        }

        if !error_counts.is_empty() {
            println!("\n❌ Error Breakdown:");
            for (error_type, count) in error_counts {
                println!("   • {}: {}", error_type, count);
            }
        }

        // Find when rate limiting started (if any)
        let first_failure = self.results.iter().find(|r| !r.success);
        if let Some(failure) = first_failure {
            println!("\n🚫 First Failure:");
            println!("   • Request number: {}", failure.request_number);
            println!("   • Error type: {:?}", failure.error_type);
            println!("   • Time: {}", failure.timestamp.format("%H:%M:%S"));
        }

        // Recommendations
        println!("\n💡 Recommendations:");
        if success_rate > 95.0 {
            println!("   • Current request rate seems sustainable");
            println!("   • Consider increasing request frequency for better throughput");
        } else if success_rate > 80.0 {
            println!("   • Some rate limiting detected");
            println!("   • Consider increasing delay between requests");
        } else {
            println!("   • Significant rate limiting detected");
            println!(
                "   • Recommend increasing delay to {}ms or more",
                self.config.delay_between_requests_ms * 2
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // You can customize the test configuration here
    let config = TestConfig {
        delay_between_requests_ms: 50,  // Start with 50ms delay
        max_requests: 3000,             // Test with 100 requests initially
        test_different_endpoints: true, // Test multiple endpoints
        ..Default::default()
    };

    let mut tester = RateLimitTester::new(config);
    tester.run_test().await?;

    println!("\n🔄 Want to test with different parameters?");
    println!("   Edit the TestConfig in main() and run again with:");
    println!("   cargo run --example yahoo_api_rate_limit_test");

    Ok(())
}
