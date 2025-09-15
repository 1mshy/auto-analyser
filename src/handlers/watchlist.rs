use axum::{
    extract::{State, Path},
    response::Json,
    Extension,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::AppState;
use crate::models::{WatchlistItem, AddToWatchlistRequest};
use crate::utils::errors::AppResult;

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
    Ok(Json(item))
}

pub async fn remove_from_watchlist(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(symbol): Path<String>,
) -> AppResult<Json<&'static str>> {
    state.db.remove_from_watchlist(user_id, &symbol).await?;
    Ok(Json("Symbol removed from watchlist"))
}
