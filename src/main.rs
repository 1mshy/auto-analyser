use anyhow::Result;
use auto_analyser::{StockAnalyzer, StockFilter};
use priority_queue::PriorityQueue;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Auto Stock Analyser - Enhanced with Customizable Filtering");
    println!("{}", "=".repeat(70));

    let mut prior: PriorityQueue<String, i32> = PriorityQueue::new();
    let mut analyser = StockAnalyzer::new();

    // Create customizable filters
    let filter = create_custom_filter();
    print_filter_settings(&filter);

    println!("📡 Fetching available tickers from Nasdaq...");
    match StockAnalyzer::fetch_n_tickers(500).await {
        Ok(all_tickers) => {
            println!("✅ Fetched {} tickers successfully!", all_tickers.len());
            
            // Apply filters to get qualifying tickers
            let filtered_tickers = StockAnalyzer::filter_tickers(&all_tickers, &filter);
            println!("🔍 {} tickers passed initial filters", filtered_tickers.len());

            if filtered_tickers.is_empty() {
                println!("❌ No tickers match your criteria. Consider relaxing your filters.");
                return Ok(());
            }

            // Add high priority tickers to queue
            let top_performers = StockAnalyzer::get_top_performers(&filtered_tickers, 50);
            for ticker in &top_performers {
                prior.push(ticker.symbol.clone(), 10);
            }

            // Add remaining filtered tickers with lower priority
            for ticker in &filtered_tickers {
                if !top_performers.iter().any(|t| t.symbol == ticker.symbol) {
                    prior.push(ticker.symbol.clone(), 5);
                }
            }

            println!("🎯 Analyzing {} prioritized stocks...", prior.len());
            let mut analyzed_count = 0;
            let mut found_opportunities = 0;

            while let Some((ticker, priority)) = prior.pop() {
                analyzed_count += 1;
                
                if analyzed_count % 10 == 0 {
                    println!("📊 Analyzed {}/{} stocks...", analyzed_count, filtered_tickers.len());
                }

                let stock_data = match analyser.fetch_all_stock_data(&ticker).await {
                    Ok(data) => data,
                    Err(e) => {
                        println!("⚠️  Failed to fetch data for {}: {}", ticker, e);
                        continue;
                    }
                };

                if stock_data.is_empty() {
                    continue;
                }

                let ticker_indicators = analyser.calculate_indicators(&ticker, &stock_data);
                
                if let Some(current_indicator) = ticker_indicators.last() {
                    if let Some(current_rsi) = current_indicator.rsi {
                        // Check if stock meets our opportunity criteria
                        if is_opportunity(&ticker, current_rsi, &stock_data, &filter) {
                            found_opportunities += 1;
                            print_opportunity(&ticker, current_rsi, &stock_data, current_indicator, priority);
                        }
                    }
                } else {
                    continue;
                }

                // Limit analysis to prevent rate limiting
                if analyzed_count >= 100 {
                    println!("⏱️  Reached analysis limit to prevent rate limiting");
                    break;
                }
            }

            println!("\n{}", "=".repeat(70));
            println!("✨ Analysis complete!");
            println!("📈 Found {} investment opportunities out of {} analyzed stocks", found_opportunities, analyzed_count);
        }
        Err(e) => {
            println!("❌ Error fetching tickers: {}", e);
        }
    }

    println!("\n� Tip: Modify the `create_custom_filter()` function in main.rs to adjust your filtering criteria!");
    
    Ok(())
}

fn create_custom_filter() -> StockFilter {
    StockFilter::new()
        // Market cap range: $1B to $500B (based on available data)
        .with_market_cap_range(Some(1_000_000_000.0), Some(500_000_000_000.0))
        
        // Price range: $5 to $1000 (reasonable stock prices)
        .with_price_range(Some(5.0), Some(1000.0))
        
        // Percentage change: look for stocks with some movement
        .with_pct_change_range(Some(-20.0), Some(20.0))
        
        // RSI thresholds for identifying oversold conditions
        .with_rsi_thresholds(Some(30.0), Some(70.0))
}

