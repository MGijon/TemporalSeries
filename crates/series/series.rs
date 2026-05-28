use crate::errors::TemporalSeriesError;
use crate::rolling::RollingSeries;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub index: Vec<i64>,
    pub values: Vec<f64>,
}

impl TimeSeries {
    pub fn new(index: Vec<i64>, values: Vec<f64>) -> Result<Self, TemporalSeriesError> {
        if index.len() != values.len() {
            return Err(TemporalSeriesError::LengthMismatch {
                index_len: index.len(),
                values_len: values.len(),
            });
        }

        Ok(Self { index, values })
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn shift(&self, periods: usize) -> Result<Self, TemporalSeriesError> {
        let mut values = vec![f64::NAN; self.len()];
        for i in periods..self.len() {
            values[i] = self.values[i - periods];
        }
        Self::new(self.index.clone(), values)
    }

    pub fn diff(&self) -> Result<Self, TemporalSeriesError> {
        let mut values = vec![f64::NAN; self.len()];
        for i in 1..self.len() {
            values[i] = self.values[i] - self.values[i - 1];
        }
        Self::new(self.index.clone(), values)
    }

    /// Computes the percentage change between consecutive observations.
    ///
    /// The percentage change is defined as:
    ///
    /// `value[t] / value[t-1] - 1`
    ///
    /// This operation converts price series into returns,
    /// which are commonly used in quantitative finance.
    ///
    /// The first element is set to `NaN` because there is
    /// no previous observation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(
    ///     vec![1, 2, 3],
    ///     vec![100.0, 110.0, 121.0]
    /// ).unwrap();
    ///
    /// let returns = ts.pct_change().unwrap();
    ///
    /// assert!((returns.values[1] - 0.10).abs() < 1e-6);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `TimeSeries` containing percentage returns.
    ///
    /// # Notes
    ///
    /// This function uses simple returns rather than logarithmic returns.
    ///
    /// See also: `log_returns()`
    pub fn pct_change(&self) -> Result<Self, TemporalSeriesError> {
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
