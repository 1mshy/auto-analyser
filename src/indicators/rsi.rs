/// Custom RSI implementation that matches TradingView's calculation
/// Uses Wilder's smoothing method (exponential moving average with alpha = 1/period)
#[derive(Debug, Clone)]
pub struct CustomRSI {
    period: usize,
    avg_gain: Option<f64>,
    avg_loss: Option<f64>,
    previous_close: Option<f64>,
    count: usize,
    initial_gains: Vec<f64>,
    initial_losses: Vec<f64>,
}

impl CustomRSI {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            avg_gain: None,
            avg_loss: None,
            previous_close: None,
            count: 0,
            initial_gains: Vec::new(),
            initial_losses: Vec::new(),
        }
    }

    pub fn next(&mut self, close: f64) -> Option<f64> {
        if let Some(prev_close) = self.previous_close {
            let change = close - prev_close;
            let gain = if change > 0.0 { change } else { 0.0 };
            let loss = if change < 0.0 { -change } else { 0.0 };

            if self.count < self.period {
                // Collect initial values for the first period
                self.initial_gains.push(gain);
                self.initial_losses.push(loss);
                self.count += 1;

                if self.count == self.period {
                    // Calculate initial averages using simple moving average
                    self.avg_gain = Some(self.initial_gains.iter().sum::<f64>() / self.period as f64);
                    self.avg_loss = Some(self.initial_losses.iter().sum::<f64>() / self.period as f64);
                }
            } else {
                // Use Wilder's smoothing for subsequent values
                let alpha = 1.0 / self.period as f64;
                self.avg_gain = Some(alpha * gain + (1.0 - alpha) * self.avg_gain.unwrap());
                self.avg_loss = Some(alpha * loss + (1.0 - alpha) * self.avg_loss.unwrap());
            }

            // Calculate RSI if we have enough data
            if let (Some(avg_gain), Some(avg_loss)) = (self.avg_gain, self.avg_loss) {
                if avg_loss == 0.0 {
                    return Some(100.0);
                }
                let rs = avg_gain / avg_loss;
                let rsi = 100.0 - (100.0 / (1.0 + rs));
                self.previous_close = Some(close);
                return Some(rsi);
            }
        }

        self.previous_close = Some(close);
        None
    }

    pub fn reset(&mut self) {
        self.avg_gain = None;
        self.avg_loss = None;
        self.previous_close = None;
        self.count = 0;
        self.initial_gains.clear();
        self.initial_losses.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_creation() {
        let rsi = CustomRSI::new(14);
        assert_eq!(rsi.period, 14);
        assert_eq!(rsi.count, 0);
    }

    #[test]
    fn test_rsi_reset() {
        let mut rsi = CustomRSI::new(14);
        rsi.next(100.0);
        rsi.next(105.0);
        rsi.reset();
        assert_eq!(rsi.count, 0);
        assert!(rsi.avg_gain.is_none());
        assert!(rsi.avg_loss.is_none());
    }

    #[test]
    fn test_rsi_calculation() {
        let mut rsi = CustomRSI::new(2); // Small period for testing
        
        // First value - no RSI yet
        assert!(rsi.next(100.0).is_none());
        
        // Second value - still no RSI (need one more for initial calculation)
        assert!(rsi.next(105.0).is_none());
        
        // Third value - should have RSI now
        let result = rsi.next(110.0);
        assert!(result.is_some());
        assert!(result.unwrap() > 0.0 && result.unwrap() <= 100.0);
    }
}
