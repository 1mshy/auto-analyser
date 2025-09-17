use auto_analyser::StockAnalyzer;
use chrono::{Utc, Duration};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut analyzer = StockAnalyzer::new();
    
    // Example symbols to analyze
    let symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA"];
    
    // Date range for historical data (last 100 days)
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
                
                // Example: Get latest quote
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
    
    println!("\n{}", "=".repeat(50));
    println!("Analysis complete!");
    
    Ok(())
}
