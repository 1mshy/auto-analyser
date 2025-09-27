use crate::StockData;

pub struct AverageTrueRange {
    pub period: usize,
}

impl AverageTrueRange {
    pub fn new(period: usize) -> Self {
        Self { period }
    }

    pub fn calculate(&self, data: &[StockData]) -> Vec<Option<f64>> {
        if data.len() < 2 {
            return vec![None; data.len()];
        }

        let mut true_ranges = Vec::new();
        let mut results = vec![None]; // First value is None

        // Calculate True Range for each period
        for i in 1..data.len() {
            let high = data[i].high;
            let low = data[i].low;
            let prev_close = data[i-1].close;
            
            let tr = (high - low)
                .max((high - prev_close).abs())
                .max((low - prev_close).abs());
            
            true_ranges.push(tr);
            
            // Calculate ATR once we have enough TR values
            if true_ranges.len() >= self.period {
                let atr = if true_ranges.len() == self.period {
                    // First ATR is simple average
                    true_ranges.iter().sum::<f64>() / self.period as f64
                } else {
                    // Subsequent ATRs use smoothing
                    let prev_atr = results.last().unwrap().unwrap();
                    (prev_atr * (self.period - 1) as f64 + tr) / self.period as f64
                };
                results.push(Some(atr));
            } else {
                results.push(None);
            }
        }

        results
    }

    /// Calculate volatility percentile based on ATR
    pub fn volatility_percentile(&self, current_atr: f64, atr_values: &[Option<f64>]) -> f64 {
        let valid_values: Vec<f64> = atr_values.iter()
            .filter_map(|&v| v)
            .collect();
        
        if valid_values.is_empty() {
            return 50.0; // Default to 50th percentile
        }
        
        let count_below = valid_values.iter()
            .filter(|&&v| v < current_atr)
            .count();
        
        (count_below as f64 / valid_values.len() as f64) * 100.0
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        Self::new(14)
    }
}

pub struct CommodityChannelIndex {
    pub period: usize,
    pub factor: f64,
}

impl CommodityChannelIndex {
    pub fn new(period: usize, factor: f64) -> Self {
        Self { period, factor }
    }

    pub fn calculate(&self, data: &[StockData]) -> Vec<Option<f64>> {
        if data.len() < self.period {
            return vec![None; data.len()];
        }

        let mut results = vec![None; self.period - 1];

        for i in (self.period - 1)..data.len() {
            let window = &data[i - self.period + 1..=i];
            
            // Calculate Typical Price for each day
            let typical_prices: Vec<f64> = window.iter()
                .map(|d| (d.high + d.low + d.close) / 3.0)
                .collect();
            
            // Calculate Simple Moving Average of Typical Price
            let sma_tp = typical_prices.iter().sum::<f64>() / self.period as f64;
            
            // Calculate Mean Deviation
            let mean_deviation = typical_prices.iter()
                .map(|&tp| (tp - sma_tp).abs())
                .sum::<f64>() / self.period as f64;
            
            // Calculate CCI
            let current_tp = typical_prices.last().unwrap();
            let cci = if mean_deviation != 0.0 {
                (current_tp - sma_tp) / (self.factor * mean_deviation)
            } else {
                0.0
            };
            
            results.push(Some(cci));
        }

        results
    }

    /// Generate trading signals based on CCI
    pub fn generate_signals(&self, values: &[Option<f64>]) -> Vec<String> {
        let mut signals = Vec::new();
        
        if values.len() < 2 {
            return signals;
        }

        for i in 1..values.len() {
            if let (Some(current), Some(prev)) = (values[i], values[i-1]) {
                // Overbought/Oversold levels
                if current >= 100.0 {
                    signals.push("CCI Overbought - Potential Reversal".to_string());
                } else if current <= -100.0 {
                    signals.push("CCI Oversold - Potential Reversal".to_string());
                }
                
                // Zero line crosses
                if prev < 0.0 && current >= 0.0 {
                    signals.push("CCI Bullish - Crossed Above Zero".to_string());
                } else if prev > 0.0 && current <= 0.0 {
                    signals.push("CCI Bearish - Crossed Below Zero".to_string());
                }
                
                // Extreme readings
                if current >= 200.0 {
                    signals.push("CCI Extremely Overbought - Strong Sell Signal".to_string());
                } else if current <= -200.0 {
                    signals.push("CCI Extremely Oversold - Strong Buy Signal".to_string());
                }
                
                // Trend strength
                if current > 100.0 && prev > 100.0 {
                    signals.push("CCI Strong Uptrend Continuation".to_string());
                } else if current < -100.0 && prev < -100.0 {
                    signals.push("CCI Strong Downtrend Continuation".to_string());
                }
            }
        }
        
        signals
    }
}

impl Default for CommodityChannelIndex {
    fn default() -> Self {
        Self::new(20, 0.015)
    }
}
