use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error, warn, debug};
use std::collections::HashMap;

use crate::api::AppState;
use crate::models::StockPriority;
use crate::services::market_data::MarketDataService;

pub struct PriorityScheduler {
    state: Arc<AppState>,
    market_service: MarketDataService,
    last_update_times: HashMap<StockPriority, std::time::Instant>,
}

impl PriorityScheduler {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            market_service: MarketDataService::new(),
            last_update_times: HashMap::new(),
        }
    }

    pub async fn start_priority_based_scheduler(state: Arc<AppState>) {
        let scheduler = Self::new(state);
        
        info!("Starting priority-based market data scheduler");
        
        // Initialize watchlist priorities on startup
        tokio::spawn({
            let state = scheduler.state.clone();
            async move {
                if let Err(e) = Self::initialize_watchlist_priorities(&state).await {
                    error!("Failed to initialize watchlist priorities: {}", e);
                }
            }
        });

        // Start the main scheduling loop
        scheduler.run_scheduler().await;
    }

    async fn run_scheduler(mut self) {
        // Check every 30 seconds for stocks that need updates
        let mut interval_timer = interval(Duration::from_secs(30));
        
        loop {
            interval_timer.tick().await;
            
            // Process each priority level
            for priority in [StockPriority::High, StockPriority::Medium, StockPriority::Low] {
                if self.should_update_priority(priority) {
                    tokio::spawn({
                        let state = self.state.clone();
                        let market_service = MarketDataService::new();
                        async move {
                            if let Err(e) = Self::update_stocks_by_priority(&state, priority, &market_service).await {
                                error!("Failed to update {} priority stocks: {}", 
                                    format!("{:?}", priority).to_lowercase(), e);
                            }
                        }
                    });
                    
                    self.last_update_times.insert(priority, std::time::Instant::now());
                }
            }
        }
    }

    fn should_update_priority(&self, priority: StockPriority) -> bool {
        let update_interval = Duration::from_secs(priority.update_interval_seconds());
        
        match self.last_update_times.get(&priority) {
            Some(last_update) => last_update.elapsed() >= update_interval,
            None => true, // First time, always update
        }
    }

    async fn update_stocks_by_priority(
        state: &Arc<AppState>,
        priority: StockPriority,
        market_service: &MarketDataService
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        
        let symbols = state.db.get_stocks_needing_update(priority).await?;
        
        if symbols.is_empty() {
            debug!("No {} priority stocks need updating", format!("{:?}", priority).to_lowercase());
            return Ok(());
        }

        info!("Updating {} {} priority stocks", symbols.len(), format!("{:?}", priority).to_lowercase());

        let mut successful_updates = 0;
        let mut failed_updates = 0;

        // Determine batch size based on priority
        let batch_size = match priority {
            StockPriority::High => 20,   // Smaller batches for high priority (faster updates)
            StockPriority::Medium => 50, // Medium batches
            StockPriority::Low => 100,   // Larger batches for low priority
        };

        // Determine delay between requests based on priority
        let request_delay = match priority {
            StockPriority::High => Duration::from_millis(100),   // Faster for high priority
            StockPriority::Medium => Duration::from_millis(200), // Medium delay
            StockPriority::Low => Duration::from_millis(500),    // Slower for low priority
        };

        for batch in symbols.chunks(batch_size) {
            for symbol in batch {
                match market_service.fetch_current_quote_with_delisting_check(symbol, state.db.pool()).await {
                    Ok(quote) => {
                        // Store the quote data as market data
                        let market_data = crate::models::MarketData {
                            id: uuid::Uuid::new_v4(),
                            symbol: symbol.to_string(),
                            timestamp: quote.timestamp,
                            open: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                            high: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                            low: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                            close: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                            volume: quote.volume,
                            created_at: chrono::Utc::now(),
                            last_updated: Some(chrono::Utc::now()),
                        };

                        if let Err(e) = state.db.store_market_data(&market_data).await {
                            error!("Failed to store market data for {}: {}", symbol, e);
                            failed_updates += 1;
                        } else {
                            // Update the stock's last price update timestamp
                            if let Err(e) = state.db.update_stock_price_timestamp(symbol).await {
                                warn!("Failed to update price timestamp for {}: {}", symbol, e);
                            }
                            successful_updates += 1;
                        }
                    }
                    Err(e) => {
                        error!("Failed to fetch market data for {}: {}", symbol, e);
                        failed_updates += 1;
                    }
                }

                // Rate limiting delay
                tokio::time::sleep(request_delay).await;
            }

            // Longer delay between batches
            let batch_delay = match priority {
                StockPriority::High => Duration::from_secs(1),
                StockPriority::Medium => Duration::from_secs(2),
                StockPriority::Low => Duration::from_secs(5),
            };
            tokio::time::sleep(batch_delay).await;
        }

        info!("{} priority update completed: {} successful, {} failed", 
            format!("{:?}", priority), successful_updates, failed_updates);
        Ok(())
    }

    async fn initialize_watchlist_priorities(state: &Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Initializing watchlist stock priorities...");
        
        let watchlist_symbols = state.db.get_watchlist_symbols().await?;
        
        if watchlist_symbols.is_empty() {
            info!("No watchlist symbols found");
            return Ok(());
        }

        info!("Setting high priority for {} watchlist stocks", watchlist_symbols.len());

        for symbol in watchlist_symbols {
            if let Err(e) = state.db.set_stock_priority(&symbol, StockPriority::High).await {
                warn!("Failed to set high priority for watchlist stock {}: {}", symbol, e);
            }
        }

        info!("Watchlist priority initialization completed");
        Ok(())
    }

    /// Manually trigger an update for a specific stock (useful for watchlist additions)
    pub async fn trigger_immediate_update(
        state: &Arc<AppState>, 
        symbol: &str
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let market_service = MarketDataService::new();
        
        match market_service.fetch_current_quote_with_delisting_check(symbol, state.db.pool()).await {
            Ok(quote) => {
                let market_data = crate::models::MarketData {
                    id: uuid::Uuid::new_v4(),
                    symbol: symbol.to_string(),
                    timestamp: quote.timestamp,
                    open: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                    high: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                    low: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                    close: rust_decimal::Decimal::from_f64_retain(quote.price).unwrap_or_default(),
                    volume: quote.volume,
                    created_at: chrono::Utc::now(),
                    last_updated: Some(chrono::Utc::now()),
                };

                state.db.store_market_data(&market_data).await?;
                state.db.update_stock_price_timestamp(symbol).await?;
                
                info!("Immediate update completed for {}", symbol);
                Ok(())
            }
            Err(e) => {
                error!("Failed immediate update for {}: {}", symbol, e);
                Err(e.into())
            }
        }
    }
}
