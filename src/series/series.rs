use crate::rolling::RollingSeries;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub index: Vec<i64>,
    pub values: Vec<f64>,
}

impl TimeSeries {
    pub fn new(index: Vec<i64>, values: Vec<f64>) -> Self {
        assert_eq!(index.len(), values.len());
        Self { index, values }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn shift(&self, periods: usize) -> Self {
        let mut values = vec![f64::NAN; self.len()];
        for i in periods..self.len() {
            values[i] = self.values[i - periods];
        }
        Self::new(self.index.clone(), values)
    }

    pub fn diff(&self) -> Self {
        let mut values = vec![f64::NAN; self.len()];
        for i in 1..self.len() {
            values[i] = self.values[i] - self.values[i - 1];
        }
        Self::new(self.index.clone(), values)
    }

    pub fn pct_change(&self) -> Self {
        let mut values = vec![f64::NAN; self.len()];
        for i in 1..self.len() {
            values[i] = self.values[i] / self.values[i - 1] - 1.0;
        }
        Self::new(self.index.clone(), values)
    }

    pub fn rolling(&self, window: usize) -> RollingSeries<'_> {
        RollingSeries::new(self, window)
    }
}
