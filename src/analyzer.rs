use yahoo_finance_api as yahoo;
use chrono::{DateTime, Utc};
use time::OffsetDateTime;
use anyhow::Result;
use ta::indicators::{SimpleMovingAverage, RelativeStrengthIndex, MovingAverageConvergenceDivergence};
use ta::{Next, Reset};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StockData {
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug)]
pub struct TechnicalIndicators {
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub rsi: Option<f64>,
    pub macd: Option<(f64, f64, f64)>, // (macd, signal, histogram)
}

pub struct StockAnalyzer {
    provider: yahoo::YahooConnector,
    indicators: HashMap<String, IndicatorSet>,
}

struct IndicatorSet {
    sma_20: SimpleMovingAverage,
    sma_50: SimpleMovingAverage,
    rsi: RelativeStrengthIndex,
    macd: MovingAverageConvergenceDivergence,
}

impl StockAnalyzer {
    pub fn new() -> Self {
        Self {
            provider: yahoo::YahooConnector::new().unwrap(),
            indicators: HashMap::new(),
        }
    }

    /// Fetch historical stock data for a given symbol
    pub async fn fetch_stock_data(
        &self,
        symbol: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<StockData>> {
        // Convert chrono DateTime to time OffsetDateTime
        let start_time = OffsetDateTime::from_unix_timestamp(start.timestamp())?;
        let end_time = OffsetDateTime::from_unix_timestamp(end.timestamp())?;
        
        let response = self
            .provider
            .get_quote_history(symbol, start_time, end_time)
            .await?;

        let mut stock_data = Vec::new();
        let quotes = response.quotes()?;

        for quote in quotes {
            stock_data.push(StockData {
                symbol: symbol.to_string(),
                timestamp: DateTime::from_timestamp(quote.timestamp as i64, 0)
                    .unwrap_or(Utc::now()),
                open: quote.open,
                high: quote.high,
                low: quote.low,
                close: quote.close,
                volume: quote.volume,
            });
        }

        // Sort by timestamp (oldest first)
        stock_data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(stock_data)
    }

    /// Get the latest quote for a symbol
    pub async fn get_latest_quote(&self, symbol: &str) -> Result<StockData> {
        let response = self.provider.get_latest_quotes(symbol, "1d").await?;
        let quote = response.last_quote()?;

        Ok(StockData {
            symbol: symbol.to_string(),
            timestamp: DateTime::from_timestamp(quote.timestamp as i64, 0)
                .unwrap_or(Utc::now()),
            open: quote.open,
            high: quote.high,
            low: quote.low,
            close: quote.close,
            volume: quote.volume,
        })
    }

    /// Initialize indicators for a specific symbol
    fn initialize_indicators(&mut self, symbol: &str) {
        let indicator_set = IndicatorSet {
            sma_20: SimpleMovingAverage::new(20).unwrap(),
            sma_50: SimpleMovingAverage::new(50).unwrap(),
            rsi: RelativeStrengthIndex::new(14).unwrap(),
            macd: MovingAverageConvergenceDivergence::new(12, 26, 9).unwrap(),
        };
        self.indicators.insert(symbol.to_string(), indicator_set);
    }

    /// Calculate technical indicators for stock data
    pub fn calculate_indicators(
        &mut self,
        symbol: &str,
        stock_data: &[StockData],
    ) -> Vec<TechnicalIndicators> {
        if !self.indicators.contains_key(symbol) {
            self.initialize_indicators(symbol);
        }

        let mut results = Vec::new();
        
        if let Some(indicators) = self.indicators.get_mut(symbol) {
            // Reset indicators
            indicators.sma_20.reset();
            indicators.sma_50.reset();
            indicators.rsi.reset();
            indicators.macd.reset();

            for data in stock_data {
                let sma_20 = indicators.sma_20.next(data.close);
                let sma_50 = indicators.sma_50.next(data.close);
                let rsi = indicators.rsi.next(data.close);
                let macd_result = indicators.macd.next(data.close);

                results.push(TechnicalIndicators {
                    sma_20: Some(sma_20),
                    sma_50: Some(sma_50),
                    rsi: Some(rsi),
                    macd: Some((macd_result.macd, macd_result.signal, macd_result.histogram)),
                });
            }
        }

        results
    }

    /// Analyze stock with basic signals
    pub fn analyze_signals(&self, data: &StockData, indicators: &TechnicalIndicators) -> Vec<String> {
        let mut signals = Vec::new();

        // RSI signals
        if let Some(rsi) = indicators.rsi {
            if rsi > 70.0 {
                signals.push("RSI Overbought (>70)".to_string());
            } else if rsi < 30.0 {
                signals.push("RSI Oversold (<30)".to_string());
            }
        }

        // SMA crossover signals
        if let (Some(sma_20), Some(sma_50)) = (indicators.sma_20, indicators.sma_50) {
            if sma_20 > sma_50 && data.close > sma_20 {
                signals.push("Bullish: Price above SMA20 > SMA50".to_string());
            } else if sma_20 < sma_50 && data.close < sma_20 {
                signals.push("Bearish: Price below SMA20 < SMA50".to_string());
            }
        }

        // MACD signals
        if let Some((macd, signal, _)) = indicators.macd {
            if macd > signal {
                signals.push("MACD Bullish: MACD above Signal".to_string());
            } else {
                signals.push("MACD Bearish: MACD below Signal".to_string());
            }
        }

        signals
    }

    /// Print analysis results
    pub fn print_analysis(
        &self,
        symbol: &str,
        stock_data: &[StockData],
        indicators: &[TechnicalIndicators],
    ) {
        println!("=== Stock Analysis for {} ===", symbol);
        
        if let (Some(latest_data), Some(latest_indicators)) = 
            (stock_data.last(), indicators.last()) {
            
            println!("Latest Data ({}):", latest_data.timestamp.format("%Y-%m-%d"));
            println!("  Price: ${:.2}", latest_data.close);
            println!("  Volume: {}", latest_data.volume);
            
            println!("\nTechnical Indicators:");
            if let Some(sma_20) = latest_indicators.sma_20 {
                println!("  SMA(20): ${:.2}", sma_20);
            }
            if let Some(sma_50) = latest_indicators.sma_50 {
                println!("  SMA(50): ${:.2}", sma_50);
            }
            if let Some(rsi) = latest_indicators.rsi {
                println!("  RSI(14): {:.2}", rsi);
            }
            if let Some((macd, signal, histogram)) = latest_indicators.macd {
                println!("  MACD: {:.4}, Signal: {:.4}, Histogram: {:.4}", 
                         macd, signal, histogram);
            }

            println!("\nSignals:");
            let signals = self.analyze_signals(latest_data, latest_indicators);
            if signals.is_empty() {
                println!("  No strong signals detected");
            } else {
                for signal in signals {
                    println!("  • {}", signal);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_stock_data_creation() {
        let stock_data = StockData {
            symbol: "AAPL".to_string(),
            timestamp: Utc::now(),
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 1000000,
        };

        assert_eq!(stock_data.symbol, "AAPL");
        assert_eq!(stock_data.open, 100.0);
        assert_eq!(stock_data.close, 102.0);
    }

    #[test]
    fn test_stock_analyzer_creation() {
        let _analyzer = StockAnalyzer::new();
        // Just test that we can create an analyzer without panic
        assert!(true);
    }

    #[test]
    fn test_technical_indicators_creation() {
        let indicators = TechnicalIndicators {
            sma_20: Some(100.0),
            sma_50: Some(95.0),
            rsi: Some(65.0),
            macd: Some((0.5, 0.3, 0.2)),
        };

        assert_eq!(indicators.sma_20, Some(100.0));
        assert_eq!(indicators.rsi, Some(65.0));
    }
}