fn print_filter_settings(filter: &StockFilter) {
    println!("🎛️  Current Filter Settings:");
    println!("{}", "-".repeat(50));
    
    if let (Some(min), Some(max)) = (filter.min_market_cap, filter.max_market_cap) {
        println!("💰 Market Cap: ${:.1}M - ${:.1}B", min / 1_000_000.0, max / 1_000_000_000.0);
    }
    
    if let (Some(min), Some(max)) = (filter.min_price, filter.max_price) {
        println!("💵 Price Range: ${:.2} - ${:.2}", min, max);
    }
    
    if let Some(min) = filter.min_volume {
        println!("📊 Min Volume: {} shares", format_number(min as f64));
    }
    
    if let (Some(min), Some(max)) = (filter.min_pct_change, filter.max_pct_change) {
        println!("📈 % Change Range: {:.1}% to {:.1}%", min, max);
    }
    
    if let Some(sectors) = &filter.sectors {
        println!("🏢 Sectors: {}", sectors.join(", "));
    }
    
    if let Some(countries) = &filter.countries {
        println!("🌎 Countries: {}", countries.join(", "));
    }
    
    if let Some(min_year) = filter.min_ipo_year {
        println!("🎂 IPO Year: {} or later", min_year);
    }
    
    if let (Some(oversold), Some(overbought)) = (filter.oversold_rsi_threshold, filter.overbought_rsi_threshold) {
        println!("📉 RSI Thresholds: Oversold < {}, Overbought > {}", oversold, overbought);
    }
    
    println!("{}", "-".repeat(50));
}

fn is_opportunity(_ticker: &str, rsi: f64, stock_data: &[auto_analyser::StockData], filter: &StockFilter) -> bool {
    // Check if RSI indicates oversold condition
    if let Some(oversold_threshold) = filter.oversold_rsi_threshold {
        if rsi < oversold_threshold {
            return true;
        }
    }

    // Check for recent strong performance (price increase over last week)
    if stock_data.len() >= 7 {
        let recent_data = &stock_data[stock_data.len() - 7..];
        let start_price = recent_data[0].close;
        let end_price = recent_data[recent_data.len() - 1].close;
        let week_change = ((end_price - start_price) / start_price) * 100.0;
        
        if week_change > 5.0 && rsi < 60.0 {  // Strong recent performance but not overbought
            return true;
        }
    }

    false
}

fn print_opportunity(
    ticker: &str, 
    rsi: f64, 
    stock_data: &[auto_analyser::StockData], 
    indicators: &auto_analyser::TechnicalIndicators,
    priority: i32
) {
    let latest_data = stock_data.last().unwrap();
    
    println!("\n🎯 OPPORTUNITY FOUND: {}", ticker);
    println!("   💰 Current Price: ${:.2}", latest_data.close);
    println!("   📉 RSI: {:.2}", rsi);
    println!("   📊 Volume: {}", format_number(latest_data.volume as f64));
    
    if let Some(sma_20) = indicators.sma_20 {
        println!("   📈 SMA(20): ${:.2}", sma_20);
    }
    
    if let Some(sma_50) = indicators.sma_50 {
        println!("   📈 SMA(50): ${:.2}", sma_50);
    }
    
    // Calculate recent performance
    if stock_data.len() >= 7 {
        let week_ago = &stock_data[stock_data.len() - 7];
        let week_change = ((latest_data.close - week_ago.close) / week_ago.close) * 100.0;
        println!("   📅 7-Day Change: {:.2}%", week_change);
    }
    
    println!("   ⭐ Priority: {}", priority);
}

fn format_number(num: f64) -> String {
    if num >= 1_000_000_000.0 {
        format!("{:.1}B", num / 1_000_000_000.0)
    } else if num >= 1_000_000.0 {
        format!("{:.1}M", num / 1_000_000.0)
    } else if num >= 1_000.0 {
        format!("{:.1}K", num / 1_000.0)
    } else {
        format!("{:.0}", num)
    }
}
