use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;

use crate::indicators::{CustomRSI, SimpleMovingAverage, MovingAverageConvergenceDivergence};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerInfo {
    pub symbol: String,
    pub name: String,
    pub last_sale: Option<String>,
    pub net_change: Option<String>,
    pub pct_change: Option<String>,
    pub market_cap: Option<String>,
    pub country: Option<String>,
    pub ipo_year: Option<String>,
    pub volume: Option<String>,
    pub sector: Option<String>,
    pub industry: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NasdaqApiResponse {
    data: NasdaqData,
}

#[derive(Debug, Deserialize)]
struct NasdaqData {
    table: NasdaqTable,
}

#[derive(Debug, Deserialize)]
struct NasdaqTable {
    rows: Vec<NasdaqRow>,
}

#[derive(Debug, Deserialize)]
struct NasdaqRow {
    symbol: String,
    name: String,
    #[serde(rename = "lastsale")]
    last_sale: Option<String>,
    #[serde(rename = "netchange")]
    net_change: Option<String>,
    #[serde(rename = "pctchange")]
    pct_change: Option<String>,
    #[serde(rename = "marketCap")]
    market_cap: Option<String>,
    country: Option<String>,
    #[serde(rename = "ipoyear")]
    ipo_year: Option<String>,
    volume: Option<String>,
    sector: Option<String>,
    industry: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StockData {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug, Clone)]
pub struct TechnicalIndicators {
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub rsi: Option<f64>,
    pub macd: Option<(f64, f64, f64)>, // (macd, signal, histogram)
}

pub struct StockAnalyzer {
    provider: yahoo::YahooConnector,
    indicators: HashMap<String, IndicatorSet>,
}

struct IndicatorSet {
    sma_20: SimpleMovingAverage,
    sma_50: SimpleMovingAverage,
    rsi: CustomRSI,
    macd: MovingAverageConvergenceDivergence,
}

struct PriceChange {
    pub symbol: String,
    pub percent_change: f64,
    pub start_price: f64,
    pub end_price: f64,
}

struct AnalysisResult {
    pub symbol: String,
    pub priority: u32,
    pub latest_data: StockData,
    pub latest_indicators: TechnicalIndicators,
}

impl StockAnalyzer {
    pub fn new() -> Self {
        Self {
            provider: yahoo::YahooConnector::new().unwrap(),
            indicators: HashMap::new(),
        }
    }
    /**
     * Fetches all historical stock data of a symbol in 1 day intervals
     */
    pub async fn fetch_all_stock_data(&self, symbol: &str) -> Result<Vec<StockData>> {
        return self
            .fetch_stock_data(symbol, DateTime::<Utc>::UNIX_EPOCH, Utc::now())
            .await;
    }

    /// Fetch historical stock data for a given symbol
    pub async fn fetch_stock_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<StockData>> {
        // Convert chrono DateTime to time OffsetDateTime
        let start_time = OffsetDateTime::from_unix_timestamp(start.timestamp())?;
        let end_time = OffsetDateTime::from_unix_timestamp(end.timestamp())?;

        let response = self
            .provider
            .get_quote_history(symbol, start_time, end_time)
            .await?;

        let mut stock_data = Vec::new();
        let quotes = response.quotes()?;

        for quote in quotes {
            stock_data.push(StockData {
                symbol: symbol.to_string(),
                timestamp: DateTime::from_timestamp(quote.timestamp as i64, 0)
                    .unwrap_or(Utc::now()),
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume,
            });
        }

        // Sort by timestamp (oldest first)
        stock_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(stock_data)
    }

    /// Get the latest quote for a symbol
    pub async fn get_latest_quote(&self, symbol: &str) -> Result<StockData> {
        let response = self.provider.get_latest_quotes(symbol, "1d").await?;
        let quote = response.last_quote()?;

        Ok(StockData {
            symbol: symbol.to_string(),
            timestamp: DateTime::from_timestamp(quote.timestamp as i64, 0).unwrap_or(Utc::now()),
            open: quote.open,
            high: quote.high,
            low: quote.low,
            close: quote.close,
            volume: quote.volume,
        })
    }

    /// Initialize indicators for a specific symbol
    fn initialize_indicators(&mut self, symbol: &str) {
        let indicator_set = IndicatorSet {
            sma_20: SimpleMovingAverage::new(20).unwrap(),
            sma_50: SimpleMovingAverage::new(50).unwrap(),
            rsi: CustomRSI::new(14),
            macd: MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap(),
        };
        self.indicators.insert(symbol.to_string(), indicator_set);
    }

    /// Calculate technical indicators for stock data
    pub fn calculate_indicators(
        &mut self,
        symbol: &str,
        stock_data: &[StockData],
    ) -> Vec<TechnicalIndicators> {
        if !self.indicators.contains_key(symbol) {
            self.initialize_indicators(symbol);
        }

        let mut results = Vec::new();

        if let Some(indicators) = self.indicators.get_mut(symbol) {
            // Reset indicators
            indicators.sma_20.reset();
            indicators.sma_50.reset();
            indicators.rsi.reset();
            indicators.macd.reset();

            for data in stock_data {
                let sma_20 = indicators.sma_20.next(data.close);
                let sma_50 = indicators.sma_50.next(data.close);
                let rsi = indicators.rsi.next(data.close);
                let macd_result = indicators.macd.next(data.close);

                results.push(TechnicalIndicators {
                    sma_20: Some(sma_20),
                    sma_50: Some(sma_50),
                    rsi: rsi,
                    macd: Some((macd_result.macd, macd_result.signal, macd_result.histogram)),
                });
            }
        }

        results
    }

    /// Analyze stock with basic signals
    pub fn analyze_signals(
        &self,
        data: &StockData,
        indicators: &TechnicalIndicators,
    ) -> Vec<String> {
        let mut signals = Vec::new();

        // RSI signals
        if let Some(rsi) = indicators.rsi {
            if rsi > 70.0 {
                signals.push("RSI Overbought (>70)".to_string());
            } else if rsi < 30.0 {
                signals.push("RSI Oversold (<30)".to_string());
            }
        }

        // SMA crossover signals
        if let (Some(sma_20), Some(sma_50)) = (indicators.sma_20, indicators.sma_50) {
            if sma_20 > sma_50 && data.close > sma_20 {
                signals.push("Bullish: Price above SMA20 > SMA50".to_string());
            } else if sma_20 < sma_50 && data.close < sma_20 {
                signals.push("Bearish: Price below SMA20 < SMA50".to_string());
            }
        }

        // MACD signals
        if let Some((macd, signal, _)) = indicators.macd {
            if macd > signal {
                signals.push("MACD Bullish: MACD above Signal".to_string());
            } else {
                signals.push("MACD Bearish: MACD below Signal".to_string());
            }
        }

        signals
    }

    /// Print analysis results
    pub fn print_analysis(
        &self,
        symbol: &str,
        stock_data: &[StockData],
        indicators: &[TechnicalIndicators],
    ) {
        println!("=== Stock Analysis for {} ===", symbol);

        if let (Some(latest_data), Some(latest_indicators)) = (stock_data.last(), indicators.last())
        {
            println!(
                "Latest Data ({}):",
                latest_data.timestamp.format("%Y-%m-%d")
            );
            println!("  Price: ${:.2}", latest_data.close);
            println!("  Volume: {}", latest_data.volume);

            println!("\nTechnical Indicators:");
            if let Some(sma_20) = latest_indicators.sma_20 {
                println!("  SMA(20): ${:.2}", sma_20);
            }
            if let Some(sma_50) = latest_indicators.sma_50 {
                println!("  SMA(50): ${:.2}", sma_50);
            }
            if let Some(rsi) = latest_indicators.rsi {
                println!("  RSI(14): {:.2}", rsi);
            }
            if let Some((macd, signal, histogram)) = latest_indicators.macd {
                println!(
                    "  MACD: {:.4}, Signal: {:.4}, Histogram: {:.4}",
                    macd, signal, histogram
                );
            }

            println!("\nSignals:");
            let signals = self.analyze_signals(latest_data, latest_indicators);
            if signals.is_empty() {
                println!("  No strong signals detected");
            } else {
                for signal in signals {
                    println!("  • {}", signal);
                }
            }
        }
    }
    /**
     * Fetches n amount of tickers from the nasdaq api.
     * NOTE: count=0 infers max amount
     */
    pub async fn fetch_n_tickers(count: usize) -> Result<Vec<TickerInfo>> {
        let url = format!(
            "https://api.nasdaq.com/api/screener/stocks?tableonly=true&limit={}",
            count
        );

        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;

        let nasdaq_response: NasdaqApiResponse = response.json().await?;

        let mut tickers = Vec::new();
        for row in nasdaq_response.data.table.rows {
            if row.symbol.contains("^") || row.symbol.contains("/") {
                continue; // Skip indices and special symbols
            }
            tickers.push(TickerInfo {
                symbol: row.symbol,
                name: row.name,
                last_sale: row.last_sale,
                net_change: row.net_change,
                pct_change: row.pct_change,
                market_cap: row.market_cap,
                country: row.country,
                ipo_year: row.ipo_year,
                volume: row.volume,
                sector: row.sector,
                industry: row.industry,
            });
        }

        println!("📊 Fetched {} tickers from Nasdaq API", tickers.len());
        Ok(tickers)
    }

    /// Fetch all available tickers from Nasdaq API
    pub async fn fetch_all_tickers() -> Result<Vec<TickerInfo>> {
        return StockAnalyzer::fetch_n_tickers(0).await;
    }

    /// Filter tickers by various criteria
    pub fn filter_tickers(
        tickers: &[TickerInfo],
        sector: Option<&str>,
        min_market_cap: Option<f64>,
        country: Option<&str>,
    ) -> Vec<TickerInfo> {
        tickers
            .iter()
            .filter(|ticker| {
                // Filter by sector if specified
                if let Some(sector_filter) = sector {
                    if let Some(ticker_sector) = &ticker.sector {
                        if !ticker_sector
                            .to_lowercase()
                            .contains(&sector_filter.to_lowercase())
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by minimum market cap if specified
                if let Some(min_cap) = min_market_cap {
                    if let Some(market_cap_str) = &ticker.market_cap {
                        if let Ok(market_cap) = Self::parse_market_cap(market_cap_str) {
                            if market_cap < min_cap {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                // Filter by country if specified
                if let Some(country_filter) = country {
                    if let Some(ticker_country) = &ticker.country {
                        if !ticker_country
                            .to_lowercase()
                            .contains(&country_filter.to_lowercase())
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// Parse market cap string (e.g., "$1.5B", "$500M") to float
    fn parse_market_cap(market_cap_str: &str) -> Result<f64, std::num::ParseFloatError> {
        let cleaned = market_cap_str.replace('$', "").replace(',', "");

        if cleaned.ends_with('B') {
            let num_str = cleaned.trim_end_matches('B');
            let num: f64 = num_str.parse()?;
            Ok(num * 1_000_000_000.0)
        } else if cleaned.ends_with('M') {
            let num_str = cleaned.trim_end_matches('M');
            let num: f64 = num_str.parse()?;
            Ok(num * 1_000_000.0)
        } else if cleaned.ends_with('K') {
            let num_str = cleaned.trim_end_matches('K');
            let num: f64 = num_str.parse()?;
            Ok(num * 1_000.0)
        } else {
            cleaned.parse()
        }
    }

    /// Get top performing tickers by percentage change
    pub fn get_top_performers(tickers: &[TickerInfo], limit: usize) -> Vec<TickerInfo> {
        let mut sorted_tickers: Vec<TickerInfo> = tickers
            .iter()
            .filter_map(|ticker| {
                if let Some(pct_change_str) = &ticker.pct_change {
                    let cleaned = pct_change_str.replace('%', "");
                    if cleaned.parse::<f64>().is_ok() {
                        let ticker_clone = ticker.clone();
                        // Store the parsed percentage for sorting
                        return Some(ticker_clone);
                    }
                }
                None
            })
            .collect();

        // Sort by percentage change (descending)
        sorted_tickers.sort_by(|a, b| {
            let a_pct = a
                .pct_change
                .as_ref()
                .and_then(|s| s.replace('%', "").parse::<f64>().ok())
                .unwrap_or(0.0);
            let b_pct = b
                .pct_change
                .as_ref()
                .and_then(|s| s.replace('%', "").parse::<f64>().ok())
                .unwrap_or(0.0);
            b_pct
                .partial_cmp(&a_pct)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        sorted_tickers.into_iter().take(limit).collect()
    }

    /// Print ticker information in a formatted table
    pub fn print_tickers(tickers: &[TickerInfo], title: &str) {
        println!("\n{}", "=".repeat(80));
        println!("📊 {}", title);
        println!("{}", "=".repeat(80));
        println!(
            "{:<8} {:<30} {:<12} {:<10} {:<15} {:<20}",
            "Symbol", "Name", "Last Sale", "Change%", "Market Cap", "Sector"
        );
        println!("{}", "-".repeat(80));

        for ticker in tickers.iter().take(20) {
            // Show max 20 for readability
            println!(
                "{:<8} {:<30} {:<12} {:<10} {:<15} {:<20}",
                ticker.symbol,
                ticker.name.chars().take(28).collect::<String>(),
                ticker.last_sale.as_deref().unwrap_or("N/A"),
                ticker.pct_change.as_deref().unwrap_or("N/A"),
                ticker.market_cap.as_deref().unwrap_or("N/A"),
                ticker
                    .sector
                    .as_deref()
                    .unwrap_or("N/A")
                    .chars()
                    .take(18)
                    .collect::<String>()
            );
        }

        if tickers.len() > 20 {
            println!("... and {} more tickers", tickers.len() - 20);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_stock_data_creation() {
        let stock_data = StockData {
            symbol: "AAPL".to_string(),
            timestamp: Utc::now(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 1000000,
        };

        assert_eq!(stock_data.symbol, "AAPL");
        assert_eq!(stock_data.open, 100.0);
        assert_eq!(stock_data.close, 102.0);
    }

    #[test]
    fn test_stock_analyzer_creation() {
        let _analyzer = StockAnalyzer::new();
        // Just test that we can create an analyzer without panic
        assert!(true);
    }

    #[test]
    fn test_technical_indicators_creation() {
        let indicators = TechnicalIndicators {
            sma_20: Some(100.0),
            sma_50: Some(95.0),
            rsi: Some(65.0),
            macd: Some((0.5, 0.3, 0.2)),
        };

        assert_eq!(indicators.sma_20, Some(100.0));
        assert_eq!(indicators.rsi, Some(65.0));
    }
}
