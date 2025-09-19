use anyhow::Result;
use auto_analyser::{StockAnalyzer, StockFilter};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Debug: Analyzing Ticker Data Structure");
    println!("{}", "=".repeat(60));

    match StockAnalyzer::fetch_all_tickers().await {
        Ok(tickers) => {
            println!("✅ Fetched {} tickers successfully!", tickers.len());
            
            println!("\n📊 Sample ticker data:");
            for (i, ticker) in tickers.iter().take(10).enumerate() {
                println!("\n{}. Symbol: {}", i + 1, ticker.symbol);
                println!("   Name: {}", ticker.name);
                println!("   Last Sale: {:?}", ticker.last_sale);
                println!("   Net Change: {:?}", ticker.net_change);
                println!("   Pct Change: {:?}", ticker.pct_change);
                println!("   Market Cap: {:?}", ticker.market_cap);
                println!("   Country: {:?}", ticker.country);
                println!("   IPO Year: {:?}", ticker.ipo_year);
                println!("   Volume: {:?}", ticker.volume);
                println!("   Sector: {:?}", ticker.sector);
                println!("   Industry: {:?}", ticker.industry);
            }

            // Analyze country distribution
            println!("\n🌍 Country distribution:");
            let mut country_counts = std::collections::HashMap::new();
            for ticker in &tickers {
                let country = ticker.country.as_deref().unwrap_or("Unknown");
                *country_counts.entry(country).or_insert(0) += 1;
            }
            
            for (country, count) in country_counts {
                println!("   {}: {} stocks", country, count);
            }

            // Test a very lenient filter
            println!("\n🧪 Testing very lenient filter:");
            let lenient_filter = StockFilter::new()
                .with_price_range(Some(0.01), Some(10000.0));
            
            let filtered = StockAnalyzer::filter_tickers(&tickers, &lenient_filter);
            println!("   {} out of {} tickers passed the lenient filter", filtered.len(), tickers.len());

            if !filtered.is_empty() {
                println!("   Sample filtered tickers:");
                for ticker in filtered.iter().take(5) {
                    println!("      • {} ({})", ticker.symbol, ticker.name.chars().take(30).collect::<String>());
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching tickers: {}", e);
        }
    }

    Ok(())
}
