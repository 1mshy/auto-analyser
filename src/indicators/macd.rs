use ta::indicators::MovingAverageConvergenceDivergence as TaMACD;
use ta::{Next, Reset};

/// MACD output structure
#[derive(Debug, Clone, Copy)]
pub struct MACDOutput {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

/// MACD (Moving Average Convergence Divergence) wrapper
/// Currently uses the ta crate implementation but can be extended with custom logic
#[derive(Debug, Clone)]
pub struct MovingAverageConvergenceDivergence {
    inner: TaMACD,
}

impl MovingAverageConvergenceDivergence {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Result<Self, ta::errors::TaError> {
        Ok(Self {
            inner: TaMACD::new(fast_period, slow_period, signal_period)?,
        })
    }

    pub fn next(&mut self, input: f64) -> MACDOutput {
        let result = self.inner.next(input);
        MACDOutput {
            macd: result.macd,
            signal: result.signal,
            histogram: result.histogram,
        }
    }

    pub fn reset(&mut self) {
        self.inner.reset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macd_creation() {
        let macd = MovingAverageConvergenceDivergence::new(12, 26, 9);
        assert!(macd.is_ok());
    }

    #[test]
    fn test_macd_calculation() {
        let mut macd = MovingAverageConvergenceDivergence::new(2, 4, 2).unwrap();
        
        // Feed some data
        for price in [100.0, 101.0, 102.0, 103.0, 104.0].iter() {
            let result = macd.next(*price);
            // Just verify we get valid numeric results
            assert!(result.macd.is_finite());
            assert!(result.signal.is_finite());
            assert!(result.histogram.is_finite());
        }
    }
}
