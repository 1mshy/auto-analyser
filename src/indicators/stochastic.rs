use crate::StockData;

pub struct StochasticOscillator {
    pub k_period: usize,
    pub d_period: usize,
}

#[derive(Debug, Clone)]
pub struct StochasticValue {
    pub k_percent: f64,  // Fast stochastic
    pub d_percent: f64,  // Slow stochastic (SMA of %K)
}

impl StochasticOscillator {
    pub fn new(k_period: usize, d_period: usize) -> Self {
        Self { k_period, d_period }
    }

    pub fn calculate(&self, data: &[StockData]) -> Vec<Option<StochasticValue>> {
        if data.len() < self.k_period {
            return vec![None; data.len()];
        }

        let mut k_values = Vec::new();
        let mut results = vec![None; self.k_period - 1];

        // Calculate %K values
        for i in (self.k_period - 1)..data.len() {
            let window = &data[i - self.k_period + 1..=i];
            
            let highest_high = window.iter().map(|d| d.high).fold(f64::NEG_INFINITY, f64::max);
            let lowest_low = window.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);
            let current_close = data[i].close;
            
            let k_percent = if highest_high != lowest_low {
                ((current_close - lowest_low) / (highest_high - lowest_low)) * 100.0
            } else {
                50.0
            };
            
            k_values.push(k_percent);
            
            // Calculate %D (SMA of %K) if we have enough %K values
            if k_values.len() >= self.d_period {
                let d_start = k_values.len() - self.d_period;
                let d_percent = k_values[d_start..].iter().sum::<f64>() / self.d_period as f64;
                
                results.push(Some(StochasticValue {
                    k_percent,
                    d_percent,
                }));
            } else {
                results.push(Some(StochasticValue {
                    k_percent,
                    d_percent: k_percent, // Use %K as %D until we have enough data
                }));
            }
        }

        results
    }

    /// Generate trading signals based on Stochastic Oscillator
    pub fn generate_signals(&self, values: &[Option<StochasticValue>]) -> Vec<String> {
        let mut signals = Vec::new();
        
        if values.len() < 2 {
            return signals;
        }

        for i in 1..values.len() {
            if let (Some(current), Some(prev)) = (&values[i], &values[i-1]) {
                // Overbought/Oversold conditions
                if current.k_percent >= 80.0 && current.d_percent >= 80.0 {
                    signals.push("Stochastic Overbought - Potential Sell Signal".to_string());
                } else if current.k_percent <= 20.0 && current.d_percent <= 20.0 {
                    signals.push("Stochastic Oversold - Potential Buy Signal".to_string());
                }
                
                // Bullish crossover (%K crosses above %D)
                if prev.k_percent <= prev.d_percent && current.k_percent > current.d_percent {
                    if current.k_percent < 50.0 {
                        signals.push("Stochastic Bullish Crossover - Buy Signal".to_string());
                    } else {
                        signals.push("Stochastic Bullish Crossover - Weak Buy".to_string());
                    }
                }
                
                // Bearish crossover (%K crosses below %D)
                if prev.k_percent >= prev.d_percent && current.k_percent < current.d_percent {
                    if current.k_percent > 50.0 {
                        signals.push("Stochastic Bearish Crossover - Sell Signal".to_string());
                    } else {
                        signals.push("Stochastic Bearish Crossover - Weak Sell".to_string());
                    }
                }
                
                // Divergence detection (simplified)
                if current.k_percent > 80.0 && prev.k_percent > current.k_percent {
                    signals.push("Stochastic Bearish Divergence - Momentum Weakening".to_string());
                } else if current.k_percent < 20.0 && prev.k_percent < current.k_percent {
                    signals.push("Stochastic Bullish Divergence - Momentum Building".to_string());
                }
            }
        }
        
        signals
    }
}

impl Default for StochasticOscillator {
    fn default() -> Self {
        Self::new(14, 3)
    }
}
