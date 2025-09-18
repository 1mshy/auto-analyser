use anyhow::Result;
use auto_analyser::StockAnalyzer;
use chrono::{Duration, Utc};
use priority_queue::PriorityQueue;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Auto Stock Analyser - Enhanced with Ticker Collection");
    println!("{}", "=".repeat(60));

    let mut prior: PriorityQueue<String, i32> = PriorityQueue::new();
    let mut analyser = StockAnalyzer::new();

    // First, demonstrate ticker collection
    println!("📡 Fetching available tickers from Nasdaq...");
    match StockAnalyzer::fetch_all_tickers().await {
        Ok(all_tickers) => {
            println!("✅ Fetched {} tickers successfully!", all_tickers.len());
            for ticker in all_tickers {
                prior.push(ticker.symbol, 5);
            }

            prior.push("TSLA".to_string(), 10);
            prior.push("AAPL".to_string(), 15);
            prior.push("MSFT".to_string(), 10);
            prior.push("GOOGL".to_string(), 10);
            prior.push("TSLA".to_string(), 20);
            prior.push("AMZN".to_string(), 15);

            while let Some((ticker, _priority)) = prior.pop() {
                let stock_data = analyser.fetch_all_stock_data(&ticker).await.unwrap();
                let ticker_indicators = analyser.calculate_indicators(&ticker, &stock_data);
                println!(
                    "{}: {:?}",
                    ticker,
                    ticker_indicators[ticker_indicators.len() - 1].rsi
                );
            }
        }
        Err(e) => {
            println!("❌ Error fetching tickers: {}", e);
            println!("🔄 Falling back to default symbols...");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("✨ Analysis complete!");
    println!("\n💡 Try these examples:");
    println!("  cargo run --example ticker_collection  # Explore all available tickers");
    println!("  cargo run --example simple_analysis    # Analyze a single stock");

    Ok(())
}
