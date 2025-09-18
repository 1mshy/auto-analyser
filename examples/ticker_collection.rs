use anyhow::Result;
/// # Ticker Collection Example
///
/// This example demonstrates how to:
/// 1. Fetch all available tickers from Nasdaq API
/// 2. Filter tickers by sector, market cap, and country
/// 3. Find top performing stocks
/// 4. Display formatted ticker information
///
/// Run this example with: `cargo run --example ticker_collection`
use auto_analyser::{StockAnalyzer, TickerInfo};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Auto Stock Analyser - Ticker Collection Example");
    println!("{}", "=".repeat(60));

    // Fetch all tickers from Nasdaq API
    println!("📡 Fetching all tickers from Nasdaq API...");
    match StockAnalyzer::fetch_all_tickers().await {
        Ok(all_tickers) => {
            println!("✅ Successfully fetched {} tickers!", all_tickers.len());

            // Show some basic statistics
            let sectors: std::collections::HashSet<String> = all_tickers
                .iter()
                .filter_map(|t| t.sector.clone())
                .collect();

            println!("📊 Found {} unique sectors", sectors.len());

            // 1. Show top 10 performing stocks
            let top_performers = StockAnalyzer::get_top_performers(&all_tickers, 10);
            StockAnalyzer::print_tickers(&top_performers, "🚀 Top 10 Performing Stocks Today");

            // 2. Filter by Technology sector
            let tech_stocks =
                StockAnalyzer::filter_tickers(&all_tickers, Some("Technology"), None, None);
            println!("\n💻 Found {} Technology stocks", tech_stocks.len());

            // Show top 10 tech performers
            let top_tech = StockAnalyzer::get_top_performers(&tech_stocks, 10);
            StockAnalyzer::print_tickers(&top_tech, "💻 Top Technology Performers");

            // 3. Filter by large market cap (> $1B)
            let large_cap_stocks = StockAnalyzer::filter_tickers(
                &all_tickers,
                None,
                Some(1_000_000_000.0), // $1B minimum
                None,
            );
            println!(
                "\n🏢 Found {} large-cap stocks (>$1B market cap)",
                large_cap_stocks.len()
            );

            // Show top 10 large cap performers
            let top_large_cap = StockAnalyzer::get_top_performers(&large_cap_stocks, 10);
            StockAnalyzer::print_tickers(&top_large_cap, "🏢 Top Large-Cap Performers");

            // 4. Filter by Healthcare sector with large market cap
            let healthcare_large_cap = StockAnalyzer::filter_tickers(
                &all_tickers,
                Some("Health Care"),
                Some(1_000_000_000.0),
                None,
            );
            println!(
                "\n🏥 Found {} large-cap Healthcare stocks",
                healthcare_large_cap.len()
            );

            if !healthcare_large_cap.is_empty() {
                let top_healthcare = StockAnalyzer::get_top_performers(&healthcare_large_cap, 5);
                StockAnalyzer::print_tickers(
                    &top_healthcare,
                    "🏥 Top Healthcare Large-Cap Performers",
                );
            }

            // 5. Show some sector distribution
            println!("\n📈 Sector Distribution (Top 10):");
            println!("{}", "=".repeat(40));

            let mut sector_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            for ticker in &all_tickers {
                if let Some(sector) = &ticker.sector {
                    *sector_counts.entry(sector.clone()).or_insert(0) += 1;
                }
            }

            let mut sector_vec: Vec<_> = sector_counts.into_iter().collect();
            sector_vec.sort_by(|a, b| b.1.cmp(&a.1));

            for (sector, count) in sector_vec.iter().take(10) {
                println!("{:<30} {:>6} stocks", sector, count);
            }

            // 6. Sample some specific high-value tickers for analysis
            let sample_symbols = ["AAPL", "GOOGL", "MSFT", "NVDA", "TSLA"];
            let sample_tickers: Vec<TickerInfo> = all_tickers
                .into_iter()
                .filter(|t| sample_symbols.contains(&t.symbol.as_str()))
                .collect();

            if !sample_tickers.is_empty() {
                StockAnalyzer::print_tickers(
                    &sample_tickers,
                    "🎯 Sample High-Value Stocks for Analysis",
                );

                println!("\n💡 Tip: You can now use these symbols with the main analyzer:");
                for ticker in &sample_tickers {
                    println!("   cargo run -- {}", ticker.symbol);
                }
            }
        }
        Err(e) => {
            println!("❌ Error fetching tickers: {}", e);
            println!("💡 This might be due to:");
            println!("   - Network connectivity issues");
            println!("   - Nasdaq API rate limiting");
            println!("   - API endpoint changes");
            println!("\n🔄 Try running the command again in a few minutes.");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("✨ Ticker collection complete!");

    Ok(())
}
