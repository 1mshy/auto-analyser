use axum::{
    extract::{State, Path},
    response::Json,
    Extension,
};
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::api::AppState;
use crate::models::{Alert, CreateAlertRequest};
use crate::utils::errors::AppResult;

pub async fn get_alerts(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
) -> AppResult<Json<Vec<Alert>>> {
    let alerts = state.db.get_alerts(user_id).await?;
    Ok(Json(alerts))
}

pub async fn create_alert(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateAlertRequest>,
) -> AppResult<Json<Alert>> {
    let condition_value = Decimal::from_f64_retain(payload.condition_value)
        .ok_or_else(|| crate::utils::errors::AppError::BadRequest("Invalid condition value".to_string()))?;
    
    let alert = state.db.create_alert(
        user_id,
        &payload.symbol,
        &payload.condition_type,
        condition_value,
    ).await?;
    Ok(Json(alert))
}

pub async fn update_alert(
    State(state): State<Arc<AppState>>,
    Extension(_user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<&'static str>> {
    // For simplicity, we'll toggle the alert status
    // In a real implementation, you'd accept more update parameters
    state.db.update_alert(id, false).await?;
    Ok(Json("Alert updated"))
}

pub async fn delete_alert(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<Uuid>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<&'static str>> {
    state.db.delete_alert(id, user_id).await?;
    Ok(Json("Alert deleted"))
}
