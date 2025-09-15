use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use yahoo_finance_api as yahoo;

use crate::models::{QuoteResponse, MarketData};
use crate::utils::errors::{AppError, AppResult};

pub struct MarketDataService {
    provider: yahoo::YahooConnector,
}

impl MarketDataService {
    pub fn new() -> Self {
        Self {
            provider: yahoo::YahooConnector::new().unwrap(),
        }
    }

    pub async fn fetch_current_quote(&self, symbol: &str) -> AppResult<QuoteResponse> {
        let response = self
            .provider
            .get_latest_quotes(symbol, "1d")
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to fetch quote: {}", e)))?;

        let quote = response
            .last_quote()
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse quote: {}", e)))?;

        Ok(QuoteResponse {
            symbol: symbol.to_string(),
            price: quote.close,
            change: quote.close - quote.open,
            change_percent: ((quote.close - quote.open) / quote.open) * 100.0,
            volume: quote.volume as i64,
            timestamp: Utc::now(),
        })
    }

    pub async fn fetch_historical_data(&self, symbol: &str, period: &str) -> AppResult<Vec<MarketData>> {
        let response = self
            .provider
            .get_quote_range(symbol, "1d", period)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to fetch historical data: {}", e)))?;

        let quotes = response
            .quotes()
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse historical data: {}", e)))?;

        let market_data: Vec<MarketData> = quotes
            .iter()
            .map(|quote| MarketData {
                id: uuid::Uuid::new_v4(),
                symbol: symbol.to_string(),
                timestamp: DateTime::from_timestamp(quote.timestamp as i64, 0)
                    .unwrap_or_else(|| Utc::now()),
                open: Decimal::from_f64_retain(quote.open).unwrap_or_default(),
                high: Decimal::from_f64_retain(quote.high).unwrap_or_default(),
                low: Decimal::from_f64_retain(quote.low).unwrap_or_default(),
                close: Decimal::from_f64_retain(quote.close).unwrap_or_default(),
                volume: quote.volume as i64,
                created_at: Utc::now(),
                last_updated: Some(Utc::now()),
            })
            .collect();

        Ok(market_data)
    }
}
