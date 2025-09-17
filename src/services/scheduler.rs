use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error, warn};

use crate::api::AppState;
use crate::services::market_data::MarketDataService;
use crate::services::stock_list::StockListService;

pub async fn start_market_data_scheduler(state: Arc<AppState>) {
    let mut interval_timer = interval(Duration::from_secs(state.config.market_data_interval_seconds));
    
    info!("Starting market data scheduler with interval: {}s", state.config.market_data_interval_seconds);

    // Initialize stock list on startup
    tokio::spawn({
        let state = state.clone();
        async move {
            if let Err(e) = initialize_stock_list(&state).await {
                error!("Failed to initialize stock list: {}", e);
            }
        }
    });

    loop {
        interval_timer.tick().await;
        
        match update_market_data(&state).await {
            Ok(_) => info!("Market data update completed successfully"),
            Err(e) => error!("Market data update failed: {}", e),
        }
    }
}

pub async fn start_stock_list_scheduler(state: Arc<AppState>) {
    // Update stock list once per day (24 hours)
    let mut interval_timer = interval(Duration::from_secs(24 * 60 * 60));
    
    info!("Starting stock list scheduler with daily updates");

    loop {
        interval_timer.tick().await;
        
        match update_stock_list(&state).await {
            Ok(_) => info!("Stock list update completed successfully"),
            Err(e) => error!("Stock list update failed: {}", e),
        }
    }
}

async fn update_market_data(state: &Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get all active stock symbols, filtering out delisted ones and symbols with "^"
    let symbols = state.db.get_all_stock_symbols().await?;
    
    // Filter symbols to exclude ones that should be ignored
    let filtered_symbols: Vec<String> = symbols
        .into_iter()
        .filter(|symbol| !StockListService::should_ignore_symbol(symbol))
        .collect();
    
    if filtered_symbols.is_empty() {
        warn!("No active stocks found in database. Updating stock list first...");
        if let Err(e) = update_stock_list(state).await {
            error!("Failed to update stock list: {}", e);
            return Ok(());
        }
        // Try again after stock list update
        let symbols = state.db.get_all_stock_symbols().await?;
        let filtered_symbols: Vec<String> = symbols
            .into_iter()
            .filter(|symbol| !StockListService::should_ignore_symbol(symbol))
            .collect();
            
        if filtered_symbols.is_empty() {
            warn!("Still no active stocks found after update");
            return Ok(());
        }
    }

    info!("Updating market data for {} active symbols", filtered_symbols.len());

    let market_service = MarketDataService::new();
    let mut successful_updates = 0;
    let mut failed_updates = 0;

    // Process stocks in batches to avoid overwhelming the API
    const BATCH_SIZE: usize = 50;
    for batch in filtered_symbols.chunks(BATCH_SIZE) {
        for symbol in batch {
            match market_service.fetch_historical_data_with_delisting_check(symbol, state.db.pool()).await {
                Ok(data) => {
                    for market_data in data {
                        if let Err(e) = state.db.store_market_data(&market_data).await {
                            error!("Failed to store market data for {}: {}", symbol, e);
                            failed_updates += 1;
                        } else {
                            successful_updates += 1;
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to fetch market data for {}: {}", symbol, e);
                    failed_updates += 1;
                }
            }

            // Small delay to avoid rate limiting
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        // Longer delay between batches
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    info!("Market data update completed: {} successful, {} failed", successful_updates, failed_updates);
    Ok(())
}

async fn initialize_stock_list(state: &Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Check if we already have stocks in the database
    let count = state.db.get_stocks_count().await?;
    
    if count > 0 {
        info!("Stock list already initialized with {} stocks", count);
        return Ok(());
    }

    info!("Initializing stock list for the first time...");
    update_stock_list(state).await
}

async fn update_stock_list(state: &Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let stock_service = StockListService::new();
    
    info!("Fetching all US stocks...");
    
    let stocks = match stock_service.fetch_all_us_stocks().await {
        Ok(stocks) => stocks,
        Err(e) => {
            error!("Failed to fetch stocks from API: {}. Using fallback list...", e);
            stock_service.get_major_us_stocks()
        }
    };

    if stocks.is_empty() {
        error!("No stocks to update");
        return Ok(());
    }

    info!("Updating database with {} stocks", stocks.len());

    let mut successful_inserts = 0;
    let mut failed_inserts = 0;

    for stock in stocks {
        match state.db.create_stock(
            &stock.symbol,
            Some(&stock.name),
            Some(&stock.exchange),
            stock.sector.as_deref(),
            stock.industry.as_deref(),
            stock.market_cap,
        ).await {
            Ok(_) => successful_inserts += 1,
            Err(e) => {
                error!("Failed to insert stock {}: {}", stock.symbol, e);
                failed_inserts += 1;
            }
        }
    }

    info!("Stock list update completed: {} successful, {} failed", successful_inserts, failed_inserts);
    Ok(())
}
