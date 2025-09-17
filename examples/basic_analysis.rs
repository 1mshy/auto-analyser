use auto_analyser::StockAnalyzer;
use chrono::{Utc, Duration};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut analyzer = StockAnalyzer::new();
    
    // Analyze a single stock with 30 days of data
    let symbol = "AAPL";
    let end = Utc::now();
    let start = end - Duration::days(30);
    
    println!("Fetching 30 days of data for {}...", symbol);
    
    match analyzer.fetch_stock_data(symbol, start, end).await {
        Ok(stock_data) => {
            if stock_data.is_empty() {
                println!("No data available for {}", symbol);
                return Ok(());
            }
            
            println!("Retrieved {} data points", stock_data.len());
            
            let indicators = analyzer.calculate_indicators(symbol, &stock_data);
            analyzer.print_analysis(symbol, &stock_data, &indicators);
            
            // Show the last 5 data points with indicators
            println!("\n=== Last 5 Trading Days ===");
            let len = stock_data.len();
            let start_idx = if len >= 5 { len - 5 } else { 0 };
            
            for i in start_idx..len {
                let data = &stock_data[i];
                let indicator = &indicators[i];
                
                println!("\n{}: ${:.2}", 
                         data.timestamp.format("%Y-%m-%d"), 
                         data.close);
                
                if let Some(sma_20) = indicator.sma_20 {
                    println!("  SMA(20): ${:.2}", sma_20);
                }
                if let Some(rsi) = indicator.rsi {
                    println!("  RSI: {:.1}", rsi);
                }
            }
        }
        Err(e) => {
            println!("Error fetching data: {}", e);
        }
    }
    
    Ok(())
}
