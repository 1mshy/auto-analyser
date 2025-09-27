use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row};
use uuid::Uuid;

use crate::web_api::StockAnalysisResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredAnalysisResult {
    pub id: String,
    pub ticker: String,
    pub name: String,
    pub current_price: Option<f64>,
    pub rsi: Option<f64>,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub macd: Option<f64>,
    pub macd_signal: Option<f64>,
    pub macd_histogram: Option<f64>,
    pub volume: Option<i64>,
    pub pct_change: Option<f64>,
    pub market_cap: Option<String>,
    pub is_opportunity: bool,
    pub signals: String, // JSON array as string
    pub timestamp: DateTime<Utc>,
    pub analysis_session: String,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        tracing::info!("Connecting to database: {}", database_url);
        
        let pool = SqlitePool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(Self { pool })
    }

    pub async fn initialize_tables(&self) -> Result<()> {
        tracing::info!("Initializing database tables");
        
        let query = r#"
        CREATE TABLE IF NOT EXISTS analysis_results (
            id TEXT PRIMARY KEY,
            ticker TEXT NOT NULL,
            name TEXT NOT NULL,
            current_price REAL,
            rsi REAL,
            sma_20 REAL,
            sma_50 REAL,
            macd REAL,
            macd_signal REAL,
            macd_histogram REAL,
            volume INTEGER,
            pct_change REAL,
            market_cap TEXT,
            is_opportunity INTEGER NOT NULL,
            signals TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            analysis_session TEXT NOT NULL,
            UNIQUE(ticker, analysis_session)
        );
        
        CREATE INDEX IF NOT EXISTS idx_ticker ON analysis_results(ticker);
        CREATE INDEX IF NOT EXISTS idx_timestamp ON analysis_results(timestamp);
        CREATE INDEX IF NOT EXISTS idx_session ON analysis_results(analysis_session);
        CREATE INDEX IF NOT EXISTS idx_opportunity ON analysis_results(is_opportunity);
        CREATE INDEX IF NOT EXISTS idx_rsi ON analysis_results(rsi);
        "#;
        
        sqlx::query(query).execute(&self.pool).await?;
        
        tracing::info!("Database tables initialized successfully");
        Ok(())
    }

    pub async fn store_analysis_result(&self, result: &StockAnalysisResult, session: &str) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let signals_json = serde_json::to_string(&result.signals)?;
        
        let query = r#"
        INSERT OR REPLACE INTO analysis_results (
            id, ticker, name, current_price, rsi, sma_20, sma_50, macd, macd_signal, 
            macd_histogram, volume, pct_change, market_cap, is_opportunity, signals, 
            timestamp, analysis_session
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;
        
        sqlx::query(query)
            .bind(id)
            .bind(&result.ticker)
            .bind(&result.name)
            .bind(result.current_price)
            .bind(result.rsi)
            .bind(result.sma_20)
            .bind(result.sma_50)
            .bind(result.macd)
            .bind(result.macd_signal)
            .bind(result.macd_histogram)
            .bind(result.volume.map(|v| v as i64))
            .bind(result.pct_change)
            .bind(&result.market_cap)
            .bind(result.is_opportunity)
            .bind(signals_json)
            .bind(result.timestamp.to_rfc3339())
            .bind(session)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_latest_results(&self, limit: Option<i32>) -> Result<Vec<StockAnalysisResult>> {
        let query = if let Some(limit) = limit {
            format!(
                r#"
                SELECT * FROM analysis_results 
                WHERE timestamp = (
                    SELECT MAX(timestamp) FROM analysis_results WHERE ticker = analysis_results.ticker
                )
                ORDER BY timestamp DESC 
                LIMIT {}
                "#,
                limit
            )
        } else {
            r#"
            SELECT * FROM analysis_results 
            WHERE timestamp = (
                SELECT MAX(timestamp) FROM analysis_results WHERE ticker = analysis_results.ticker
            )
            ORDER BY timestamp DESC
            "#.to_string()
        };

        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;
        
        let mut results = Vec::new();
        for row in rows {
            let signals_json: String = row.get("signals");
            let signals: Vec<String> = serde_json::from_str(&signals_json)?;
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?
                .with_timezone(&Utc);

            results.push(StockAnalysisResult {
                ticker: row.get("ticker"),
                name: row.get("name"),
                current_price: row.get("current_price"),
                rsi: row.get("rsi"),
                sma_20: row.get("sma_20"),
                sma_50: row.get("sma_50"),
                macd: row.get("macd"),
                macd_signal: row.get("macd_signal"),
                macd_histogram: row.get("macd_histogram"),
                volume: row.get::<Option<i64>, _>("volume").map(|v| v as u64),
                pct_change: row.get("pct_change"),
                market_cap: row.get("market_cap"),
                is_opportunity: row.get::<i32, _>("is_opportunity") != 0,
                signals,
                timestamp,
            });
        }

        Ok(results)
    }

    pub async fn get_results_by_session(&self, session: &str) -> Result<Vec<StockAnalysisResult>> {
        let query = r#"
        SELECT * FROM analysis_results 
        WHERE analysis_session = ?
        ORDER BY timestamp DESC
        "#;

        let rows = sqlx::query(query)
            .bind(session)
            .fetch_all(&self.pool)
            .await?;
        
        let mut results = Vec::new();
        for row in rows {
            let signals_json: String = row.get("signals");
            let signals: Vec<String> = serde_json::from_str(&signals_json)?;
            let timestamp_str: String = row.get("timestamp");
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?
                .with_timezone(&Utc);

            results.push(StockAnalysisResult {
                ticker: row.get("ticker"),
                name: row.get("name"),
                current_price: row.get("current_price"),
                rsi: row.get("rsi"),
                sma_20: row.get("sma_20"),
                sma_50: row.get("sma_50"),
                macd: row.get("macd"),
                macd_signal: row.get("macd_signal"),
                macd_histogram: row.get("macd_histogram"),
                volume: row.get::<Option<i64>, _>("volume").map(|v| v as u64),
                pct_change: row.get("pct_change"),
                market_cap: row.get("market_cap"),
                is_opportunity: row.get::<i32, _>("is_opportunity") != 0,
                signals,
                timestamp,
            });
        }

        Ok(results)
    }

    pub async fn cleanup_old_results(&self, older_than_days: i32) -> Result<usize> {
        let query = r#"
        DELETE FROM analysis_results 
        WHERE timestamp < datetime('now', '-' || ? || ' days')
        "#;

        let result = sqlx::query(query)
            .bind(older_than_days)
            .execute(&self.pool)
            .await?;

        tracing::info!("Cleaned up {} old analysis results", result.rows_affected());
        Ok(result.rows_affected() as usize)
    }

    pub async fn get_analysis_stats(&self) -> Result<AnalysisStats> {
        let query = r#"
        SELECT 
            COUNT(*) as total_results,
            COUNT(DISTINCT ticker) as unique_tickers,
            COUNT(DISTINCT analysis_session) as total_sessions,
            SUM(CASE WHEN is_opportunity = 1 THEN 1 ELSE 0 END) as opportunities,
            AVG(rsi) as avg_rsi,
            MIN(timestamp) as oldest_result,
            MAX(timestamp) as newest_result
        FROM analysis_results
        "#;

        let row = sqlx::query(query).fetch_one(&self.pool).await?;
        
        let oldest_str: Option<String> = row.get("oldest_result");
        let newest_str: Option<String> = row.get("newest_result");
        
        let oldest_result = if let Some(s) = oldest_str {
            Some(DateTime::parse_from_rfc3339(&s)?.with_timezone(&Utc))
        } else {
            None
        };
        
        let newest_result = if let Some(s) = newest_str {
            Some(DateTime::parse_from_rfc3339(&s)?.with_timezone(&Utc))
        } else {
            None
        };

        Ok(AnalysisStats {
            total_results: row.get::<i32, _>("total_results") as u64,
            unique_tickers: row.get::<i32, _>("unique_tickers") as u64,
            total_sessions: row.get::<i32, _>("total_sessions") as u64,
            opportunities: row.get::<i32, _>("opportunities") as u64,
            avg_rsi: row.get("avg_rsi"),
            oldest_result,
            newest_result,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisStats {
    pub total_results: u64,
    pub unique_tickers: u64,
    pub total_sessions: u64,
    pub opportunities: u64,
    pub avg_rsi: Option<f64>,
    pub oldest_result: Option<DateTime<Utc>>,
    pub newest_result: Option<DateTime<Utc>>,
}