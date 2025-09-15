use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::api::AppState;
use crate::models::{StockListResponse, StockListQuery};
use crate::services::stock_list::StockListService;
use crate::utils::errors::AppResult;

#[derive(Debug, Deserialize)]
pub struct RefreshQuery {
    pub force: Option<bool>,
}

pub async fn list_stocks(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StockListQuery>,
) -> AppResult<Json<StockListResponse>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(100).min(1000); // Max 1000 per page
    let offset = (page - 1) * limit;

    let stocks = state.db.get_all_active_stocks(Some(limit), Some(offset)).await?;
    let total = state.db.get_stocks_count().await?;

    Ok(Json(StockListResponse {
        total,
        stocks,
    }))
}

pub async fn refresh_stock_list(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RefreshQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let stock_service = StockListService::new();
    
    // Check if we should force refresh or if it's been a while since last update
    let should_refresh = query.force.unwrap_or(false) || {
        let count = state.db.get_stocks_count().await?;
        count == 0 // Refresh if no stocks exist
    };

    if !should_refresh {
        return Ok(Json(serde_json::json!({
            "message": "Stock list is up to date",
            "refreshed": false
        })));
    }

    let stocks = match stock_service.fetch_all_us_stocks().await {
        Ok(stocks) => stocks,
        Err(e) => {
            return Ok(Json(serde_json::json!({
                "error": format!("Failed to fetch stocks: {}", e),
                "refreshed": false
            })));
        }
    };

    let mut successful_inserts = 0;
    let mut failed_inserts = 0;

    for stock in stocks {
        match state.db.create_stock(
            &stock.symbol,
            &stock.name,
            &stock.exchange,
            stock.sector.as_deref(),
            stock.industry.as_deref(),
            stock.market_cap,
        ).await {
            Ok(_) => successful_inserts += 1,
            Err(_) => failed_inserts += 1,
        }
    }

    Ok(Json(serde_json::json!({
        "message": "Stock list refreshed successfully",
        "refreshed": true,
        "successful_inserts": successful_inserts,
        "failed_inserts": failed_inserts
    })))
}
