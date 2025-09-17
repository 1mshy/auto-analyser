use auto_analyser::StockAnalyzer;
use chrono::{Utc, Duration};
use anyhow::Result;
use priority_queue::PriorityQueue;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Auto Stock Analyser - Enhanced with Ticker Collection");
    println!("{}", "=".repeat(60));

    let mut prior = PriorityQueue::new();
    prior.push("TSLA", 10);
    prior.push("AAPL", 15);
    prior.push("MSFT", 10);
    prior.push("GOOGL", 10);
    prior.push("TSLA", 20);
    prior.push("AMZN", 15);
    println!("{:?}", prior);
    while let Some((symbol, priority)) = prior.pop() {
        println!("Symbol: {}, Priority: {}", symbol, priority);
    }

    // First, demonstrate ticker collection
    println!("📡 Fetching available tickers from Nasdaq...");
    match StockAnalyzer::fetch_all_tickers().await {
        Ok(all_tickers) => {
            println!("✅ Fetched {} tickers successfully!", all_tickers.len());
            
            // Get top 5 performers to analyze
            let top_performers = StockAnalyzer::get_top_performers(&all_tickers, 5);
            StockAnalyzer::print_tickers(&top_performers, "🚀 Today's Top 5 Performers");
            
            // Get symbols from top performers for analysis
            let mut symbols_to_analyze: Vec<String> = top_performers
                .iter()
                .map(|t| t.symbol.clone())
                .collect();
            
            // Add some stable large-cap stocks for comparison
            symbols_to_analyze.extend(["AAPL", "GOOGL", "MSFT"].iter().map(|s| s.to_string()));
            
            // Remove duplicates
            symbols_to_analyze.sort();
            symbols_to_analyze.dedup();
            
            println!("\n📊 Will analyze these symbols: {:?}", symbols_to_analyze);
            
            // Technical analysis section
            let mut analyzer = StockAnalyzer::new();
            let end = Utc::now();
            let start = end - Duration::days(100);
            
            for symbol in symbols_to_analyze.iter().take(5) { // Limit to 5 for demo
                println!("\n{}", "=".repeat(50));
                
                match analyzer.fetch_stock_data(symbol, start, end).await {
                    Ok(stock_data) => {
                        if stock_data.is_empty() {
                            println!("❌ No data available for {}", symbol);
                            continue;
                        }
                        
                        let indicators = analyzer.calculate_indicators(symbol, &stock_data);
                        analyzer.print_analysis(symbol, &stock_data, &indicators);
                        
                        // Get latest quote
                        match analyzer.get_latest_quote(symbol).await {
                            Ok(latest) => {
                                println!("\n⚡ Latest Quote:");
                                println!("  Time: {}", latest.timestamp.format("%Y-%m-%d %H:%M UTC"));
                                println!("  Price: ${:.2}", latest.close);
                            }
                            Err(e) => println!("Failed to get latest quote: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to fetch data for {}: {}", symbol, e);
                    }
                }
                
                // Add a small delay to be respectful to the API
                // tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }
        Err(e) => {
            println!("❌ Error fetching tickers: {}", e);
            println!("🔄 Falling back to default symbols...");
            
            // Fallback to default analysis
            let mut analyzer = StockAnalyzer::new();
            let symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA"];
            let end = Utc::now();
            let start = end - Duration::days(100);
            
            for symbol in symbols {
                println!("\n{}", "=".repeat(50));
                
                match analyzer.fetch_stock_data(symbol, start, end).await {
                    Ok(stock_data) => {
                        if stock_data.is_empty() {
                            println!("No data available for {}", symbol);
                            continue;
                        }
                        
                        let indicators = analyzer.calculate_indicators(symbol, &stock_data);
                        analyzer.print_analysis(symbol, &stock_data, &indicators);
                        
                        match analyzer.get_latest_quote(symbol).await {
                            Ok(latest) => {
                                println!("\nLatest Quote Update:");
                                println!("  Time: {}", latest.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
                                println!("  Price: ${:.2}", latest.close);
                            }
                            Err(e) => println!("Failed to get latest quote: {}", e),
                        }
                    }
                    Err(e) => {
                        println!("Failed to fetch data for {}: {}", symbol, e);
                    }
                }
            }
        }
    }
    
    println!("\n{}", "=".repeat(60));
    println!("✨ Analysis complete!");
    println!("\n💡 Try these examples:");
    println!("  cargo run --example ticker_collection  # Explore all available tickers");
    println!("  cargo run --example simple_analysis    # Analyze a single stock");
    
    Ok(())
}
