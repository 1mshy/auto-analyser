use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error};

use crate::api::AppState;
use crate::services::market_data::MarketDataService;

pub async fn start_market_data_scheduler(state: Arc<AppState>) {
    let mut interval_timer = interval(Duration::from_secs(state.config.market_data_interval_seconds));
    
    info!("Starting market data scheduler with interval: {}s", state.config.market_data_interval_seconds);

    loop {
        interval_timer.tick().await;
        
        match update_market_data(&state).await {
            Ok(_) => info!("Market data update completed successfully"),
            Err(e) => error!("Market data update failed: {}", e),
        }
    }
}

async fn update_market_data(state: &Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let symbols = state.db.get_symbols_to_update().await?;
    
    if symbols.is_empty() {
        return Ok(());
    }

    info!("Updating market data for {} symbols", symbols.len());

    let market_service = MarketDataService::new();

    for symbol in symbols {
        match market_service.fetch_historical_data(&symbol, "1d").await {
            Ok(data) => {
                for market_data in data {
                    if let Err(e) = state.db.store_market_data(&market_data).await {
                        error!("Failed to store market data for {}: {}", symbol, e);
                    }
                }
                info!("Updated market data for {}", symbol);
            }
            Err(e) => {
                error!("Failed to fetch market data for {}: {}", symbol, e);
            }
        }

        // Small delay to avoid rate limiting
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
