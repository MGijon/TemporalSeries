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

    /// Returns the value of the mean estimator.
    ///
    /// TODO: check that this formula is working properly
    /// $$ \hat{\mu} = \frac{1}{n} \sum^{n}_{i=0} x_i$$
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let sut_1: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4], vec![0.0, 0.0, 0.0, 0.0]).unwrap();
    /// assert!(sut_1.mean() == 0.0);
    ///
    /// let sut_2: TimeSeries = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    /// assert!(sut_2.mean() == 2.5);
    /// ```
    pub fn mean(&self) -> f64 {
        let total_sum: f64 = self.values.iter().sum();
        let len_casted: f64 = self.len() as f64;

        total_sum / len_casted
    }

    // With Bessel's correction
    // Compute the std stimator
    // Lets assume that is working since all tests are passing
    pub fn std_deviation(&self) -> f64 {
        let estimated_mean: f64 = self.mean();
        let len_casted: f64 = self.len() as f64;
        if len_casted == 1.0 {
            return 0.0;
        } else {
            let bessels_correction_factor: f64 = 1.0 / (len_casted - 1.0);
            let mut summatory: f64 = 0.0;

            for element in &self.values {
                summatory += (element - estimated_mean).powi(2);
            }
            return bessels_correction_factor * summatory;
        }
    }

    /// Returns the p-th quantile of the series using linear interpolation.
    ///
    /// Computes the value below which a fraction `p` of observations fall.
    /// The virtual index is `h = p * (n - 1)`; the result interpolates linearly
    /// between `sorted[floor(h)]` and `sorted[ceil(h)]`.
    ///
    /// This matches `numpy.quantile(arr, p, method='linear')`.
    ///
    /// # Errors
    ///
    /// - [`TemporalSeriesError::ParameterRangeError`] if `p` is outside `[0.0, 1.0]`.
    /// - [`TemporalSeriesError::EmptySeries`] if the series has no non-NaN values.
    ///
    /// # Examples
    ///
    /// Exact quantiles on an odd-length series:
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    ///
    /// assert_eq!(ts.quantile(0.0).unwrap(), 1.0);
    /// assert_eq!(ts.quantile(0.25).unwrap(), 2.0);
    /// assert_eq!(ts.quantile(0.5).unwrap(), 3.0);
    /// assert_eq!(ts.quantile(0.75).unwrap(), 4.0);
    /// assert_eq!(ts.quantile(1.0).unwrap(), 5.0);
    /// ```
    ///
    /// Interpolated median on an even-length series:
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    ///
    /// // h = 0.5 * 3 = 1.5  ->  2.0 + 0.5 * (3.0 - 2.0) = 2.5
    /// assert!((ts.quantile(0.5).unwrap() - 2.5).abs() < 1e-9);
    /// ```
    ///
    /// Out-of-range `p` returns an error:
    ///
    /// ```rust
    /// use temporalseries::{errors::TemporalSeriesError, series::TimeSeries};
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();
    ///
    /// assert!(matches!(ts.quantile(-0.1), Err(TemporalSeriesError::ParameterRangeError(_))));
    /// assert!(matches!(ts.quantile(1.1),  Err(TemporalSeriesError::ParameterRangeError(_))));
    /// ```
    pub fn quantile(&self, p: f32) -> Result<f64, TemporalSeriesError> {
        if p < 0.0 || p > 1.0 {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "p must be in [0.0, 1.0], got {p}"
            )));
        }

        let mut sorted: Vec<f64> = self.values.iter().copied().filter(|v| !v.is_nan()).collect();
        if sorted.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted.len();
        let h = p as f64 * (n - 1) as f64;
        let lo = h.floor() as usize;
        let hi = h.ceil() as usize;
        let frac = h - lo as f64;

        Ok(sorted[lo] + frac * (sorted[hi] - sorted[lo]))
    }

    /// TODO: create an interface for this object or similar -> change rust's approach to thsi problem
    /// This will only call quantile function a bunch of times...
    #[allow(dead_code)]
    pub fn all_quantiles(&self) -> Vec<f64> {
        vec![0.0]
    }

    /// Returns the Interquartile Range (IQR) of the series.
    ///
    /// IQR = Q3 − Q1 = `quantile(0.75)` − `quantile(0.25)`.
    ///
    /// # Errors
    ///
    /// - [`TemporalSeriesError::EmptySeries`] if the series has no non-NaN values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    ///
    /// // Q1 = 2.0, Q3 = 4.0, IQR = 2.0
    /// assert_eq!(ts.iqr().unwrap(), 2.0);
    /// ```
    pub fn iqr(&self) -> Result<f64, TemporalSeriesError> {
        let q1: f64 = self.quantile(0.25)?;
        let q3: f64 = self.quantile(0.75)?;
        Ok(q3 - q1)
    }

    /// TODO: use the formula for computing this!
    #[allow(dead_code)]
    pub fn rimple_return(&self) -> f64 {
        0.0
    }

    /// TODO: use the formula for computing this!
    /// TODO: Check how to work with logs on RUST
    #[allow(dead_code)]
    pub fn log_return(&self) -> f64 {
        0.0
    }

    /// TODO: use the formula for computing this!
    #[allow(dead_code)]
    pub fn cumulative_return(&self) -> f64 {
        0.0
    }

    /// TODO: look for the correct type... probably return a time series?
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn moving_average(&self, n: u64) -> f64 {
        0.0
    }

    /// TODO: use the formula for computing this!
    #[allow(dead_code)]
    pub fn exponential_moving_average(&self) -> f64 {
        0.0
    }

    /// TODO: use the formula for computing this!
    #[allow(dead_code)]
    pub fn crossover_signal(&self) -> f64 {
        0.0
    }

    // VOLATILITY -------------------------------------------------------------
    /// TODO: use the formula for computing this!
    #[allow(dead_code)]
    pub fn rolling_standard_derivation(&self) -> f64 {
        0.0
    }

    /// TODO: check how to compute this!
    #[allow(dead_code)]
    pub fn true_range(&self) -> f64 {
        0.0
    }

    /// TODO: check how to compute this!
    #[allow(dead_code)]
    pub fn average_true_range(&self) -> f64 {
        0.0
    }

    /// TODO: check how we can return both bands... what is the most rustonean way to do it?
    #[allow(dead_code)]
    pub fn borillenger_bands(&self) -> f64 {
        0.0
    }

    // AUTOCORRELATION --------------------------------------------------------
    /// TODO: check how to compute this
    #[allow(dead_code)]
    pub fn autocorrelation_function(&self) -> f64 {
        0.0
    }

    /// TODO: check how to compute this
    #[allow(dead_code)]
    pub fn partial_autocorrelation_function(&self) -> f64 {
        0.0
    }

    // STATIONARITY -----------------------------------------------------------

    /// TODO: add test for this function in this same file
    /// CHECK the formula for computing this statistic
    #[allow(dead_code)]
    fn stationary_dickey_fuller_statistics(&self) -> f64 {
        0.0
    }

    /// TODO: add test for this function in this same file
    /// CHECK the formula for computing this statistic
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn stationary_dickey_fuller_test(&self, alpha: f32) -> bool {
        true
    }

    // DISTRIBUTION ANALYSIS --------------------------------------------------

    /// TODO: add test for this function in this same file
    /// TODO: CHECK the formula for computing this statistic
    #[allow(dead_code)]
    pub fn skewness(&self) -> f64 {
        0.0
    }

    /// TODO: CHECK the formula for computing this statistic
    #[allow(dead_code)]
    pub fn excess_kurtosis(&self) -> f64 {
        0.0
    }

    /// TODO: add test for this function in this same file
    /// TODO: look for this formula
    #[allow(dead_code)]
    fn jacque_bera_statistics(&self) -> f64 {
        0.0
    }

    /// TODO: check this test implementation
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn jacque_bera_test(&self, alpha: f32) -> bool {
        true
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
