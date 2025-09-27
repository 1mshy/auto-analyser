use crate::StockData;

pub struct BollingerBands {
    pub period: usize,
    pub std_dev_multiplier: f64,
}

#[derive(Debug, Clone)]
pub struct BollingerBandsValue {
    pub upper_band: f64,
    pub middle_band: f64, // SMA
    pub lower_band: f64,
    pub bandwidth: f64,
    pub percent_b: f64, // Position within bands (0-1)
}

impl BollingerBands {
    pub fn new(period: usize, std_dev_multiplier: f64) -> Self {
        Self {
            period,
            std_dev_multiplier,
        }
    }

    pub fn calculate(&self, data: &[StockData]) -> Vec<Option<BollingerBandsValue>> {
        if data.len() < self.period {
            return vec![None; data.len()];
        }

        let mut results = vec![None; self.period - 1];
        
        for i in (self.period - 1)..data.len() {
            let window = &data[i - self.period + 1..=i];
            
            // Calculate SMA (middle band)
            let sma: f64 = window.iter().map(|d| d.close).sum::<f64>() / self.period as f64;
            
            // Calculate standard deviation
            let variance: f64 = window.iter()
                .map(|d| (d.close - sma).powi(2))
                .sum::<f64>() / self.period as f64;
            let std_dev = variance.sqrt();
            
            // Calculate bands
            let upper_band = sma + (self.std_dev_multiplier * std_dev);
            let lower_band = sma - (self.std_dev_multiplier * std_dev);
            
            // Calculate bandwidth (volatility measure)
            let bandwidth = if sma != 0.0 {
                (upper_band - lower_band) / sma * 100.0
            } else {
                0.0
            };
            
            // Calculate %B (position within bands)
            let current_price = data[i].close;
            let percent_b = if upper_band != lower_band {
                (current_price - lower_band) / (upper_band - lower_band)
            } else {
                0.5
            };
            
            results.push(Some(BollingerBandsValue {
                upper_band,
                middle_band: sma,
                lower_band,
                bandwidth,
                percent_b,
            }));
        }
        
        results
    }

    /// Generate trading signals based on Bollinger Bands
    pub fn generate_signals(&self, data: &[StockData], bb_values: &[Option<BollingerBandsValue>]) -> Vec<String> {
        let mut signals = Vec::new();
        
        if data.len() < 2 || bb_values.len() < 2 {
            return signals;
        }

        for i in 1..data.len() {
            if let (Some(current_bb), Some(prev_bb)) = (&bb_values[i], &bb_values[i-1]) {
                let current_price = data[i].close;
                let prev_price = data[i-1].close;
                
                // Bollinger Squeeze (low volatility)
                if current_bb.bandwidth < 10.0 {
                    signals.push("Bollinger Squeeze - Volatility Breakout Expected".to_string());
                }
                
                // Price touching upper band (potential sell signal)
                if current_price >= current_bb.upper_band * 0.99 {
                    signals.push("Price Near Upper Band - Potential Overbought".to_string());
                }
                
                // Price touching lower band (potential buy signal)
                if current_price <= current_bb.lower_band * 1.01 {
                    signals.push("Price Near Lower Band - Potential Oversold".to_string());
                }
                
                // Bollinger Band Walk (strong trend)
                if current_price > current_bb.upper_band && prev_price > prev_bb.upper_band {
                    signals.push("Upper Band Walk - Strong Uptrend".to_string());
                } else if current_price < current_bb.lower_band && prev_price < prev_bb.lower_band {
                    signals.push("Lower Band Walk - Strong Downtrend".to_string());
                }
                
                // Mean reversion signals
                if prev_bb.percent_b > 0.8 && current_bb.percent_b < 0.8 {
                    signals.push("Reversal from Overbought - Potential Sell".to_string());
                } else if prev_bb.percent_b < 0.2 && current_bb.percent_b > 0.2 {
                    signals.push("Reversal from Oversold - Potential Buy".to_string());
                }
            }
        }
        
        signals
    }
}

impl Default for BollingerBands {
    fn default() -> Self {
        Self::new(20, 2.0)
    }
}
