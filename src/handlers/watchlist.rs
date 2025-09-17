use axum::{
    extract::{State, Path},
    response::Json,
    Extension,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::AppState;
use crate::models::{WatchlistItem, AddToWatchlistRequest, StockPriority};
use crate::utils::errors::AppResult;
use crate::services::priority_scheduler::PriorityScheduler;

pub async fn get_watchlist(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> AppResult<Json<Vec<WatchlistItem>>> {
    let watchlist = state.db.get_watchlist(user_id).await?;
    Ok(Json(watchlist))
}

pub async fn add_to_watchlist(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<AddToWatchlistRequest>,
) -> AppResult<Json<WatchlistItem>> {
    let item = state.db.add_to_watchlist(user_id, &payload.symbol).await?;
    
    // Set high priority for the stock since it's now on a watchlist
    if let Err(e) = state.db.set_stock_priority(&payload.symbol, StockPriority::High).await {
        tracing::warn!("Failed to set high priority for watchlist stock {}: {}", payload.symbol, e);
    }
    
    // Trigger immediate update for the newly added stock
    tokio::spawn({
        let state = state.clone();
        let symbol = payload.symbol.clone();
        async move {
            if let Err(e) = PriorityScheduler::trigger_immediate_update(&state, &symbol).await {
                tracing::error!("Failed to trigger immediate update for {}: {}", symbol, e);
            }
        }
    });
    
    Ok(Json(item))
}

pub async fn remove_from_watchlist(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(symbol): Path<String>,
) -> AppResult<Json<&'static str>> {
    state.db.remove_from_watchlist(user_id, &symbol).await?;
    
    // Check if this stock is still in any watchlist, if not reduce its priority
    let watchlist_symbols = state.db.get_watchlist_symbols().await.unwrap_or_default();
    if !watchlist_symbols.contains(&symbol) {
        if let Err(e) = state.db.set_stock_priority(&symbol, StockPriority::Medium).await {
            tracing::warn!("Failed to reduce priority for removed watchlist stock {}: {}", symbol, e);
        }
    }
    
    Ok(Json("Symbol removed from watchlist"))
}
