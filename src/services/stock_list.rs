use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::{info, error, warn};

use crate::models::Stock;
use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Deserialize)]
struct NasdaqResponse {
    data: NasdaqData,
}

#[derive(Debug, Deserialize)]
struct NasdaqData {
    table: NasdaqTable,
}

#[derive(Debug, Deserialize)]
struct NasdaqTable {
    rows: Vec<NasdaqStock>,
}

#[derive(Debug, Deserialize)]
struct NasdaqStock {
    symbol: String,
    name: String,
    lastsale: Option<String>,
    netchange: Option<String>,
    pctchange: Option<String>,
    marketCap: Option<String>,
    country: Option<String>,
    ipoyear: Option<String>,
    volume: Option<String>,
    sector: Option<String>,
    industry: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StockInfo {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap: Option<i64>,
}

pub struct StockListService {
    client: Client,
}

impl StockListService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetch all US stocks from multiple exchanges
    pub async fn fetch_all_us_stocks(&self) -> AppResult<Vec<StockInfo>> {
        let mut all_stocks = Vec::new();
        let mut seen_symbols = HashSet::new();

        // Fetch from NASDAQ
        match self.fetch_nasdaq_stocks().await {
            Ok(mut nasdaq_stocks) => {
                for stock in &nasdaq_stocks {
                    seen_symbols.insert(stock.symbol.clone());
                }
                all_stocks.append(&mut nasdaq_stocks);
                info!("Fetched {} stocks from NASDAQ", nasdaq_stocks.len());
            }
            Err(e) => {
                error!("Failed to fetch NASDAQ stocks: {}", e);
            }
        }

        // Fetch from NYSE
        match self.fetch_nyse_stocks().await {
            Ok(mut nyse_stocks) => {
                let mut unique_nyse = Vec::new();
                for stock in nyse_stocks {
                    if !seen_symbols.contains(&stock.symbol) {
                        seen_symbols.insert(stock.symbol.clone());
                        unique_nyse.push(stock);
                    }
                }
                let count = unique_nyse.len();
                all_stocks.append(&mut unique_nyse);
                info!("Fetched {} unique stocks from NYSE", count);
            }
            Err(e) => {
                error!("Failed to fetch NYSE stocks: {}", e);
            }
        }

        info!("Total unique stocks fetched: {}", all_stocks.len());
        Ok(all_stocks)
    }

    async fn fetch_nasdaq_stocks(&self) -> AppResult<Vec<StockInfo>> {
        let url = "https://api.nasdaq.com/api/screener/stocks?tableonly=true&limit=0";
        
        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to fetch NASDAQ data: {}", e)))?;

        let text = response.text().await
            .map_err(|e| AppError::InternalServerError(format!("Failed to read NASDAQ response: {}", e)))?;

        let nasdaq_response: NasdaqResponse = serde_json::from_str(&text)
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse NASDAQ response: {}", e)))?;

        let stocks: Vec<StockInfo> = nasdaq_response
            .data
            .table
            .rows
            .into_iter()
            .filter_map(|stock| {
                if stock.symbol.is_empty() || stock.name.is_empty() {
                    return None;
                }

                let market_cap = stock.marketCap
                    .and_then(|cap| cap.replace('$', "").replace(',', "").parse::<f64>().ok())
                    .map(|cap| cap as i64);

                Some(StockInfo {
                    symbol: stock.symbol,
                    name: stock.name,
                    exchange: "NASDAQ".to_string(),
                    sector: stock.sector,
                    industry: stock.industry,
                    market_cap,
                })
            })
            .collect();

        Ok(stocks)
    }

    async fn fetch_nyse_stocks(&self) -> AppResult<Vec<StockInfo>> {
        let url = "https://api.nasdaq.com/api/screener/stocks?tableonly=true&limit=0";
        
        let response = self
            .client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to fetch NYSE data: {}", e)))?;

        let text = response.text().await
            .map_err(|e| AppError::InternalServerError(format!("Failed to read NYSE response: {}", e)))?;

        let nasdaq_response: NasdaqResponse = serde_json::from_str(&text)
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse NYSE response: {}", e)))?;

        let stocks: Vec<StockInfo> = nasdaq_response
            .data
            .table
            .rows
            .into_iter()
            .filter_map(|stock| {
                if stock.symbol.is_empty() || stock.name.is_empty() {
                    return None;
                }

                let market_cap = stock.marketCap
                    .and_then(|cap| cap.replace('$', "").replace(',', "").parse::<f64>().ok())
                    .map(|cap| cap as i64);

                Some(StockInfo {
                    symbol: stock.symbol,
                    name: stock.name,
                    exchange: "NYSE".to_string(),
                    sector: stock.sector,
                    industry: stock.industry,
                    market_cap,
                })
            })
            .collect();

        Ok(stocks)
    }

