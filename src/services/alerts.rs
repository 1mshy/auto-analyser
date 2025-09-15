use rust_decimal::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{info, error};

use crate::api::AppState;
use crate::services::indicators::IndicatorService;

pub async fn start_alert_evaluator(state: Arc<AppState>) {
    let mut interval_timer = interval(Duration::from_secs(state.config.alert_check_interval_seconds));
    
    info!("Starting alert evaluator with interval: {}s", state.config.alert_check_interval_seconds);

    loop {
        interval_timer.tick().await;
        
        match evaluate_alerts(&state).await {
            Ok(_) => info!("Alert evaluation completed successfully"),
            Err(e) => error!("Alert evaluation failed: {}", e),
        }
    }
}

async fn evaluate_alerts(state: &Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let alerts = state.db.get_active_alerts().await?;
    
    if alerts.is_empty() {
        return Ok(());
    }

    info!("Evaluating {} active alerts", alerts.len());

    let indicator_service = IndicatorService::new();

    for alert in alerts {
        match evaluate_single_alert(state, &indicator_service, &alert).await {
            Ok(triggered) => {
                if triggered {
                    info!("Alert {} triggered for {} {}", alert.id, alert.symbol, alert.condition_type);
                }
            }
            Err(e) => {
                error!("Failed to evaluate alert {}: {}", alert.id, e);
            }
        }
    }

    Ok(())
}

async fn evaluate_single_alert(
    state: &Arc<AppState>,
    indicator_service: &IndicatorService,
    alert: &crate::models::Alert,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let latest_data = state.db.get_latest_market_data(&alert.symbol).await?;
    
    let Some(data) = latest_data else {
        return Ok(false);
    };

    let triggered = match alert.condition_type.as_str() {
        "price_above" => data.close > alert.condition_value,
        "price_below" => data.close < alert.condition_value,
        "rsi_above" | "rsi_below" => {
            let historical_data = state.db.get_historical_data(&alert.symbol, 100).await?;
            let indicators = indicator_service.calculate_indicators(&alert.symbol, &historical_data).await?;
            
            match (alert.condition_type.as_str(), indicators.rsi_14) {
                ("rsi_above", Some(rsi)) => {
                    let rsi_decimal = rust_decimal::Decimal::from_f64_retain(rsi).unwrap_or_default();
                    rsi_decimal > alert.condition_value
                },
                ("rsi_below", Some(rsi)) => {
                    let rsi_decimal = rust_decimal::Decimal::from_f64_retain(rsi).unwrap_or_default();
                    rsi_decimal < alert.condition_value
                },
                _ => false,
            }
        }
        _ => false,
    };

    if triggered {
        let message = format!(
            "Alert triggered: {} {} {}",
            alert.symbol,
            alert.condition_type,
            alert.condition_value
        );

        state.db.create_alert_trigger(alert.id, data.close, &message).await?;
        
        // Disable the alert after triggering (optional - you might want to keep it active)
        state.db.update_alert(alert.id, false).await?;
    }

    Ok(triggered)
}
