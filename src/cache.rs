use dashmap::DashMap;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;

use crate::{StockData, TechnicalIndicators, TickerInfo};

#[derive(Clone)]
pub struct CacheManager {
    stock_data_cache: Cache<String, (Vec<StockData>, Instant)>,
    indicators_cache: Cache<String, (Vec<TechnicalIndicators>, Instant)>,
    tickers_cache: Cache<String, (Vec<TickerInfo>, Instant)>,
    rate_limiter: Arc<DashMap<String, Instant>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            stock_data_cache: Cache::builder()
                .time_to_live(Duration::from_secs(300)) // 5 minutes
                .max_capacity(1000)
                .build(),
            indicators_cache: Cache::builder()
                .time_to_live(Duration::from_secs(300)) // 5 minutes
                .max_capacity(1000)
                .build(),
            tickers_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600)) // 1 hour
                .max_capacity(10)
                .build(),
            rate_limiter: Arc::new(DashMap::new()),
        }
    }

    pub async fn get_stock_data(&self, key: &str) -> Option<Vec<StockData>> {
        if let Some((data, cached_at)) = self.stock_data_cache.get(key).await {
            // Check if cache is still fresh (less than 5 minutes old)
            if cached_at.elapsed() < Duration::from_secs(300) {
                tracing::debug!("Cache hit for stock data: {}", key);
                return Some(data);
            }
        }
        None
    }

    pub async fn cache_stock_data(&self, key: String, data: Vec<StockData>) {
        tracing::debug!("Caching stock data: {}", key);
        self.stock_data_cache.insert(key, (data, Instant::now())).await;
    }

    pub async fn get_indicators(&self, key: &str) -> Option<Vec<TechnicalIndicators>> {
        if let Some((indicators, cached_at)) = self.indicators_cache.get(key).await {
            if cached_at.elapsed() < Duration::from_secs(300) {
                tracing::debug!("Cache hit for indicators: {}", key);
                return Some(indicators);
            }
        }
        None
    }

    pub async fn cache_indicators(&self, key: String, indicators: Vec<TechnicalIndicators>) {
        tracing::debug!("Caching indicators: {}", key);
        self.indicators_cache.insert(key, (indicators, Instant::now())).await;
    }

    pub async fn get_tickers(&self, key: &str) -> Option<Vec<TickerInfo>> {
        if let Some((tickers, cached_at)) = self.tickers_cache.get(key).await {
            if cached_at.elapsed() < Duration::from_secs(3600) {
                tracing::debug!("Cache hit for tickers: {}", key);
                return Some(tickers);
            }
        }
        None
    }

    pub async fn cache_tickers(&self, key: String, tickers: Vec<TickerInfo>) {
        tracing::debug!("Caching tickers: {}", key);
        self.tickers_cache.insert(key, (tickers, Instant::now())).await;
    }

    pub fn should_rate_limit(&self, identifier: &str, min_interval: Duration) -> bool {
        if let Some(last_request) = self.rate_limiter.get(identifier) {
            if last_request.elapsed() < min_interval {
                tracing::warn!("Rate limiting request for: {}", identifier);
                return true;
            }
        }
        
        self.rate_limiter.insert(identifier.to_string(), Instant::now());
        false
    }

    pub async fn clear_cache(&self) {
        tracing::info!("Clearing all caches");
        self.stock_data_cache.invalidate_all();
        self.indicators_cache.invalidate_all();
        self.tickers_cache.invalidate_all();
        self.rate_limiter.clear();
    }

    pub async fn get_cache_stats(&self) -> CacheStats {
        CacheStats {
            stock_data_entries: self.stock_data_cache.entry_count(),
            indicators_entries: self.indicators_cache.entry_count(),
            tickers_entries: self.tickers_cache.entry_count(),
            rate_limiter_entries: self.rate_limiter.len(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CacheStats {
    pub stock_data_entries: u64,
    pub indicators_entries: u64,
    pub tickers_entries: u64,
    pub rate_limiter_entries: usize,
}