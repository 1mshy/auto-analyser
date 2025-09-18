use ta::indicators::SimpleMovingAverage as TaSimpleMovingAverage;
use ta::{Next, Reset};

/// Simple Moving Average wrapper
/// Currently uses the ta crate implementation but can be extended with custom logic
#[derive(Debug, Clone)]
pub struct SimpleMovingAverage {
    inner: TaSimpleMovingAverage,
}

impl SimpleMovingAverage {
    pub fn new(period: usize) -> Result<Self, ta::errors::TaError> {
        Ok(Self {
            inner: TaSimpleMovingAverage::new(period)?,
        })
    }

    pub fn next(&mut self, input: f64) -> f64 {
        self.inner.next(input)
    }

    pub fn reset(&mut self) {
        self.inner.reset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma_creation() {
        let sma = SimpleMovingAverage::new(20);
        assert!(sma.is_ok());
    }

    #[test]
    fn test_sma_calculation() {
        let mut sma = SimpleMovingAverage::new(3).unwrap();
        
        let result1 = sma.next(10.0);
        let result2 = sma.next(20.0);
        let result3 = sma.next(30.0);
        
        // After 3 values, SMA should be (10 + 20 + 30) / 3 = 20
        assert!((result3 - 20.0).abs() < 0.001);
    }
}
