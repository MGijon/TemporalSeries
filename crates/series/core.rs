use crate::errors::TemporalSeriesError;
use crate::rolling::RollingSeries;

/// A one-dimensional time series of floating-point observations.
///
/// A `TimeSeries` pairs an ordered sequence of integer timestamps (`index`)
/// with a corresponding sequence of `f64` values. Both sequences must always
/// have the same length — enforced at construction time by [`TimeSeries::new`].
///
/// # Fields
///
/// - `index` — monotonically increasing integer timestamps (e.g. Unix seconds,
///   trading day offsets, or simple integer steps).
/// - `values` — the observed values at each timestamp. `NaN` is used as a
///   sentinel for missing or undefined observations (e.g. the first element
///   after a [`TimeSeries::diff`] or [`TimeSeries::shift`]).
///
/// # Example
///
/// ```rust
/// use temporalseries::series::TimeSeries;
///
/// let ts = TimeSeries::new(
///     vec![1, 2, 3],
///     vec![100.0, 110.0, 121.0],
/// ).unwrap();
///
/// assert_eq!(ts.len(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub index: Vec<i64>,
    pub values: Vec<f64>,
}

impl TimeSeries {
    /// Creates a new `TimeSeries` from an index and a values vector.
    ///
    /// The index represents the time axis (e.g. timestamps, integer steps)
    /// and must have the same length as `values`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![10.0, 20.0, 30.0]).unwrap();
    /// assert_eq!(ts.len(), 3);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::LengthMismatch`] if `index` and `values`
    /// have different lengths.
    pub fn new(index: Vec<i64>, values: Vec<f64>) -> Result<Self, TemporalSeriesError> {
        if index.len() != values.len() {
            return Err(TemporalSeriesError::LengthMismatch {
                index_len: index.len(),
                values_len: values.len(),
            });
        }

        Ok(Self { index, values })
    }

    /// Returns the number of elements in the series.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![10.0, 20.0, 30.0]).unwrap();
    /// assert_eq!(ts.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` if the series contains no elements.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let empty = TimeSeries::new(vec![], vec![]).unwrap();
    /// assert!(empty.is_empty());
    ///
    /// let ts = TimeSeries::new(vec![1], vec![1.0]).unwrap();
    /// assert!(!ts.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Shifts the series forward by `periods` positions.
    ///
    /// Values are moved to the right by `periods` steps. The first `periods`
    /// elements are set to `NaN` since there are no preceding values to shift from.
    ///
    /// This is commonly used to align a series with a lagged version of itself.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    /// let shifted = ts.shift(2).unwrap();
    ///
    /// assert!(shifted.values[0].is_nan());
    /// assert!(shifted.values[1].is_nan());
    /// assert_eq!(shifted.values[2], 1.0);
    /// assert_eq!(shifted.values[3], 2.0);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError`] if the resulting series cannot be constructed.
    pub fn shift(&self, periods: usize) -> Result<Self, TemporalSeriesError> {
        let mut values: Vec<f64> = vec![f64::NAN; self.len()];

        values[periods..].copy_from_slice(&self.values[..self.len() - periods]);
        Self::new(self.index.clone(), values)
    }

    /// Computes the first-order difference between consecutive observations.
    ///
    /// Each element is defined as `value[t] - value[t-1]`. The first element
    /// is set to `NaN` because there is no preceding observation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![10.0, 13.0, 20.0]).unwrap();
    /// let d = ts.diff().unwrap();
    ///
    /// assert!(d.values[0].is_nan());
    /// assert_eq!(d.values[1], 3.0);
    /// assert_eq!(d.values[2], 7.0);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError`] if the resulting series cannot be constructed.
    pub fn diff(&self) -> Result<Self, TemporalSeriesError> {
        let mut values: Vec<f64> = vec![f64::NAN; self.len()];

        for (result, window) in values[1..].iter_mut().zip(self.values.windows(2)) {
            *result = window[1] - window[0];
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
    pub fn pct_change(&self) -> Result<Self, TemporalSeriesError> {
        let mut values = vec![f64::NAN; self.len()];

        for (result, window) in values[1..].iter_mut().zip(self.values.windows(2)) {
            *result = window[1] / window[0] - 1.0;
        }
        Self::new(self.index.clone(), values)
    }

    /// Creates a [`RollingSeries`] view over this series with the given window size.
    ///
    /// The returned value is a lazy handle — no computation happens until you call
    /// a method on it such as [`RollingSeries::mean`].
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
    /// let mean = ts.rolling(3).mean().unwrap();
    ///
    /// assert!(mean.values[0].is_nan());
    /// assert!((mean.values[4] - 4.0).abs() < 1e-6);
    /// ```
    pub fn rolling(&self, window: usize) -> RollingSeries<'_> {
        RollingSeries::new(self, window)
    }
}
