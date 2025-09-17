/// # Stock Data Analysis Example
/// 
/// This example demonstrates how to use the auto-analyser library to:
/// 1. Fetch historical stock data from Yahoo Finance
/// 2. Calculate technical indicators (SMA, RSI, MACD)
/// 3. Generate trading signals
/// 
/// Run this example with: `cargo run --example simple_analysis`

use auto_analyser::StockAnalyzer;
use chrono::{Utc, Duration};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Auto Stock Analyser - Simple Analysis Example");
    println!("{}", "=".repeat(60));
    
    let mut analyzer = StockAnalyzer::new();
    
    // Analyze Apple stock with 30 days of historical data
    let symbol = "AAPL";
    let end = Utc::now();
    let start = end - Duration::days(30);
    
    println!("📊 Fetching {} days of data for {}...", 30, symbol);
    
    match analyzer.fetch_stock_data(symbol, start, end).await {
        Ok(stock_data) => {
            if stock_data.is_empty() {
                println!("❌ No data available for {}", symbol);
                return Ok(());
            }
            
            println!("✅ Retrieved {} data points", stock_data.len());
            
            // Calculate technical indicators
            let indicators = analyzer.calculate_indicators(symbol, &stock_data);
            
            // Print detailed analysis
            analyzer.print_analysis(symbol, &stock_data, &indicators);
            
            // Show trend over last 5 days
            println!("\n📈 Last 5 Trading Days Trend:");
            println!("{}", "-".repeat(40));
            
            let len = stock_data.len();
            let start_idx = if len >= 5 { len - 5 } else { 0 };
            
            for i in start_idx..len {
                let data = &stock_data[i];
                let indicator = &indicators[i];
                
                let trend = if i > 0 {
                    let prev_close = stock_data[i-1].close;
                    if data.close > prev_close {
                        "🟢 ↗"
                    } else if data.close < prev_close {
                        "🔴 ↘"
                    } else {
                        "🟡 →"
                    }
                } else {
                    "   "
                };
                
                println!("{} {} | ${:.2} | RSI: {:.1}", 
                         data.timestamp.format("%m-%d"), 
                         trend,
                         data.close,
                         indicator.rsi.unwrap_or(0.0));
            }
            
            // Get the latest real-time quote
            println!("\n⚡ Fetching latest quote...");
            match analyzer.get_latest_quote(symbol).await {
                Ok(latest) => {
                    println!("Latest: ${:.2} at {}", 
                             latest.close, 
                             latest.timestamp.format("%Y-%m-%d %H:%M UTC"));
                }
                Err(e) => println!("Failed to get latest quote: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Error fetching data: {}", e);
            println!("💡 This might be due to network issues or invalid symbol");
        }
    }
    
    println!("\n{}", "=".repeat(60));
    println!("✨ Analysis complete! Try modifying the symbol or date range.");
    
    Ok(())
}
