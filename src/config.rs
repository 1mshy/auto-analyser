use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub market_data_interval_seconds: u64,
    pub alert_check_interval_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:password@localhost/equity_analyser".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            market_data_interval_seconds: env::var("MARKET_DATA_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "300".to_string()) // 5 minutes
                .parse()?,
            alert_check_interval_seconds: env::var("ALERT_CHECK_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "60".to_string()) // 1 minute
                .parse()?,
        })
    }
}
