use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WatchlistItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Alert {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub condition_type: String, // 'price_above', 'price_below', 'rsi_above', 'rsi_below', etc.
    pub condition_value: Decimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AlertTrigger {
    pub id: Uuid,
    pub alert_id: Uuid,
    pub triggered_at: DateTime<Utc>,
    pub trigger_value: Decimal,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MarketData {
    pub id: Uuid,
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: i64,
    pub created_at: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalIndicators {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub ema_12: Option<f64>,
    pub ema_26: Option<f64>,
    pub rsi_14: Option<f64>,
    pub macd: Option<f64>,
    pub macd_signal: Option<f64>,
    pub bollinger_upper: Option<f64>,
    pub bollinger_lower: Option<f64>,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAlertRequest {
    pub symbol: String,
    pub condition_type: String,
    pub condition_value: f64,
}

#[derive(Debug, Deserialize)]
pub struct AddToWatchlistRequest {
    pub symbol: String,
}

#[derive(Debug, Serialize)]
pub struct QuoteResponse {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
    pub change_percent: f64,
    pub volume: i64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct HistoricalDataResponse {
    pub symbol: String,
    pub data: Vec<MarketData>,
}

#[derive(Debug, Serialize)]
pub struct IndicatorsResponse {
    pub symbol: String,
    pub indicators: TechnicalIndicators,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Stock {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub sector: Option<String>,
    pub industry: Option<String>,
    pub market_cap: Option<i64>,
    pub is_active: bool,
    pub delisting_reason: Option<String>,
    pub last_error_at: Option<DateTime<Utc>>,
    pub last_error_message: Option<String>,
    pub priority: StockPriority,
    pub last_price_update: Option<DateTime<Utc>>,
    pub price_update_interval_seconds: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StockListResponse {
    pub total: i64,
    pub stocks: Vec<Stock>,
}

#[derive(Debug, Deserialize)]
pub struct StockListQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sector: Option<String>,
    pub exchange: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "stock_priority", rename_all = "lowercase")]
pub enum StockPriority {
    High,
    Medium,
    Low,
}

impl StockPriority {
    pub fn update_interval_seconds(&self) -> u64 {
        match self {
            StockPriority::High => 60,    // 1 minute for watchlist stocks
            StockPriority::Medium => 300, // 5 minutes for actively traded stocks
            StockPriority::Low => 900,    // 15 minutes for other stocks
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityMarketUpdate {
    pub symbols: Vec<String>,
    pub priority: StockPriority,
    pub last_updated: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}
