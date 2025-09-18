use anyhow::Result;
/// # Yahoo Finance API Timing Pattern Test
///
/// This example tests different timing patterns to find optimal request rates
/// that avoid rate limiting while maximizing throughput.
///
/// Run this example with: `cargo run --example yahoo_api_timing_test`
use auto_analyser::StockAnalyzer;
use std::time::{Duration as StdDuration, Instant};
use tokio::time::sleep;

#[derive(Debug)]
struct TimingTestResult {
    delay_ms: u64,
    requests_tested: u32,
    success_rate: f64,
    avg_response_time_ms: u64,
    requests_per_second: f64,
}

async fn test_timing_pattern(
    analyzer: &StockAnalyzer,
    delay_ms: u64,
    test_requests: u32,
) -> Result<TimingTestResult> {
    let mut successful = 0;
    let mut total_response_time = 0u64;
    let start_time = Instant::now();

    println!(
        "   Testing {}ms delay with {} requests...",
        delay_ms, test_requests
    );

    for i in 1..=test_requests {
        let symbol = match i % 5 {
            0 => "AAPL",
            1 => "GOOGL",
            2 => "MSFT",
            3 => "TSLA",
            _ => "AMZN",
        };

        let request_start = Instant::now();
        match analyzer.get_latest_quote(symbol).await {
            Ok(_) => {
                successful += 1;
                total_response_time += request_start.elapsed().as_millis() as u64;
            }
            Err(_) => {
                // Request failed - likely rate limited
            }
        }

        if delay_ms > 0 {
            sleep(StdDuration::from_millis(delay_ms)).await;
        }
    }

    let total_time = start_time.elapsed();
    let success_rate = (successful as f64 / test_requests as f64) * 100.0;
    let avg_response_time = if successful > 0 {
        total_response_time / successful as u64
    } else {
        0
    };
    let requests_per_second = test_requests as f64 / total_time.as_secs_f64();

    Ok(TimingTestResult {
        delay_ms,
        requests_tested: test_requests,
        success_rate,
        avg_response_time_ms: avg_response_time,
        requests_per_second,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("⏱️  Yahoo Finance API - Timing Pattern Test");
    println!("{}", "=".repeat(60));
    println!("🎯 Finding optimal request timing for maximum throughput");
    println!("   without triggering rate limits...");
    println!();

    let analyzer = StockAnalyzer::new();
    let test_requests_per_pattern = 50; // Test each pattern with 50 requests

    // Test different delay patterns
    let delay_patterns = vec![0, 10, 25, 50, 100, 200, 500, 1000, 2000];
    let mut results = Vec::new();

    for delay_ms in delay_patterns {
        match test_timing_pattern(&analyzer, delay_ms, test_requests_per_pattern).await {
            Ok(result) => {
                println!(
                    "   ✅ {}ms delay: {:.1}% success, {:.2} req/s, {}ms avg response",
                    result.delay_ms,
                    result.success_rate,
                    result.requests_per_second,
                    result.avg_response_time_ms
                );
                results.push(result);
            }
            Err(e) => {
                println!("   ❌ {}ms delay: Error - {}", delay_ms, e);
            }
        }

        // Small break between timing tests
        sleep(StdDuration::from_secs(2)).await;
    }

    println!("\n📊 TIMING ANALYSIS RESULTS");
    println!("{}", "=".repeat(60));
    println!("| Delay (ms) | Success Rate | Req/Sec | Avg Response | Efficiency |");
    println!("| ---------- | ------------ | ------- | ------------ | ---------- |");

    let mut best_efficiency_score = 0.0;
    let mut best_config = None;

    for result in &results {
        // Efficiency score: balance success rate and throughput
        let efficiency = (result.success_rate / 100.0) * result.requests_per_second;

        if efficiency > best_efficiency_score {
            best_efficiency_score = efficiency;
            best_config = Some(result);
        }

        println!(
            "| {:>9} | {:>11.1}% | {:>7.2} | {:>9}ms | {:>9.2} |",
            result.delay_ms,
            result.success_rate,
            result.requests_per_second,
            result.avg_response_time_ms,
            efficiency
        );
    }

    println!();

    if let Some(best) = best_config {
        println!("🏆 OPTIMAL CONFIGURATION:");
        println!("   • Delay: {}ms between requests", best.delay_ms);
        println!("   • Success Rate: {:.1}%", best.success_rate);
        println!(
            "   • Throughput: {:.2} requests/second",
            best.requests_per_second
        );
        println!(
            "   • Response Time: {}ms average",
            best.avg_response_time_ms
        );
        println!("   • Efficiency Score: {:.2}", best_efficiency_score);
    }

    println!("\n💡 RECOMMENDATIONS:");

    // Find configurations with high success rates
    let reliable_configs: Vec<_> = results.iter().filter(|r| r.success_rate >= 95.0).collect();

    if let Some(fastest_reliable) = reliable_configs.iter().max_by(|a, b| {
        a.requests_per_second
            .partial_cmp(&b.requests_per_second)
            .unwrap()
    }) {
        println!(
            "   • For maximum reliability (95%+ success): {}ms delay",
            fastest_reliable.delay_ms
        );
        println!(
            "     └─ Achieves {:.2} req/s with {:.1}% success rate",
            fastest_reliable.requests_per_second, fastest_reliable.success_rate
        );
    }

    // Find fastest configuration that still works reasonably well
    let reasonable_configs: Vec<_> = results.iter().filter(|r| r.success_rate >= 80.0).collect();

    if let Some(fastest_reasonable) = reasonable_configs.iter().max_by(|a, b| {
        a.requests_per_second
            .partial_cmp(&b.requests_per_second)
            .unwrap()
    }) {
        println!(
            "   • For balanced performance (80%+ success): {}ms delay",
            fastest_reasonable.delay_ms
        );
        println!(
            "     └─ Achieves {:.2} req/s with {:.1}% success rate",
            fastest_reasonable.requests_per_second, fastest_reasonable.success_rate
        );
    }

    // Warn about rate limiting
    let heavily_limited_configs: Vec<_> =
        results.iter().filter(|r| r.success_rate < 50.0).collect();

    if !heavily_limited_configs.is_empty() {
        println!(
            "   ⚠️  Delays under {}ms show significant rate limiting",
            heavily_limited_configs
                .iter()
                .map(|r| r.delay_ms)
                .max()
                .unwrap_or(0)
        );
    }

    println!("\n🔍 USAGE GUIDELINES:");
    println!(
        "   • For production apps: Use {}ms+ delays for reliability",
        reliable_configs.first().map(|r| r.delay_ms).unwrap_or(100)
    );
    println!("   • For data collection: Monitor success rates and adjust timing");
    println!("   • For analysis scripts: Consider batch processing with delays");
    println!("   • Rate limits may vary by time of day and server load");

    Ok(())
}
