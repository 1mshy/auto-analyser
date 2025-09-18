use auto_analyser::StockAnalyzer;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut analyzer = StockAnalyzer::new();

    // Test with a known stock to compare RSI values
    println!("Testing RSI calculation with Apple (AAPL)...");
    
    // Fetch recent data (last 30 days should be enough for RSI calculation)
    let end_date = Utc::now();
    let start_date = end_date - chrono::Duration::days(30);
    
    match analyzer.fetch_stock_data("AAPL", start_date, end_date).await {
        Ok(stock_data) => {
            println!("Fetched {} data points", stock_data.len());
            
            // Calculate indicators
            let indicators = analyzer.calculate_indicators("AAPL", &stock_data);
            
            // Show the last 5 RSI values
            println!("\nLast 5 RSI values:");
            for (data, indicator) in stock_data.iter().zip(indicators.iter()).rev().take(5).collect::<Vec<_>>().iter().rev() {
                if let Some(rsi) = indicator.rsi {
                    println!(
                        "Date: {}, Close: ${:.2}, RSI: {:.2}",
                        data.timestamp.format("%Y-%m-%d"),
                        data.close,
                        rsi
                    );
                }
            }
            
            // Show latest analysis
            if let (Some(latest_data), Some(latest_indicators)) = (stock_data.last(), indicators.last()) {
                println!("\n=== Latest Analysis ===");
                println!("Date: {}", latest_data.timestamp.format("%Y-%m-%d"));
                println!("Close: ${:.2}", latest_data.close);
                if let Some(rsi) = latest_indicators.rsi {
                    println!("RSI(14): {:.2}", rsi);
                    
                    // Provide interpretation
                    if rsi > 70.0 {
                        println!("⚠️  RSI indicates overbought conditions");
                    } else if rsi < 30.0 {
                        println!("📈 RSI indicates oversold conditions");
                    } else {
                        println!("✅ RSI is in neutral range");
                    }
                }
            }
        }
        Err(e) => {
            println!("Error fetching data: {}", e);
        }
    }

    Ok(())
}
