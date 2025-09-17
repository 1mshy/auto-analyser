use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::models::{User, WatchlistItem, Alert, AlertTrigger, MarketData, Stock, StockPriority};
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

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn migrate(&self) -> AppResult<()> {
        // TODO: Enable migrations once SQLx setup is fixed
        // sqlx::migrate!("./migrations").run(&self.pool).await?;
        tracing::warn!("Database migrations are temporarily disabled");
        Ok(())
    }

    // User operations
    pub async fn create_user(&self, email: &str, password_hash: &str) -> AppResult<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id, email, password_hash, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let row = sqlx::query("SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let row = sqlx::query("SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| User {
            id: r.get("id"),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }))
    }

    // Watchlist operations
    pub async fn get_watchlist(&self, user_id: Uuid) -> AppResult<Vec<WatchlistItem>> {
        let rows = sqlx::query("SELECT id, user_id, symbol, created_at FROM watchlist WHERE user_id = $1 ORDER BY created_at DESC")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let items = rows.into_iter().map(|r| WatchlistItem {
            id: r.get("id"),
            user_id: r.get("user_id"),
            symbol: r.get("symbol"),
            created_at: r.get("created_at"),
        }).collect();

        Ok(items)
    }

    pub async fn add_to_watchlist(&self, user_id: Uuid, symbol: &str) -> AppResult<WatchlistItem> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query(
            r#"
            INSERT INTO watchlist (id, user_id, symbol, created_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id, symbol) DO NOTHING
            RETURNING id, user_id, symbol, created_at
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(symbol)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(WatchlistItem {
            id: row.get("id"),
            user_id: row.get("user_id"),
            symbol: row.get("symbol"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn remove_from_watchlist(&self, user_id: Uuid, symbol: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM watchlist WHERE user_id = $1 AND symbol = $2")
            .bind(user_id)
            .bind(symbol)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Alert operations
    pub async fn get_user_alerts(&self, user_id: Uuid) -> AppResult<Vec<Alert>> {
        let rows = sqlx::query(
            "SELECT id, user_id, symbol, condition_type, condition_value, is_active, created_at, updated_at FROM alerts WHERE user_id = $1 ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let alerts = rows.into_iter().map(|r| Alert {
            id: r.get("id"),
            user_id: r.get("user_id"),
            symbol: r.get("symbol"),
            condition_type: r.get("condition_type"),
            condition_value: r.get("condition_value"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect();

        Ok(alerts)
    }

    pub async fn get_active_alerts(&self) -> AppResult<Vec<Alert>> {
        let rows = sqlx::query(
            "SELECT id, user_id, symbol, condition_type, condition_value, is_active, created_at, updated_at FROM alerts WHERE is_active = true"
        )
        .fetch_all(&self.pool)
        .await?;

        let alerts = rows.into_iter().map(|r| Alert {
            id: r.get("id"),
            user_id: r.get("user_id"),
            symbol: r.get("symbol"),
            condition_type: r.get("condition_type"),
            condition_value: r.get("condition_value"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect();

        Ok(alerts)
    }

    pub async fn create_alert(
        &self,
        user_id: Uuid,
        symbol: &str,
        condition_type: &str,
        condition_value: Decimal,
    ) -> AppResult<Alert> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query(
            r#"
            INSERT INTO alerts (id, user_id, symbol, condition_type, condition_value, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, true, $6, $6)
            RETURNING id, user_id, symbol, condition_type, condition_value, is_active, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(symbol)
        .bind(condition_type)
        .bind(condition_value)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Alert {
            id: row.get("id"),
            user_id: row.get("user_id"),
            symbol: row.get("symbol"),
            condition_type: row.get("condition_type"),
            condition_value: row.get("condition_value"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn update_alert_status(&self, id: Uuid, is_active: bool) -> AppResult<()> {
        sqlx::query("UPDATE alerts SET is_active = $1, updated_at = $2 WHERE id = $3")
            .bind(is_active)
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_alert(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM alerts WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
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
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query(
            r#"
            INSERT INTO alert_triggers (id, alert_id, triggered_at, trigger_value, message)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, alert_id, triggered_at, trigger_value, message
            "#
        )
        .bind(id)
        .bind(alert_id)
        .bind(now)
        .bind(trigger_value)
        .bind(message)
        .fetch_one(&self.pool)
        .await?;

        Ok(AlertTrigger {
            id: row.get("id"),
            alert_id: row.get("alert_id"),
            triggered_at: row.get("triggered_at"),
            trigger_value: row.get("trigger_value"),
            message: row.get("message"),
        })
    }

    // Market data operations
    pub async fn store_market_data(&self, data: &MarketData) -> AppResult<()> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO market_data (id, symbol, timestamp, open, high, low, close, volume, created_at, last_updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (symbol, timestamp) DO UPDATE SET
                open = EXCLUDED.open,
                high = EXCLUDED.high,
                low = EXCLUDED.low,
                close = EXCLUDED.close,
                volume = EXCLUDED.volume,
                last_updated = EXCLUDED.last_updated
            "#
        )
        .bind(id)
        .bind(&data.symbol)
        .bind(data.timestamp)
        .bind(data.open)
        .bind(data.high)
        .bind(data.low)
        .bind(data.close)
        .bind(data.volume)
        .bind(now)
        .bind(data.last_updated.unwrap_or_else(|| Utc::now()))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_latest_market_data(&self, symbol: &str) -> AppResult<Option<MarketData>> {
        let row = sqlx::query(
            "SELECT id, symbol, timestamp, open, high, low, close, volume, created_at, last_updated FROM market_data WHERE symbol = $1 ORDER BY timestamp DESC LIMIT 1"
        )
        .bind(symbol)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| MarketData {
            id: r.get("id"),
            symbol: r.get("symbol"),
            timestamp: r.get("timestamp"),
            open: r.get("open"),
            high: r.get("high"),
            low: r.get("low"),
            close: r.get("close"),
            volume: r.get("volume"),
            created_at: r.get("created_at"),
            last_updated: r.get("last_updated"),
        }))
    }

    pub async fn get_market_data_history(&self, symbol: &str, limit: i64) -> AppResult<Vec<MarketData>> {
        let rows = sqlx::query(
            "SELECT id, symbol, timestamp, open, high, low, close, volume, created_at, last_updated FROM market_data WHERE symbol = $1 ORDER BY timestamp DESC LIMIT $2"
        )
        .bind(symbol)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let data = rows.into_iter().map(|r| MarketData {
            id: r.get("id"),
            symbol: r.get("symbol"),
            timestamp: r.get("timestamp"),
            open: r.get("open"),
            high: r.get("high"),
            low: r.get("low"),
            close: r.get("close"),
            volume: r.get("volume"),
            created_at: r.get("created_at"),
            last_updated: r.get("last_updated"),
        }).collect();

        Ok(data)
    }

    pub async fn get_tracked_symbols(&self) -> AppResult<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT symbol FROM (
                SELECT symbol FROM watchlist
                UNION
                SELECT symbol FROM alerts WHERE is_active = true
            ) AS symbols
            ORDER BY symbol
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let symbols = rows.into_iter().map(|r| r.get::<String, _>("symbol")).collect();
        Ok(symbols)
    }

    // Stock operations
    pub async fn upsert_stock(
        &self,
        symbol: &str,
        name: Option<&str>,
        exchange: Option<&str>,
        sector: Option<&str>,
        industry: Option<&str>,
        market_cap: Option<i64>,
    ) -> AppResult<Stock> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query(
            r#"
            INSERT INTO stocks (id, symbol, name, exchange, sector, industry, market_cap, is_active, delisting_reason, last_error, priority, price_update_interval_seconds, last_price_update, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, true, NULL, NULL, 'medium', 300, NULL, $8, $8)
            ON CONFLICT (symbol) DO UPDATE SET
                name = COALESCE(EXCLUDED.name, stocks.name),
                exchange = COALESCE(EXCLUDED.exchange, stocks.exchange),
                sector = COALESCE(EXCLUDED.sector, stocks.sector),
                industry = COALESCE(EXCLUDED.industry, stocks.industry),
                market_cap = COALESCE(EXCLUDED.market_cap, stocks.market_cap),
                updated_at = EXCLUDED.updated_at
            RETURNING id, symbol, name, exchange, sector, industry, market_cap, is_active, delisting_reason, last_error, priority, price_update_interval_seconds, last_price_update, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(symbol)
        .bind(name)
        .bind(exchange)
        .bind(sector)
        .bind(industry)
        .bind(market_cap)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Stock {
            id: row.get("id"),
            symbol: row.get("symbol"),
            name: row.get("name"),
            exchange: row.get("exchange"),
            sector: row.get("sector"),
            industry: row.get("industry"),
            market_cap: row.get("market_cap"),
            is_active: row.get("is_active"),
            delisting_reason: row.get("delisting_reason"),
            last_error: row.get("last_error"),
            priority: row.get("priority"),
            price_update_interval_seconds: row.get("price_update_interval_seconds"),
            last_price_update: row.get("last_price_update"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_stocks(&self, limit: i64, offset: i64) -> AppResult<Vec<Stock>> {
        let rows = sqlx::query(
            "SELECT id, symbol, name, exchange, sector, industry, market_cap, is_active, delisting_reason, last_error, priority, price_update_interval_seconds, last_price_update, created_at, updated_at FROM stocks WHERE is_active = true ORDER BY symbol LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let stocks = rows.into_iter().map(|r| Stock {
            id: r.get("id"),
            symbol: r.get("symbol"),
            name: r.get("name"),
            exchange: r.get("exchange"),
            sector: r.get("sector"),
            industry: r.get("industry"),
            market_cap: r.get("market_cap"),
            is_active: r.get("is_active"),
            delisting_reason: r.get("delisting_reason"),
            last_error: r.get("last_error"),
            priority: r.get("priority"),
            price_update_interval_seconds: r.get("price_update_interval_seconds"),
            last_price_update: r.get("last_price_update"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect();

        Ok(stocks)
    }

    pub async fn get_stock_count(&self) -> AppResult<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM stocks WHERE is_active = true")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i64, _>("count"))
    }

    pub async fn get_all_active_symbols(&self) -> AppResult<Vec<String>> {
        let rows = sqlx::query("SELECT symbol FROM stocks WHERE is_active = true ORDER BY symbol")
            .fetch_all(&self.pool)
            .await?;

        let symbols = rows.into_iter().map(|r| r.get::<String, _>("symbol")).collect();
        Ok(symbols)
    }

    pub async fn update_stock_status(&self, symbol: &str, is_active: bool) -> AppResult<()> {
        sqlx::query("UPDATE stocks SET is_active = $1, updated_at = $2 WHERE symbol = $3")
            .bind(is_active)
            .bind(Utc::now())
            .bind(symbol)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_symbols_by_priority(&self, priority: StockPriority) -> AppResult<Vec<String>> {
        let rows = sqlx::query("SELECT symbol FROM stocks WHERE is_active = true AND priority = $1 ORDER BY symbol")
            .bind(priority)
            .fetch_all(&self.pool)
            .await?;

        let symbols = rows.into_iter().map(|r| r.get::<String, _>("symbol")).collect();
        Ok(symbols)
    }

    pub async fn get_symbols_for_price_update(&self, priority: StockPriority, interval_seconds: i32) -> AppResult<Vec<String>> {
        let rows = sqlx::query(
            r#"
            SELECT symbol FROM stocks 
            WHERE is_active = true 
            AND priority = $1 
            AND (
                last_price_update IS NULL 
                OR last_price_update < NOW() - INTERVAL '1 second' * $2
            )
            ORDER BY symbol
            "#
        )
        .bind(priority)
        .bind(interval_seconds)
        .fetch_all(&self.pool)
        .await?;

        let symbols = rows.into_iter().map(|r| r.get::<String, _>("symbol")).collect();
        Ok(symbols)
    }

    pub async fn update_stock_price_timestamp(&self, symbol: &str) -> AppResult<()> {
        let now = Utc::now();
        sqlx::query("UPDATE stocks SET last_price_update = $1, updated_at = $1 WHERE symbol = $2")
            .bind(now)
            .bind(symbol)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_stock_priority_and_interval(
        &self,
        symbol: &str,
        priority: StockPriority,
        interval_seconds: i32,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE stocks SET priority = $1, price_update_interval_seconds = $2, updated_at = $3 WHERE symbol = $4"
        )
        .bind(priority)
        .bind(interval_seconds)
        .bind(Utc::now())
        .bind(symbol)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_watchlist_symbols(&self) -> AppResult<Vec<String>> {
        let rows = sqlx::query("SELECT DISTINCT symbol FROM watchlist ORDER BY symbol")
            .fetch_all(&self.pool)
            .await?;

        let symbols = rows.into_iter().map(|r| r.get::<String, _>("symbol")).collect();
        Ok(symbols)
    }
}
