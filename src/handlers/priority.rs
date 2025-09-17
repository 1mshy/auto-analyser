use axum::{
    extract::{Query, State},
    response::Json,
};
use std::sync::Arc;
use serde::Deserialize;

use crate::api::AppState;
use crate::models::StockPriority;
use crate::utils::errors::AppResult;

#[derive(Debug, Deserialize)]
pub struct PriorityQuery {
    pub priority: Option<String>,
}

pub async fn get_stocks_by_priority(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PriorityQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let priority = match query.priority.as_deref() {
        Some("high") => StockPriority::High,
        Some("medium") => StockPriority::Medium,
        Some("low") => StockPriority::Low,
        _ => {
            // Return all priorities with counts
            let high_symbols = state.db.get_stocks_by_priority(StockPriority::High).await?;
            let medium_symbols = state.db.get_stocks_by_priority(StockPriority::Medium).await?;
            let low_symbols = state.db.get_stocks_by_priority(StockPriority::Low).await?;
            
            let response = serde_json::json!({
                "priorities": {
                    "high": {
                        "count": high_symbols.len(),
                        "update_interval_seconds": StockPriority::High.update_interval_seconds(),
                        "symbols": high_symbols
                    },
                    "medium": {
                        "count": medium_symbols.len(),
                        "update_interval_seconds": StockPriority::Medium.update_interval_seconds(),
                        "symbols": medium_symbols
                    },
                    "low": {
                        "count": low_symbols.len(),
                        "update_interval_seconds": StockPriority::Low.update_interval_seconds(),
                        "symbols": low_symbols
                    }
                }
            });
            
            return Ok(Json(response));
        }
    };

    let symbols = state.db.get_stocks_by_priority(priority).await?;
    let response = serde_json::json!({
        "priority": format!("{:?}", priority).to_lowercase(),
        "count": symbols.len(),
        "update_interval_seconds": priority.update_interval_seconds(),
        "symbols": symbols
    });

    Ok(Json(response))
}

pub async fn get_stocks_needing_update(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PriorityQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let priority = match query.priority.as_deref() {
        Some("high") => StockPriority::High,
        Some("medium") => StockPriority::Medium,
        Some("low") => StockPriority::Low,
        _ => {
            // Return all priorities with counts of stocks needing updates
            let high_symbols = state.db.get_stocks_needing_update(StockPriority::High).await?;
            let medium_symbols = state.db.get_stocks_needing_update(StockPriority::Medium).await?;
            let low_symbols = state.db.get_stocks_needing_update(StockPriority::Low).await?;
            
            let response = serde_json::json!({
                "priorities": {
                    "high": {
                        "needing_update": high_symbols.len(),
                        "symbols": high_symbols
                    },
                    "medium": {
                        "needing_update": medium_symbols.len(),
                        "symbols": medium_symbols
                    },
                    "low": {
                        "needing_update": low_symbols.len(),
                        "symbols": low_symbols
                    }
                }
            });
            
            return Ok(Json(response));
        }
    };

    let symbols = state.db.get_stocks_needing_update(priority).await?;
    let response = serde_json::json!({
        "priority": format!("{:?}", priority).to_lowercase(),
        "needing_update": symbols.len(),
        "symbols": symbols
    });

    Ok(Json(response))
}