    /// Get a predefined list of major US stocks as fallback
    pub fn get_major_us_stocks(&self) -> Vec<StockInfo> {
        vec![
            // Tech giants
            StockInfo { symbol: "AAPL".to_string(), name: "Apple Inc.".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Technology".to_string()), industry: Some("Consumer Electronics".to_string()), market_cap: None },
            StockInfo { symbol: "MSFT".to_string(), name: "Microsoft Corporation".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Technology".to_string()), industry: Some("Software".to_string()), market_cap: None },
            StockInfo { symbol: "GOOGL".to_string(), name: "Alphabet Inc.".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Technology".to_string()), industry: Some("Internet Content & Information".to_string()), market_cap: None },
            StockInfo { symbol: "AMZN".to_string(), name: "Amazon.com Inc.".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Consumer Discretionary".to_string()), industry: Some("Internet Retail".to_string()), market_cap: None },
            StockInfo { symbol: "TSLA".to_string(), name: "Tesla Inc.".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Consumer Discretionary".to_string()), industry: Some("Auto Manufacturers".to_string()), market_cap: None },
            StockInfo { symbol: "META".to_string(), name: "Meta Platforms Inc.".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Technology".to_string()), industry: Some("Internet Content & Information".to_string()), market_cap: None },
            StockInfo { symbol: "NVDA".to_string(), name: "NVIDIA Corporation".to_string(), exchange: "NASDAQ".to_string(), sector: Some("Technology".to_string()), industry: Some("Semiconductors".to_string()), market_cap: None },
            
            // Financial
            StockInfo { symbol: "JPM".to_string(), name: "JPMorgan Chase & Co.".to_string(), exchange: "NYSE".to_string(), sector: Some("Financial Services".to_string()), industry: Some("Banks".to_string()), market_cap: None },
            StockInfo { symbol: "BAC".to_string(), name: "Bank of America Corp".to_string(), exchange: "NYSE".to_string(), sector: Some("Financial Services".to_string()), industry: Some("Banks".to_string()), market_cap: None },
            StockInfo { symbol: "WFC".to_string(), name: "Wells Fargo & Company".to_string(), exchange: "NYSE".to_string(), sector: Some("Financial Services".to_string()), industry: Some("Banks".to_string()), market_cap: None },
            
            // Healthcare
            StockInfo { symbol: "JNJ".to_string(), name: "Johnson & Johnson".to_string(), exchange: "NYSE".to_string(), sector: Some("Healthcare".to_string()), industry: Some("Drug Manufacturers".to_string()), market_cap: None },
            StockInfo { symbol: "PFE".to_string(), name: "Pfizer Inc.".to_string(), exchange: "NYSE".to_string(), sector: Some("Healthcare".to_string()), industry: Some("Drug Manufacturers".to_string()), market_cap: None },
            
            // Retail
            StockInfo { symbol: "WMT".to_string(), name: "Walmart Inc.".to_string(), exchange: "NYSE".to_string(), sector: Some("Consumer Defensive".to_string()), industry: Some("Discount Stores".to_string()), market_cap: None },
            StockInfo { symbol: "HD".to_string(), name: "The Home Depot Inc.".to_string(), exchange: "NYSE".to_string(), sector: Some("Consumer Discretionary".to_string()), industry: Some("Home Improvement Retail".to_string()), market_cap: None },
            
            // Energy
            StockInfo { symbol: "XOM".to_string(), name: "Exxon Mobil Corporation".to_string(), exchange: "NYSE".to_string(), sector: Some("Energy".to_string()), industry: Some("Oil & Gas Integrated".to_string()), market_cap: None },
            StockInfo { symbol: "CVX".to_string(), name: "Chevron Corporation".to_string(), exchange: "NYSE".to_string(), sector: Some("Energy".to_string()), industry: Some("Oil & Gas Integrated".to_string()), market_cap: None },
        ]
    }
}
