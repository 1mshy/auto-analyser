use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use yahoo_finance_api as yahoo;
use tracing::warn;

use crate::models::{QuoteResponse, MarketData};
use crate::utils::errors::{AppError, AppResult};
use crate::services::stock_list::StockListService;

pub struct MarketDataService {
    provider: yahoo::YahooConnector,
    stock_service: StockListService,
}

impl MarketDataService {
    pub fn new() -> Self {
        Self {
            provider: yahoo::YahooConnector::new().unwrap(),
            stock_service: StockListService::new(),
        }
    }

    /// Check if an error indicates a delisted stock
    fn is_delisting_error(error_msg: &str) -> bool {
        let error_lower = error_msg.to_lowercase();
        error_lower.contains("no data found") ||
        error_lower.contains("symbol may be delisted") ||
        error_lower.contains("invalid symbol") ||
        error_lower.contains("not found")
    }

    pub async fn fetch_current_quote(&self, symbol: &str) -> AppResult<QuoteResponse> {
        // Skip symbols with "^" as they are typically indices
        if StockListService::should_ignore_symbol(symbol) {
            return Err(AppError::BadRequest(format!("Symbol {} is ignored due to filtering rules", symbol)));
        }

        let response = self
            .provider
            .get_latest_quotes(symbol, "1d")
            .await
            .map_err(|e| {
                let error_msg = format!("Failed to fetch quote: {}", e);
                if Self::is_delisting_error(&error_msg) {
                    AppError::BadRequest(format!("Stock {} may be delisted: {}", symbol, e))
                } else {
                    AppError::InternalServerError(error_msg)
                }
            })?;

        let quote = response
            .last_quote()
            .map_err(|e| {
                let error_msg = format!("Failed to parse quote: {}", e);
                if Self::is_delisting_error(&error_msg) {
                    AppError::BadRequest(format!("Stock {} may be delisted: {}", symbol, e))
                } else {
                    AppError::InternalServerError(error_msg)
                }
            })?;

        Ok(QuoteResponse {
            symbol: symbol.to_string(),
            price: quote.close,
            change: quote.close - quote.open,
            change_percent: ((quote.close - quote.open) / quote.open) * 100.0,
            volume: quote.volume as i64,
            timestamp: Utc::now(),
        })
    }

    /// Fetch current quote with automatic delisting detection
    pub async fn fetch_current_quote_with_delisting_check(&self, symbol: &str, db_pool: &sqlx::PgPool) -> AppResult<QuoteResponse> {
        match self.fetch_current_quote(symbol).await {
            Ok(quote) => Ok(quote),
            Err(AppError::BadRequest(msg)) if Self::is_delisting_error(&msg) => {
                // Mark stock as delisted if we detect delisting errors
                if let Err(e) = self.stock_service.mark_stock_as_delisted(symbol, db_pool).await {
                    tracing::warn!("Failed to mark stock {} as delisted: {}", symbol, e);
                }
                Err(AppError::BadRequest(msg))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_historical_data(&self, symbol: &str) -> AppResult<Vec<MarketData>> {
        // Skip symbols with "^" as they are typically indices
        if StockListService::should_ignore_symbol(symbol) {
            return Err(AppError::BadRequest(format!("Symbol {} is ignored due to filtering rules", symbol)));
        }

        let response = self
            .provider
            .get_quote_range(symbol, "1d", "1y")
            .await
            .map_err(|e| {
                let error_msg = format!("Failed to fetch historical data: {}", e);
                if Self::is_delisting_error(&error_msg) {
                    AppError::BadRequest(format!("Stock {} may be delisted: {}", symbol, e))
                } else {
                    AppError::InternalServerError(error_msg)
                }
            })?;

        let quotes = response
            .quotes()
            .map_err(|e| {
                let error_msg = format!("Failed to parse historical data: {}", e);
                if Self::is_delisting_error(&error_msg) {
                    AppError::BadRequest(format!("Stock {} may be delisted: {}", symbol, e))
                } else {
                    AppError::InternalServerError(error_msg)
                }
            })?;

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

    /// Fetch historical data with automatic delisting detection
    pub async fn fetch_historical_data_with_delisting_check(&self, symbol: &str, db_pool: &sqlx::PgPool) -> AppResult<Vec<MarketData>> {
        match self.fetch_historical_data(symbol).await {
            Ok(data) => Ok(data),
            Err(AppError::BadRequest(msg)) if Self::is_delisting_error(&msg) => {
                // Mark stock as delisted if we detect delisting errors
                if let Err(e) = self.stock_service.mark_stock_as_delisted(symbol, db_pool).await {
                    tracing::warn!("Failed to mark stock {} as delisted: {}", symbol, e);
                }
                Err(AppError::BadRequest(msg))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_delisting_error() {
        // Should detect delisting errors
        assert!(MarketDataService::is_delisting_error("No data found"));
        assert!(MarketDataService::is_delisting_error("Symbol may be delisted"));
        assert!(MarketDataService::is_delisting_error("Invalid symbol"));
        assert!(MarketDataService::is_delisting_error("Not found"));
        assert!(MarketDataService::is_delisting_error("NO DATA FOUND")); // case insensitive
        
        // Should NOT detect non-delisting errors
        assert!(!MarketDataService::is_delisting_error("Some other error"));
        assert!(!MarketDataService::is_delisting_error("Timeout expired"));
        assert!(!MarketDataService::is_delisting_error("Connection refused"));
        assert!(!MarketDataService::is_delisting_error("Network timeout"));
        assert!(!MarketDataService::is_delisting_error("API rate limit exceeded"));
        assert!(!MarketDataService::is_delisting_error("Internal server error"));
    }
}
