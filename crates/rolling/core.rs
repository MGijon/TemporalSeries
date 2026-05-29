use crate::errors::TemporalSeriesError;
use crate::series::TimeSeries;

pub struct RollingSeries<'a> {
    series: &'a TimeSeries,
    window: usize,
}

impl<'a> RollingSeries<'a> {
    pub fn new(series: &'a TimeSeries, window: usize) -> Self {
        Self { series, window }
    }

    /// Computes the rolling mean over a sliding window.
    ///
    /// For each position `t`, the result is the arithmetic mean of
    /// `values[t - window + 1..=t]`. The first `window - 1` elements
    /// are set to `NaN` because there are not enough preceding observations
    /// to fill the window.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(
    ///     vec![1, 2, 3, 4, 5],
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0],
    /// ).unwrap();
    ///
    /// let result = ts.rolling(3).mean().unwrap();
    ///
    /// assert!(result.values[0].is_nan());
    /// assert!(result.values[1].is_nan());
    /// assert!((result.values[2] - 2.0).abs() < 1e-6); // (1 + 2 + 3) / 3
    /// assert!((result.values[4] - 4.0).abs() < 1e-6); // (3 + 4 + 5) / 3
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError`] if the resulting series cannot be constructed.
    pub fn mean(&self) -> Result<TimeSeries, TemporalSeriesError> {
        let n: usize = self.series.len();
        let mut result = vec![f64::NAN; n];

        for (i, item) in result.iter_mut().enumerate().skip(self.window - 1) {
            let slice: &[f64] = &self.series.values[i + 1 - self.window..=i];
            let mean: f64 = slice.iter().sum::<f64>() / self.window as f64;
            *item = mean;
        }

        TimeSeries::new(self.series.index.clone(), result)
    }
}
