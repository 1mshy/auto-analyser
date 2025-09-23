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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockFilter {
    pub min_market_cap: Option<f64>,
    pub max_market_cap: Option<f64>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub min_volume: Option<u64>,
    pub max_volume: Option<u64>,
    pub min_pct_change: Option<f64>,
    pub max_pct_change: Option<f64>,
    pub min_rsi: Option<f64>,
    pub max_rsi: Option<f64>,
    pub sectors: Option<Vec<String>>,
    pub countries: Option<Vec<String>>,
    pub industries: Option<Vec<String>>,
    pub min_ipo_year: Option<i32>,
    pub max_ipo_year: Option<i32>,
    pub oversold_rsi_threshold: Option<f64>,
    pub overbought_rsi_threshold: Option<f64>,
}

impl Default for StockFilter {
    fn default() -> Self {
        Self {
            min_market_cap: None,
            max_market_cap: None,
            min_price: None,
            max_price: None,
            min_volume: None,
            max_volume: None,
            min_pct_change: None,
            max_pct_change: None,
            min_rsi: None,
            max_rsi: None,
            sectors: None,
            countries: None,
            industries: None,
            min_ipo_year: None,
            max_ipo_year: None,
            oversold_rsi_threshold: Some(30.0),
            overbought_rsi_threshold: Some(70.0),
        }
    }
}

