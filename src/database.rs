use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{User, WatchlistItem, Alert, AlertTrigger, MarketData, Stock};
use crate::utils::errors::AppResult;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> AppResult<Self> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> AppResult<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    // User operations
    pub async fn create_user(&self, email: &str, password_hash: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING *
            "#,
            Uuid::new_v4(),
            email,
            password_hash,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // Watchlist operations
    pub async fn get_watchlist(&self, user_id: Uuid) -> AppResult<Vec<WatchlistItem>> {
        let items = sqlx::query_as!(
            WatchlistItem,
            "SELECT * FROM watchlist WHERE user_id = $1 ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn add_to_watchlist(&self, user_id: Uuid, symbol: &str) -> AppResult<WatchlistItem> {
        let item = sqlx::query_as!(
            WatchlistItem,
            r#"
            INSERT INTO watchlist (id, user_id, symbol, created_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, symbol) DO NOTHING
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            symbol,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn remove_from_watchlist(&self, user_id: Uuid, symbol: &str) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM watchlist WHERE user_id = $1 AND symbol = $2",
            user_id,
            symbol
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Alert operations
    pub async fn get_alerts(&self, user_id: Uuid) -> AppResult<Vec<Alert>> {
        let alerts = sqlx::query_as!(
            Alert,
            "SELECT * FROM alerts WHERE user_id = $1 ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    pub async fn get_active_alerts(&self) -> AppResult<Vec<Alert>> {
        let alerts = sqlx::query_as!(
            Alert,
            "SELECT * FROM alerts WHERE is_active = true"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(alerts)
    }

    pub async fn create_alert(
        &self,
        user_id: Uuid,
        symbol: &str,
        condition_type: &str,
        condition_value: Decimal,
    ) -> AppResult<Alert> {
        let alert = sqlx::query_as!(
            Alert,
            r#"
            INSERT INTO alerts (id, user_id, symbol, condition_type, condition_value, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, true, $6, $6)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            symbol,
            condition_type,
            condition_value,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(alert)
    }

    pub async fn update_alert(&self, id: Uuid, is_active: bool) -> AppResult<()> {
        sqlx::query!(
            "UPDATE alerts SET is_active = $1, updated_at = $2 WHERE id = $3",
            is_active,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_alert(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query!(
            "DELETE FROM alerts WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_alert_trigger(
        &self,
        alert_id: Uuid,
        trigger_value: Decimal,
        message: &str,
    ) -> AppResult<AlertTrigger> {
        let trigger = sqlx::query_as!(
            AlertTrigger,
            r#"
            INSERT INTO alert_triggers (id, alert_id, triggered_at, trigger_value, message)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            Uuid::new_v4(),
            alert_id,
            Utc::now(),
            trigger_value,
            message
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(trigger)
    }

    // Market data operations
    pub async fn store_market_data(&self, data: &MarketData) -> AppResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO market_data (id, symbol, timestamp, open, high, low, close, volume, created_at, last_updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (symbol, timestamp) DO UPDATE SET
                open = EXCLUDED.open,
                high = EXCLUDED.high,
                low = EXCLUDED.low,
                close = EXCLUDED.close,
                volume = EXCLUDED.volume,
                created_at = EXCLUDED.created_at,
                last_updated = EXCLUDED.last_updated
            "#,
            data.id,
            data.symbol,
            data.timestamp,
            data.open,
            data.high,
            data.low,
            data.close,
            data.volume,
            data.created_at,
            data.last_updated.unwrap_or_else(|| Utc::now())
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_market_data(&self, symbol: &str) -> AppResult<Option<MarketData>> {
        let data = sqlx::query_as!(
            MarketData,
            "SELECT * FROM market_data WHERE symbol = $1 ORDER BY timestamp DESC LIMIT 1",
            symbol
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(data)
    }

    pub async fn get_historical_data(
        &self,
        symbol: &str,
        limit: i64,
    ) -> AppResult<Vec<MarketData>> {
        let data = sqlx::query_as!(
            MarketData,
            "SELECT * FROM market_data WHERE symbol = $1 ORDER BY timestamp DESC LIMIT $2",
            symbol,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(data)
    }

    pub async fn get_symbols_to_update(&self) -> AppResult<Vec<String>> {
        let symbols = sqlx::query!(
            r#"
            SELECT DISTINCT symbol FROM (
                SELECT symbol FROM watchlist
                UNION
                SELECT symbol FROM alerts WHERE is_active = true
            ) AS all_symbols
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(symbols.into_iter().filter_map(|row| row.symbol).collect())
    }

    // Stock operations
    pub async fn create_stock(
        &self,
        symbol: &str,
        name: &str,
        exchange: &str,
        sector: Option<&str>,
        industry: Option<&str>,
        market_cap: Option<i64>,
    ) -> AppResult<Stock> {
        let stock = sqlx::query_as!(
            Stock,
            r#"
            INSERT INTO stocks (id, symbol, name, exchange, sector, industry, market_cap, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, true, $8, $8)
            ON CONFLICT (symbol) DO UPDATE SET
                name = EXCLUDED.name,
                exchange = EXCLUDED.exchange,
                sector = EXCLUDED.sector,
                industry = EXCLUDED.industry,
                market_cap = EXCLUDED.market_cap,
                updated_at = EXCLUDED.updated_at
            RETURNING *
            "#,
            Uuid::new_v4(),
            symbol,
            name,
            exchange,
            sector,
            industry,
            market_cap,
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(stock)
    }

    pub async fn get_all_active_stocks(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<Stock>> {
        let limit = limit.unwrap_or(1000);
        let offset = offset.unwrap_or(0);

        let stocks = sqlx::query_as!(
            Stock,
            "SELECT * FROM stocks WHERE is_active = true ORDER BY symbol LIMIT $1 OFFSET $2",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(stocks)
    }

    pub async fn get_stocks_count(&self) -> AppResult<i64> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM stocks WHERE is_active = true"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count.unwrap_or(0))
    }

    pub async fn get_all_stock_symbols(&self) -> AppResult<Vec<String>> {
        let symbols = sqlx::query!(
            "SELECT symbol FROM stocks WHERE is_active = true ORDER BY symbol"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(symbols.into_iter().map(|row| row.symbol).collect())
    }

    pub async fn update_stock_status(&self, symbol: &str, is_active: bool) -> AppResult<()> {
        sqlx::query!(
            "UPDATE stocks SET is_active = $1, updated_at = $2 WHERE symbol = $3",
            is_active,
            Utc::now(),
            symbol
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
