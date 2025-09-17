use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::api::AppState;
use crate::models::{QuoteResponse, HistoricalDataResponse, IndicatorsResponse};
use crate::services::market_data::MarketDataService;
use crate::services::indicators::IndicatorService;
use crate::utils::errors::AppResult;

#[derive(Debug, Deserialize)]
pub struct HistoricalQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    100
}

pub async fn get_quote(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
) -> AppResult<Json<QuoteResponse>> {
    let market_service = MarketDataService::new();
    let quote = market_service.fetch_current_quote_with_delisting_check(&symbol, state.db.pool()).await?;
    Ok(Json(quote))
}

pub async fn get_historical(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
    Query(query): Query<HistoricalQuery>,
) -> AppResult<Json<HistoricalDataResponse>> {
    let data = state.db.get_historical_data(&symbol, query.limit).await?;
    
    Ok(Json(HistoricalDataResponse {
        symbol,
        data,
    }))
}

pub async fn get_indicators(
    State(state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
) -> AppResult<Json<IndicatorsResponse>> {
    let data = state.db.get_historical_data(&symbol, 200).await?;
    let indicator_service = IndicatorService::new();
    let indicators = indicator_service.calculate_indicators(&symbol, &data).await?;
    
    Ok(Json(IndicatorsResponse {
        symbol,
        indicators,
    }))
}