impl StockFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_market_cap_range(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.min_market_cap = min;
        self.max_market_cap = max;
        self
    }

    pub fn with_price_range(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.min_price = min;
        self.max_price = max;
        self
    }

    pub fn with_volume_range(mut self, min: Option<u64>, max: Option<u64>) -> Self {
        self.min_volume = min;
        self.max_volume = max;
        self
    }

    pub fn with_pct_change_range(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.min_pct_change = min;
        self.max_pct_change = max;
        self
    }

    pub fn with_rsi_range(mut self, min: Option<f64>, max: Option<f64>) -> Self {
        self.min_rsi = min;
        self.max_rsi = max;
        self
    }

    pub fn with_sectors(mut self, sectors: Vec<String>) -> Self {
        self.sectors = Some(sectors);
        self
    }

    pub fn with_countries(mut self, countries: Vec<String>) -> Self {
        self.countries = Some(countries);
        self
    }

    pub fn with_industries(mut self, industries: Vec<String>) -> Self {
        self.industries = Some(industries);
        self
    }

    pub fn with_ipo_year_range(mut self, min: Option<i32>, max: Option<i32>) -> Self {
        self.min_ipo_year = min;
        self.max_ipo_year = max;
        self
    }

    pub fn with_rsi_thresholds(mut self, oversold: Option<f64>, overbought: Option<f64>) -> Self {
        self.oversold_rsi_threshold = oversold;
        self.overbought_rsi_threshold = overbought;
        self
    }
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
     * NOTE: count=0 infers max amount (no limit)
     */
    pub async fn fetch_n_tickers(count: usize) -> Result<Vec<TickerInfo>> {
        let url = if count == 0 {
            // For unlimited, don't include limit parameter or use a very large number
            "https://api.nasdaq.com/api/screener/stocks?tableonly=true".to_string()
        } else {
            format!(
                "https://api.nasdaq.com/api/screener/stocks?tableonly=true&limit={}",
                count
            )
        };

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

        let message = if count == 0 { 
            " (no limit)".to_string() 
        } else { 
            format!(" (requested: {})", count) 
        };
        println!("📊 Fetched {} tickers from Nasdaq API{}", tickers.len(), message);
        Ok(tickers)
    }

    /// Fetch all available tickers from Nasdaq API
    pub async fn fetch_all_tickers() -> Result<Vec<TickerInfo>> {
        return StockAnalyzer::fetch_n_tickers(0).await;
    }

    /// Filter tickers by comprehensive criteria
    pub fn filter_tickers(tickers: &[TickerInfo], filter: &StockFilter) -> Vec<TickerInfo> {
        tickers
            .iter()
            .filter(|ticker| Self::passes_basic_filters(ticker, filter))
            .cloned()
            .collect()
    }

    /// Filter tickers and their corresponding RSI values
    pub async fn filter_tickers_with_analysis(
        &mut self,
        tickers: &[TickerInfo],
        filter: &StockFilter,
    ) -> Vec<(TickerInfo, Option<f64>)> {
        let mut results = Vec::new();

        for ticker in tickers {
            if !Self::passes_basic_filters(ticker, filter) {
                continue;
            }

            // Get RSI for additional filtering
            let rsi = match self.get_current_rsi(&ticker.symbol).await {
                Ok(rsi_value) => rsi_value,
                Err(_) => {
                    // If we can't get RSI, include it only if RSI filters are not specified
                    if filter.min_rsi.is_some() || filter.max_rsi.is_some() 
                        || filter.oversold_rsi_threshold.is_some() 
                        || filter.overbought_rsi_threshold.is_some() {
                        continue;
                    }
                    None
                }
            };

            // Apply RSI-based filters
            if let Some(rsi_value) = rsi {
                if let Some(min_rsi) = filter.min_rsi {
                    if rsi_value < min_rsi { continue; }
                }
                if let Some(max_rsi) = filter.max_rsi {
                    if rsi_value > max_rsi { continue; }
                }
                if let Some(oversold_threshold) = filter.oversold_rsi_threshold {
                    if rsi_value >= oversold_threshold { continue; }
                }
                if let Some(overbought_threshold) = filter.overbought_rsi_threshold {
                    if rsi_value <= overbought_threshold { continue; }
                }
            }

            results.push((ticker.clone(), rsi));
        }

        results
    }

    /// Get current RSI for a symbol
    pub async fn get_current_rsi(&mut self, symbol: &str) -> Result<Option<f64>> {
        let stock_data = self.fetch_all_stock_data(symbol).await?;
        if stock_data.is_empty() {
            return Ok(None);
        }
        
        let indicators = self.calculate_indicators(symbol, &stock_data);
        if let Some(latest_indicator) = indicators.last() {
            Ok(latest_indicator.rsi)
        } else {
            Ok(None)
        }
    }

    /// Check if a ticker passes the basic (non-RSI) filters
    fn passes_basic_filters(ticker: &TickerInfo, filter: &StockFilter) -> bool {
        // Filter by market cap
        if let (Some(market_cap_str), Some(min_cap)) = (&ticker.market_cap, filter.min_market_cap) {
            if let Ok(market_cap) = Self::parse_market_cap(market_cap_str) {
                if market_cap < min_cap { return false; }
            } else { return false; }
        }
        if let (Some(market_cap_str), Some(max_cap)) = (&ticker.market_cap, filter.max_market_cap) {
            if let Ok(market_cap) = Self::parse_market_cap(market_cap_str) {
                if market_cap > max_cap { return false; }
            } else { return false; }
        }

        // Filter by price
        if let (Some(price_str), Some(min_price)) = (&ticker.last_sale, filter.min_price) {
            if let Ok(price) = Self::parse_price(price_str) {
                if price < min_price { return false; }
            } else { return false; }
        }
        if let (Some(price_str), Some(max_price)) = (&ticker.last_sale, filter.max_price) {
            if let Ok(price) = Self::parse_price(price_str) {
                if price > max_price { return false; }
            } else { return false; }
        }

        // Filter by volume
        if let (Some(volume_str), Some(min_volume)) = (&ticker.volume, filter.min_volume) {
            if let Ok(volume) = Self::parse_volume(volume_str) {
                if volume < min_volume { return false; }
            } else { return false; }
        }
        if let (Some(volume_str), Some(max_volume)) = (&ticker.volume, filter.max_volume) {
            if let Ok(volume) = Self::parse_volume(volume_str) {
                if volume > max_volume { return false; }
            } else { return false; }
        }

        // Filter by percentage change
        if let (Some(pct_str), Some(min_pct)) = (&ticker.pct_change, filter.min_pct_change) {
            if let Ok(pct) = Self::parse_percentage(pct_str) {
                if pct < min_pct { return false; }
            } else { return false; }
        }
        if let (Some(pct_str), Some(max_pct)) = (&ticker.pct_change, filter.max_pct_change) {
            if let Ok(pct) = Self::parse_percentage(pct_str) {
                if pct > max_pct { return false; }
            } else { return false; }
        }

        // Filter by sectors
        if let Some(allowed_sectors) = &filter.sectors {
            if let Some(ticker_sector) = &ticker.sector {
                if !allowed_sectors.iter().any(|sector| 
                    ticker_sector.to_lowercase().contains(&sector.to_lowercase())
                ) {
                    return false;
                }
            } else { return false; }
        }

        // Filter by countries
        if let Some(allowed_countries) = &filter.countries {
            if let Some(ticker_country) = &ticker.country {
                if !allowed_countries.iter().any(|country| 
                    ticker_country.to_lowercase().contains(&country.to_lowercase())
                ) {
                    return false;
                }
            } else { return false; }
        }

        // Filter by industries
        if let Some(allowed_industries) = &filter.industries {
            if let Some(ticker_industry) = &ticker.industry {
                if !allowed_industries.iter().any(|industry| 
                    ticker_industry.to_lowercase().contains(&industry.to_lowercase())
                ) {
                    return false;
                }
            } else { return false; }
        }

        // Filter by IPO year
        if let (Some(ipo_str), Some(min_year)) = (&ticker.ipo_year, filter.min_ipo_year) {
            if let Ok(year) = ipo_str.parse::<i32>() {
                if year < min_year { return false; }
            } else { return false; }
        }
        if let (Some(ipo_str), Some(max_year)) = (&ticker.ipo_year, filter.max_ipo_year) {
            if let Ok(year) = ipo_str.parse::<i32>() {
                if year > max_year { return false; }
            } else { return false; }
        }

        true
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

    /// Parse price string (e.g., "$123.45") to float
    fn parse_price(price_str: &str) -> Result<f64, std::num::ParseFloatError> {
        let cleaned = price_str.replace('$', "").replace(',', "");
        cleaned.parse()
    }

    /// Parse volume string (e.g., "1,234,567") to u64
    fn parse_volume(volume_str: &str) -> Result<u64, std::num::ParseIntError> {
        let cleaned = volume_str.replace(',', "");
        cleaned.parse()
    }

    /// Parse percentage string (e.g., "2.5%", "-1.25%") to float
    fn parse_percentage(pct_str: &str) -> Result<f64, std::num::ParseFloatError> {
        let cleaned = pct_str.replace('%', "");
        cleaned.parse()
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
