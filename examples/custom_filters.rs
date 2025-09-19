use anyhow::Result;
use auto_analyser::{StockAnalyzer, StockFilter};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎛️  Stock Filter Configuration Examples");
    println!("{}", "=".repeat(60));

    // Example 1: Conservative Large Cap Filter
    println!("\n📊 Example 1: Conservative Large Cap Stocks");
    let conservative_filter = StockFilter::new()
        .with_market_cap_range(Some(10_000_000_000.0), None) // $10B+ market cap
        .with_price_range(Some(50.0), Some(300.0))           // $50-$300 price range
        .with_volume_range(Some(1_000_000), None)            // 1M+ volume
        .with_pct_change_range(Some(-5.0), Some(5.0))        // Max 5% daily change
        .with_countries(vec!["United States".to_string()])
        .with_sectors(vec![
            "Technology".to_string(),
            "Healthcare".to_string(),
            "Financial".to_string(),
        ]);

    demo_filter(&conservative_filter, "Conservative Large Cap").await?;

    // Example 2: Growth Stock Filter
    println!("\n📈 Example 2: High Growth Potential Stocks");
    let growth_filter = StockFilter::new()
        .with_market_cap_range(Some(500_000_000.0), Some(20_000_000_000.0)) // $500M-$20B
        .with_price_range(Some(10.0), Some(200.0))                          // $10-$200
        .with_volume_range(Some(500_000), None)                             // 500K+ volume
        .with_pct_change_range(Some(2.0), Some(20.0))                       // 2-20% daily gain
        .with_ipo_year_range(Some(2015), None)                              // Recent IPOs
        .with_sectors(vec![
            "Technology".to_string(),
            "Healthcare".to_string(),
            "Consumer".to_string(),
        ]);

    demo_filter(&growth_filter, "High Growth Potential").await?;

    // Example 3: Value Stock Filter (Oversold)
    println!("\n💎 Example 3: Value/Oversold Stocks");
    let value_filter = StockFilter::new()
        .with_market_cap_range(Some(1_000_000_000.0), None)    // $1B+ market cap
        .with_price_range(Some(5.0), Some(100.0))              // $5-$100
        .with_volume_range(Some(300_000), None)                 // 300K+ volume
        .with_pct_change_range(Some(-15.0), Some(-2.0))         // Recent decline
        .with_rsi_thresholds(Some(25.0), Some(35.0))           // Oversold RSI
        .with_countries(vec!["United States".to_string()]);

    demo_filter(&value_filter, "Value/Oversold Stocks").await?;

    // Example 4: Small Cap Energy Filter
    println!("\n⚡ Example 4: Small Cap Energy Stocks");
    let energy_filter = StockFilter::new()
        .with_market_cap_range(Some(100_000_000.0), Some(5_000_000_000.0)) // $100M-$5B
        .with_price_range(Some(1.0), Some(50.0))                           // $1-$50
        .with_volume_range(Some(200_000), None)                             // 200K+ volume
        .with_sectors(vec!["Energy".to_string()])
        .with_countries(vec!["United States".to_string(), "Canada".to_string()]);

    demo_filter(&energy_filter, "Small Cap Energy").await?;

    // Example 5: Custom Momentum Filter
    println!("\n🚀 Example 5: Momentum Stocks");
    let momentum_filter = StockFilter::new()
        .with_market_cap_range(Some(2_000_000_000.0), Some(50_000_000_000.0)) // $2B-$50B
        .with_price_range(Some(20.0), None)                                    // $20+
        .with_volume_range(Some(1_000_000), None)                             // 1M+ volume
        .with_pct_change_range(Some(5.0), Some(25.0))                          // Strong daily gains
        .with_rsi_range(Some(45.0), Some(65.0))                               // Not oversold/overbought
        .with_sectors(vec![
            "Technology".to_string(),
            "Consumer".to_string(),
            "Communication".to_string(),
        ]);

    demo_filter(&momentum_filter, "Momentum Stocks").await?;

    println!("\n{}", "=".repeat(60));
    println!("💡 Tips for Creating Your Own Filters:");
    println!("   • Start with broader criteria and narrow down");
    println!("   • Consider market conditions when setting ranges");
    println!("   • Test different combinations for your strategy");
    println!("   • Monitor performance and adjust accordingly");
    println!("\n🔧 To use a filter, copy the configuration to your main.rs file!");

    Ok(())
}

async fn demo_filter(filter: &StockFilter, name: &str) -> Result<()> {
    println!("Filter: {}", name);
    
    // Fetch a small sample to demonstrate
    match StockAnalyzer::fetch_n_tickers(100).await {
        Ok(tickers) => {
            let filtered = StockAnalyzer::filter_tickers(&tickers, filter);
            println!("   📊 {} out of {} tickers match this filter", filtered.len(), tickers.len());
            
            if !filtered.is_empty() {
                println!("   📈 Sample matches:");
                for ticker in filtered.iter().take(5) {
                    println!("      • {} ({})", ticker.symbol, ticker.name.chars().take(25).collect::<String>());
                }
                if filtered.len() > 5 {
                    println!("      ... and {} more", filtered.len() - 5);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Error fetching tickers: {}", e);
        }
    }

    Ok(())
}
