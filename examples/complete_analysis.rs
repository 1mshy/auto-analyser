use anyhow::Result;
/// # Complete Stock Analysis Pipeline
///
/// This example demonstrates the full power of the auto-analyser:
/// 1. Fetch all tickers from Nasdaq
/// 2. Filter and find the best opportunities
/// 3. Perform technical analysis on selected stocks
/// 4. Generate comprehensive reports
///
/// Run this example with: `cargo run --example complete_analysis`
use auto_analyser::StockAnalyzer;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Auto Stock Analyser - Complete Analysis Pipeline");
    println!("{}", "=".repeat(70));

    // Step 1: Fetch all available tickers
    println!("📡 Step 1: Fetching all tickers from Nasdaq API...");
    let all_tickers = match StockAnalyzer::fetch_all_tickers().await {
        Ok(tickers) => {
            println!("✅ Fetched {} tickers successfully!", tickers.len());
            tickers
        }
        Err(e) => {
            println!("❌ Error fetching tickers: {}", e);
            return Ok(());
        }
    };

    // Step 2: Find investment opportunities
    println!("\n🔍 Step 2: Identifying investment opportunities...");

    // Get today's top performers
    let top_performers = StockAnalyzer::get_top_performers(&all_tickers, 20);
    println!("📈 Found {} top performers today", top_performers.len());

    // Get large cap stocks (for stability)
    let large_cap_stocks = StockAnalyzer::filter_tickers(
        &all_tickers,
        None,
        Some(10_000_000_000.0), // $10B+ market cap
        None,
    );
    println!(
        "🏢 Found {} large-cap stocks (>$10B)",
        large_cap_stocks.len()
    );

    // Display findings
    StockAnalyzer::print_tickers(
        &top_performers[..10.min(top_performers.len())],
        "🚀 Today's Top 10 Performers",
    );

    let top_large_cap = StockAnalyzer::get_top_performers(&large_cap_stocks, 10);
    StockAnalyzer::print_tickers(&top_large_cap, "🏢 Top Large-Cap Performers");

    // Step 3: Select stocks for detailed analysis
    println!("\n🎯 Step 3: Selecting stocks for technical analysis...");

    let mut analysis_candidates: Vec<String> = Vec::new();

    // Add top 2 performers (high risk, high reward)
    analysis_candidates.extend(top_performers.iter().take(2).map(|t| t.symbol.clone()));

    // Add top 2 large-cap performers (stable growth)
    analysis_candidates.extend(top_large_cap.iter().take(2).map(|t| t.symbol.clone()));

    // Add some blue-chip stocks for comparison
    analysis_candidates.extend([
        "AAPL".to_string(), // Apple
        "MSFT".to_string(), // Microsoft
    ]);

    // Remove duplicates
    analysis_candidates.sort();
    analysis_candidates.dedup();

    println!(
        "📊 Selected {} stocks for analysis: {:?}",
        analysis_candidates.len(),
        analysis_candidates
    );

    // Step 4: Perform technical analysis
    println!("\n📈 Step 4: Performing technical analysis...");

    let mut analyzer = StockAnalyzer::new();
    let end = Utc::now();
    let start = end - Duration::days(60); // 60 days of data

    let mut analysis_results = Vec::new();

    for (i, symbol) in analysis_candidates.iter().enumerate() {
        println!(
            "\n{} Analyzing {} ({}/{}) {}",
            "=".repeat(20),
            symbol,
            i + 1,
            analysis_candidates.len(),
            "=".repeat(20)
        );
        match analyzer.fetch_stock_data(symbol, start, end).await {
            Ok(stock_data) => {
                if stock_data.is_empty() {
                    println!("❌ No data available for {}", symbol);
                    continue;
                }

                let indicators = analyzer.calculate_indicators(symbol, &stock_data);

                // Print detailed analysis
                analyzer.print_analysis(symbol, &stock_data, &indicators);

                // Store results for summary - clone what we need before references go out of scope
                if !stock_data.is_empty() && !indicators.is_empty() {
                    let latest_data = stock_data[stock_data.len() - 1].clone();
                    let latest_indicators = indicators[indicators.len() - 1].clone();
                    analysis_results.push((symbol.clone(), latest_data, latest_indicators));
                }

                // Get latest quote
                match analyzer.get_latest_quote(symbol).await {
                    Ok(latest) => {
                        println!("\n⚡ Real-time Update:");
                        println!("  Current Price: ${:.2}", latest.close);
                        println!("  Volume: {}", latest.volume);
                        println!("  Last Updated: {}", latest.timestamp.format("%H:%M UTC"));

                        // Calculate price change from historical
                        if let Some(last_historical) = stock_data.last() {
                            let change = latest.close - last_historical.close;
                            let change_pct = (change / last_historical.close) * 100.0;
                            println!("  Intraday Change: ${:.2} ({:+.2}%)", change, change_pct);
                        }
                    }
                    Err(e) => println!("Failed to get latest quote: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Failed to fetch data for {}: {}", symbol, e);
            }
        }

        // Add delay to be respectful to APIs
        if i < analysis_candidates.len() - 1 {
            println!("\n⏳ Waiting 2 seconds before next analysis...");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

    // Step 5: Generate summary report
    println!("\n{}", "=".repeat(70));
    println!("📊 FINAL INVESTMENT SUMMARY REPORT");
    println!("{}", "=".repeat(70));

    if !analysis_results.is_empty() {
        println!("\n🎯 Analyzed Stocks Overview:");
        println!(
            "{:<8} {:<12} {:<8} {:<8} {:<12}",
            "Symbol", "Price", "RSI", "Signal", "Recommendation"
        );
        println!("{}", "-".repeat(60));

        for (symbol, data, indicators) in &analysis_results {
            let rsi = indicators.rsi.unwrap_or(50.0);
            let price = data.close;

            let signal = if rsi > 70.0 {
                "Overbought"
            } else if rsi < 30.0 {
                "Oversold"
            } else if rsi > 60.0 {
                "Strong"
            } else if rsi < 40.0 {
                "Weak"
            } else {
                "Neutral"
            };

            let recommendation = match signal {
                "Oversold" => "🟢 BUY",
                "Weak" => "🟡 WATCH",
                "Neutral" => "⚪ HOLD",
                "Strong" => "🟡 WATCH",
                "Overbought" => "🔴 SELL",
                _ => "❓ UNKNOWN",
            };

            println!(
                "{:<8} ${:<11.2} {:<8.1} {:<8} {:<12}",
                symbol, price, rsi, signal, recommendation
            );
        }

        println!("\n💡 Investment Insights:");
        let oversold_count = analysis_results
            .iter()
            .filter(|(_, _, indicators)| indicators.rsi.unwrap_or(50.0) < 30.0)
            .count();
        let overbought_count = analysis_results
            .iter()
            .filter(|(_, _, indicators)| indicators.rsi.unwrap_or(50.0) > 70.0)
            .count();

        println!(
            "   • {} stocks appear oversold (potential buying opportunities)",
            oversold_count
        );
        println!(
            "   • {} stocks appear overbought (consider taking profits)",
            overbought_count
        );
        println!("   • {} stocks analyzed in total", analysis_results.len());

        // Find best opportunities
        let best_opportunities: Vec<_> = analysis_results
            .iter()
            .filter(|(_, _, indicators)| {
                let rsi = indicators.rsi.unwrap_or(50.0);
                rsi < 40.0 && rsi > 20.0 // Good buying range
            })
            .collect();

        if !best_opportunities.is_empty() {
            println!("\n🎯 Best Investment Opportunities:");
            for (symbol, data, _) in best_opportunities.iter().take(3) {
                println!(
                    "   • {} at ${:.2} - Undervalued with good fundamentals",
                    symbol, data.close
                );
            }
        }
    } else {
        println!("❌ No analysis results available");
    }

    println!("\n{}", "=".repeat(70));
    println!("✨ Complete analysis finished!");
    println!("⚠️  DISCLAIMER: This is for educational purposes only.");
    println!("💰 Always do your own research before investing.");
    println!("{}", "=".repeat(70));

    Ok(())
}
