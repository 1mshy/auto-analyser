use anyhow::Result;
/// # Yahoo Finance API Aggressive Rate Limit Test
///
/// This example aggressively tests the Yahoo Finance API to trigger rate limiting.
/// It makes concurrent requests within chunks to find the exact blocking point.
/// You can control the chunk size (concurrent requests per chunk) and delay between chunks.
///
/// Run this example with:
/// - Default: `cargo run --example yahoo_api_aggressive_test`
/// - With chunk size: `cargo run --example yahoo_api_aggressive_test -- --chunk-size 10`
/// - With chunk size and delay: `cargo run --example yahoo_api_aggressive_test -- --chunk-size 10 --chunk-delay 1000`
///
/// Note: Requests within each chunk are sent CONCURRENTLY for maximum impact!
use auto_analyser::StockAnalyzer;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::time::{Duration as StdDuration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let (chunk_size, chunk_delay_ms) = (50, 0);

    println!("⚡ Yahoo Finance API - AGGRESSIVE Rate Limit Test");
    println!("{}", "=".repeat(60));
    println!("⚠️  WARNING: This test will likely trigger rate limiting!");
    println!("   It's designed to find the exact blocking threshold.");
    println!();
    println!("📦 Configuration:");
    println!("   • Chunk size: {} requests per chunk", chunk_size);
    println!("   • Delay between chunks: {}ms", chunk_delay_ms);
    println!();

    let analyzer = Arc::new(StockAnalyzer::new());
    let binding = StockAnalyzer::fetch_all_tickers().await?;
    let symbols = binding
        .iter()
        .filter(|t| !(t.symbol.contains("^") || t.symbol.contains("/")))
        .map(|t| t.symbol.as_str())
        .collect::<Vec<&str>>();
    let mut successful_requests = 0;
    let mut failed_requests = 0;
    let mut first_failure_at = None;
    let start_time = Instant::now();
    let mut chunk_count = 0;

    println!("🚀 Starting concurrent chunked requests...");
    println!("   Testing symbols: {:?}", symbols);
    println!("   • Requests within each chunk will be sent CONCURRENTLY");
    println!();

    let mut request_num = 1;
    let max_requests = 100000;

    while request_num <= max_requests {
        chunk_count += 1;
        let chunk_start = Instant::now();

        // Process a chunk of requests CONCURRENTLY
        let mut futures = Vec::new();

        // Collect all the requests for this chunk
        let mut chunk_requests = Vec::new();
        for _ in 0..chunk_size {
            if request_num > max_requests {
                break;
            }

            let symbol = symbols[(request_num - 1) % symbols.len()].to_string();
            chunk_requests.push((request_num, symbol));
            request_num += 1;
        }

        // Create concurrent futures for all requests in this chunk
        for (req_num, symbol) in chunk_requests.iter() {
            let analyzer_clone = Arc::clone(&analyzer);
            let symbol_clone = symbol.clone();
            let req_num_clone = *req_num;

            let future = tokio::spawn(async move {
                let request_start = Instant::now();
                let result = analyzer_clone
                    .fetch_stock_data(&symbol_clone, DateTime::<Utc>::UNIX_EPOCH, Utc::now())
                    .await;
                (req_num_clone, symbol_clone, request_start, result)
            });

            futures.push(future);
        }

        // Wait for all requests in the chunk to complete
        let mut results = Vec::new();
        for future in futures {
            match future.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    println!("❌ Task join error: {}", e);
                    continue;
                }
            }
        }

        // Process the results
        for (req_num, symbol, request_start, result) in results {
            match result {
                Ok(_) => {
                    successful_requests += 1;
                    let response_time = request_start.elapsed().as_millis();

                    if req_num % 25 == 0 {
                        println!(
                            "✅ Request {}: {} - {}ms (concurrent)",
                            req_num, symbol, response_time
                        );
                    }
                }
                Err(e) => {
                    failed_requests += 1;
                    let error_msg = e.to_string();

                    if first_failure_at.is_none() {
                        first_failure_at = Some(req_num);
                        println!("🚫 FIRST FAILURE at request {}: {}", req_num, error_msg);
                    }

                    println!("❌ Request {}: {} - ERROR: {}", req_num, symbol, error_msg);

                    // If we hit 5 consecutive failures, we're likely blocked
                    if failed_requests >= 5 {
                        println!("🛑 Hit {} failures - likely rate limited!", failed_requests);

                        // Test different waiting periods to see recovery time
                        println!("\n🕐 Testing recovery time...");

                        for wait_time in [10, 30, 60, 120] {
                            println!("   Waiting {} seconds...", wait_time);
                            sleep(StdDuration::from_secs(wait_time)).await;

                            match analyzer.get_latest_quote("AAPL").await {
                                Ok(_) => {
                                    println!("   ✅ Recovered after {} seconds!", wait_time);
                                    break;
                                }
                                Err(e) => {
                                    println!("   ❌ Still blocked: {}", e.to_string());
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        // If we hit failures, break out of the chunk loop
        if failed_requests >= 5 {
            break;
        }

        let chunk_duration = chunk_start.elapsed().as_millis();
        if chunk_count % 10 == 0 {
            println!(
                "📦 Chunk {} completed in {}ms ({} requests)",
                chunk_count,
                chunk_duration,
                chunk_size.min(max_requests - request_num + chunk_size + 1)
            );
        }

        // Delay between chunks if specified
        if chunk_delay_ms > 0 {
            sleep(StdDuration::from_millis(chunk_delay_ms)).await;
        }
    }

    let total_time = start_time.elapsed();

    println!("\n📊 CHUNKED TEST RESULTS");
    println!("{}", "=".repeat(50));
    println!("• Chunk size: {} requests per chunk", chunk_size);
    println!("• Chunk delay: {}ms", chunk_delay_ms);
    println!("• Total chunks processed: {}", chunk_count);
    println!(
        "• Total requests attempted: {}",
        successful_requests + failed_requests
    );
    println!("• Successful requests: {}", successful_requests);
    println!("• Failed requests: {}", failed_requests);
    println!(
        "• Success rate: {:.2}%",
        (successful_requests as f64 / (successful_requests + failed_requests) as f64) * 100.0
    );
    println!("• Test duration: {:.2}s", total_time.as_secs_f64());
    println!(
        "• Requests per second: {:.2}",
        (successful_requests + failed_requests) as f64 / total_time.as_secs_f64()
    );
    println!(
        "• Chunks per second: {:.2}",
        chunk_count as f64 / total_time.as_secs_f64()
    );

    if let Some(failure_point) = first_failure_at {
        println!("• First failure at request: {}", failure_point);
        println!(
            "• Rate limit threshold: ~{} successful requests",
            failure_point - 1
        );
    } else {
        println!("• No rate limiting detected!");
        println!("• Yahoo Finance API appears very permissive");
    }

    println!("\n💡 Key Findings:");
    if successful_requests > 500 {
        println!("   • Yahoo Finance allows high request volumes");
        println!(
            "   • Rate limiting threshold is > {} requests",
            successful_requests
        );
    } else if successful_requests > 100 {
        println!("   • Moderate rate limiting detected");
        println!(
            "   • Threshold appears to be around {} requests",
            successful_requests
        );
    } else {
        println!("   • Strict rate limiting detected");
        println!(
            "   • Very low threshold of ~{} requests",
            successful_requests
        );
    }

    println!("\n⚠️  Remember:");
    println!("   • Rate limits may vary by time of day");
    println!("   • Limits may be per IP address");
    println!("   • Commercial use may have different limits");
    println!("   • Always respect API terms of service");
    println!("   • Use larger chunk delays to be more respectful");
    println!("   • Smaller chunk sizes help identify rate limit patterns");

    println!("\n💡 Usage Examples:");
    println!("   • Default (1 req/chunk, no delay): cargo run --example yahoo_api_aggressive_test");
    println!("   • 10 requests per chunk: cargo run --example yahoo_api_aggressive_test -- --chunk-size 10");
    println!("   • 5 req/chunk with 500ms delay: cargo run --example yahoo_api_aggressive_test -- --chunk-size 5 --chunk-delay 500");

    Ok(())
}
