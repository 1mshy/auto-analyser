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

    let top = StockAnalyzer::fetch_n_tickers(100).await.unwrap();

    // First, demonstrate ticker collection
    println!("📡 Fetching available tickers from Nasdaq...");
    match StockAnalyzer::fetch_all_tickers().await {
        Ok(all_tickers) => {
            println!("✅ Fetched {} tickers successfully!", all_tickers.len());
            for ticker in all_tickers {
                prior.push(ticker.symbol, 5);
            }
            for ticker in top {
                prior.push(ticker.symbol, 10);

            }

            while let Some((ticker, _priority)) = prior.pop() {
                let stock_data = match analyser.fetch_all_stock_data(&ticker).await {
                    Ok(data) => data,
                    Err(e) => {
                        println!("{}", e);
                        println!("Failed to fetch data for {}", ticker);
                        continue;
                    }
                };
                let ticker_indicators = analyser.calculate_indicators(&ticker, &stock_data);
                let current_rsi = ticker_indicators[ticker_indicators.len() - 1].rsi.unwrap();
                if current_rsi < 30.0 {
                    println!("{} is at a {} rsi", ticker, current_rsi);
                }
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
