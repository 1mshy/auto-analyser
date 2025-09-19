use anyhow::Result;
use auto_analyser::{StockAnalyzer, StockFilter};

#[tokio::main]
async fn main() -> Result<()> {
    println!("💼 Stock Filtering Examples - Working Configurations");
    println!("{}", "=".repeat(70));

    // Example 1: Large Cap Value Stocks
    println!("\n💎 Configuration 1: Large Cap Value Stocks");
    let value_filter = StockFilter::new()
        .with_market_cap_range(Some(10_000_000_000.0), None)     // $10B+ market cap
        .with_price_range(Some(20.0), Some(500.0))               // $20-$500 price range
        .with_pct_change_range(Some(-10.0), Some(-1.0))          // Recent decline (potential value)
        .with_rsi_thresholds(Some(25.0), Some(40.0));            // Oversold conditions

    test_filter_quickly(&value_filter, "Large Cap Value").await?;

    // Example 2: Growth Momentum Stocks
    println!("\n🚀 Configuration 2: Growth Momentum Stocks");
    let momentum_filter = StockFilter::new()
        .with_market_cap_range(Some(1_000_000_000.0), Some(100_000_000_000.0)) // $1B-$100B
        .with_price_range(Some(10.0), Some(300.0))                              // $10-$300
        .with_pct_change_range(Some(2.0), Some(15.0))                           // Recent gains
        .with_rsi_range(Some(40.0), Some(65.0));                               // Not overbought

    test_filter_quickly(&momentum_filter, "Growth Momentum").await?;

    // Example 3: Small to Mid Cap Opportunities
    println!("\n🎯 Configuration 3: Small to Mid Cap Opportunities");
    let small_mid_filter = StockFilter::new()
        .with_market_cap_range(Some(300_000_000.0), Some(10_000_000_000.0))    // $300M-$10B
        .with_price_range(Some(5.0), Some(150.0))                              // $5-$150
        .with_pct_change_range(Some(-8.0), Some(12.0));                        // Moderate volatility

    test_filter_quickly(&small_mid_filter, "Small to Mid Cap").await?;

    // Example 4: Conservative Dividend-Style Stocks
    println!("\n🛡️  Configuration 4: Conservative Large Stocks");
    let conservative_filter = StockFilter::new()
        .with_market_cap_range(Some(50_000_000_000.0), None)     // $50B+ (mega cap)
        .with_price_range(Some(50.0), Some(800.0))               // $50-$800
        .with_pct_change_range(Some(-3.0), Some(3.0));           // Low volatility

    test_filter_quickly(&conservative_filter, "Conservative Large").await?;

    // Example 5: Oversold Recovery Candidates
    println!("\n🔄 Configuration 5: Oversold Recovery Candidates");
    let recovery_filter = StockFilter::new()
        .with_market_cap_range(Some(2_000_000_000.0), Some(200_000_000_000.0)) // $2B-$200B
        .with_price_range(Some(15.0), Some(400.0))                             // $15-$400
        .with_pct_change_range(Some(-25.0), Some(-5.0))                        // Significant decline
        .with_rsi_thresholds(Some(20.0), Some(35.0));                         // Deeply oversold

    test_filter_quickly(&recovery_filter, "Oversold Recovery").await?;

    println!("\n{}", "=".repeat(70));
    println!("📝 How to Use These Configurations:");
    println!("   1. Copy any configuration from above");
    println!("   2. Replace the `create_custom_filter()` function in src/main.rs");
    println!("   3. Run `cargo run` to analyze stocks with your new filter");
    println!("\n⚡ Pro Tips:");
    println!("   • Start with broader filters and narrow down based on results");
    println!("   • Consider current market conditions when setting ranges");
    println!("   • Combine multiple criteria for more sophisticated screening");
    println!("   • The RSI threshold filters are particularly powerful for timing");

    Ok(())
}

async fn test_filter_quickly(filter: &StockFilter, name: &str) -> Result<()> {
    match StockAnalyzer::fetch_n_tickers(100).await {
        Ok(tickers) => {
            let filtered = StockAnalyzer::filter_tickers(&tickers, filter);
            println!("   📊 Filter effectiveness: {} out of {} tickers match", filtered.len(), tickers.len());
            
            if !filtered.is_empty() {
                println!("   🎯 Sample matching stocks:");
                for ticker in filtered.iter().take(5) {
                    let market_cap = ticker.market_cap.as_deref().unwrap_or("N/A");
                    let price = ticker.last_sale.as_deref().unwrap_or("N/A");
                    let change = ticker.pct_change.as_deref().unwrap_or("N/A");
                    println!("      • {} - ${} ({}% change, {} market cap)", 
                            ticker.symbol, price, change, market_cap);
                }
                if filtered.len() > 5 {
                    println!("      ... and {} more stocks", filtered.len() - 5);
                }
            } else {
                println!("   ⚠️  No stocks currently match this filter - consider adjusting criteria");
            }
        }
        Err(e) => {
            println!("   ❌ Error fetching tickers: {}", e);
        }
    }
    println!();
    Ok(())
}
