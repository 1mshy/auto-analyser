use rust_decimal::prelude::*;
use ta::indicators::{
    SimpleMovingAverage, ExponentialMovingAverage, RelativeStrengthIndex,
    MovingAverageConvergenceDivergence, BollingerBands
};
use ta::{Next};

use crate::models::{MarketData, TechnicalIndicators};
use crate::utils::errors::AppResult;

pub struct IndicatorService;

impl IndicatorService {
    pub fn new() -> Self {
        Self
    }

    pub async fn calculate_indicators(
        &self,
        symbol: &str,
        data: &[MarketData],
    ) -> AppResult<TechnicalIndicators> {
        if data.is_empty() {
            return Ok(TechnicalIndicators {
                symbol: symbol.to_string(),
                timestamp: chrono::Utc::now(),
                sma_20: None,
                sma_50: None,
                ema_12: None,
                ema_26: None,
                rsi_14: None,
                macd: None,
                macd_signal: None,
                bollinger_upper: None,
                bollinger_lower: None,
            });
        }

        // Sort data by timestamp (oldest first for calculation)
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Extract closing prices
        let closes: Vec<f64> = sorted_data.iter()
            .map(|d| d.close.to_f64().unwrap_or(0.0))
            .collect();

        // Calculate indicators
        let sma_20 = if closes.len() >= 20 {
            let mut sma = SimpleMovingAverage::new(20).unwrap();
            for price in &closes {
                sma.next(*price);
            }
            Some(sma.next(closes[closes.len() - 1]))
        } else {
            None
        };

        let sma_50 = if closes.len() >= 50 {
            let mut sma = SimpleMovingAverage::new(50).unwrap();
            for price in &closes {
                sma.next(*price);
            }
            Some(sma.next(closes[closes.len() - 1]))
        } else {
            None
        };

        let ema_12 = if closes.len() >= 12 {
            let mut ema = ExponentialMovingAverage::new(12).unwrap();
            for price in &closes {
                ema.next(*price);
            }
            Some(ema.next(closes[closes.len() - 1]))
        } else {
            None
        };

        let ema_26 = if closes.len() >= 26 {
            let mut ema = ExponentialMovingAverage::new(26).unwrap();
            for price in &closes {
                ema.next(*price);
            }
            Some(ema.next(closes[closes.len() - 1]))
        } else {
            None
        };

        let rsi_14 = if closes.len() >= 14 {
            let mut rsi = RelativeStrengthIndex::new(14).unwrap();
            for price in &closes {
                rsi.next(*price);
            }
            Some(rsi.next(closes[closes.len() - 1]))
        } else {
            None
        };

        let (macd, macd_signal) = if closes.len() >= 26 {
            let mut macd_indicator = MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap();
            for price in &closes {
                macd_indicator.next(*price);
            }
            let result = macd_indicator.next(closes[closes.len() - 1]);
            (Some(result.macd), Some(result.signal))
        } else {
            (None, None)
        };

        let (bollinger_upper, bollinger_lower) = if closes.len() >= 20 {
            let mut bb = BollingerBands::new(20, 2.0).unwrap();
            for price in &closes {
                bb.next(*price);
            }
            let result = bb.next(closes[closes.len() - 1]);
            (Some(result.upper), Some(result.lower))
        } else {
            (None, None)
        };

        Ok(TechnicalIndicators {
            symbol: symbol.to_string(),
            timestamp: sorted_data.last().unwrap().timestamp,
            sma_20,
            sma_50,
            ema_12,
            ema_26,
            rsi_14,
            macd,
            macd_signal,
            bollinger_upper,
            bollinger_lower,
        })
    }
}
