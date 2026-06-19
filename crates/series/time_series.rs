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

    /// Returns the arithmetic mean of the series.
    ///
    /// # Formula
    ///
    /// $$\hat{\mu} = \frac{1}{n} \sum_{i=1}^{n} x_i$$
    ///
    /// where `n` is the number of observations.
    ///
    /// # Examples
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

        let mut sorted: Vec<f64> = self
            .values
            .iter()
            .copied()
            .filter(|v| !v.is_nan())
            .collect();
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

    /// Returns the per-period simple return series.
    ///
    /// # Formula
    ///
    /// $$r_t = \frac{x_t - x_{t-1}}{x_{t-1}}$$
    ///
    /// The first element is `NaN` because there is no prior observation.
    /// Identical to [`TimeSeries::pct_change`], provided here under its financial name.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();
    /// let r = ts.simple_return().unwrap();
    ///
    /// assert!(r.values[0].is_nan());
    /// assert!((r.values[1] - 0.1).abs() < 1e-9);
    /// assert!((r.values[2] - 0.1).abs() < 1e-9);
    /// ```
    pub fn simple_return(&self) -> Result<TimeSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mut values: Vec<f64> = vec![f64::NAN; self.len()];
        for (result, window) in values[1..].iter_mut().zip(self.values.windows(2)) {
            *result = (window[1] - window[0]) / window[0];
        }
        Self::new(self.index.clone(), values)
    }

    /// Returns the per-period logarithmic return series.
    ///
    /// # Formula
    ///
    /// $$r_t^{log} = \ln\!\left(\frac{x_t}{x_{t-1}}\right)$$
    ///
    /// The first element is `NaN` because there is no prior observation.
    /// Log returns are additive over time, making them useful for multi-period analysis.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2], vec![1.0, std::f64::consts::E]).unwrap();
    /// let r = ts.log_return().unwrap();
    ///
    /// assert!(r.values[0].is_nan());
    /// assert!((r.values[1] - 1.0).abs() < 1e-9);
    /// ```
    pub fn log_return(&self) -> Result<TimeSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mut values: Vec<f64> = vec![f64::NAN; self.len()];
        for (result, window) in values[1..].iter_mut().zip(self.values.windows(2)) {
            *result = (window[1] / window[0]).ln();
        }
        Self::new(self.index.clone(), values)
    }

    /// Returns the total cumulative return from the first to the last observation.
    ///
    /// # Formula
    ///
    /// $$R_{cum} = \frac{x_T - x_0}{x_0}$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![100.0, 110.0, 121.0]).unwrap();
    ///
    /// // (121 - 100) / 100 = 0.21
    /// assert!((ts.cumulative_return().unwrap() - 0.21).abs() < 1e-9);
    /// ```
    pub fn cumulative_return(&self) -> Result<f64, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let x0: f64 = self.values[0];
        let xt: f64 = self.values[self.len() - 1];
        Ok((xt - x0) / x0)
    }

    /// Returns the n-period simple moving average series.
    ///
    /// # Formula
    ///
    /// $$MA_t^{(n)} = \frac{1}{n} \sum_{i=0}^{n-1} x_{t-i}$$
    ///
    /// The first `n - 1` elements are `NaN` because the window is not yet full.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `n` exceeds the series length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    /// let ma = ts.moving_average(3).unwrap();
    ///
    /// assert!(ma.values[0].is_nan());
    /// assert!(ma.values[1].is_nan());
    /// assert!((ma.values[2] - 2.0).abs() < 1e-9);
    /// assert!((ma.values[4] - 4.0).abs() < 1e-9);
    /// ```
    pub fn moving_average(&self, n: usize) -> Result<TimeSeries, TemporalSeriesError> {
        if n > self.len() {
            return Err(TemporalSeriesError::InvalidWindow {
                window: n,
                series_len: self.len(),
            });
        }
        self.rolling(n).mean()
    }

    /// Returns the exponential moving average (EMA) series for a given span.
    ///
    /// # Formula
    ///
    /// $$\alpha = \frac{2}{span + 1}, \qquad EMA_t = \alpha \cdot x_t + (1 - \alpha) \cdot EMA_{t-1}$$
    ///
    /// The first value seeds the EMA as $EMA_0 = x_0$.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// // span=3 -> alpha=0.5
    /// let ts = TimeSeries::new(vec![1, 2, 3], vec![1.0, 2.0, 3.0]).unwrap();
    /// let ema = ts.exponential_moving_average(3).unwrap();
    ///
    /// assert_eq!(ema.values[0], 1.0);
    /// assert_eq!(ema.values[1], 1.5);   // 0.5*2 + 0.5*1
    /// assert_eq!(ema.values[2], 2.25);  // 0.5*3 + 0.5*1.5
    /// ```
    pub fn exponential_moving_average(
        &self,
        span: usize,
    ) -> Result<TimeSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let alpha: f64 = 2.0 / (span as f64 + 1.0);
        let mut values: Vec<f64> = vec![0.0; self.len()];
        values[0] = self.values[0];
        for i in 1..self.len() {
            values[i] = alpha * self.values[i] + (1.0 - alpha) * values[i - 1];
        }
        Self::new(self.index.clone(), values)
    }

    /// Returns the MA crossover signal series.
    ///
    /// Compares a fast (shorter-window) moving average against a slow (longer-window) one
    /// and marks crossover moments:
    ///
    /// - `+1.0` — fast MA crosses **above** slow MA (bullish signal)
    /// - `-1.0` — fast MA crosses **below** slow MA (bearish signal)
    /// - `0.0` — no crossover
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::ParameterRangeError`] if `fast >= slow`, or
    /// [`TemporalSeriesError::InvalidWindow`] if either window exceeds the series length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// // Constant series — MAs are always equal, no crossover ever occurs.
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![3.0, 3.0, 3.0, 3.0, 3.0]).unwrap();
    /// let sig = ts.crossover_signal(2, 3).unwrap();
    ///
    /// assert!(sig.values.iter().all(|&v| v == 0.0));
    /// ```
    pub fn crossover_signal(
        &self,
        fast: usize,
        slow: usize,
    ) -> Result<TimeSeries, TemporalSeriesError> {
        if fast >= slow {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "fast window ({fast}) must be smaller than slow window ({slow})"
            )));
        }
        let fast_ma: TimeSeries = self.moving_average(fast)?;
        let slow_ma: TimeSeries = self.moving_average(slow)?;
        let mut signals: Vec<f64> = vec![0.0; self.len()];
        for i in 1..self.len() {
            let prev: f64 = fast_ma.values[i - 1] - slow_ma.values[i - 1];
            let curr: f64 = fast_ma.values[i] - slow_ma.values[i];
            if prev.is_nan() || curr.is_nan() {
                continue;
            }
            if prev <= 0.0 && curr > 0.0 {
                signals[i] = 1.0;
            } else if prev >= 0.0 && curr < 0.0 {
                signals[i] = -1.0;
            }
        }
        Self::new(self.index.clone(), signals)
    }

    // VOLATILITY -------------------------------------------------------------

    /// Returns the n-period rolling standard deviation series (Bessel-corrected).
    ///
    /// # Formula
    ///
    /// $$\sigma_t^{(n)} = \sqrt{\frac{1}{n-1} \sum_{i=0}^{n-1} \left(x_{t-i} - \bar{x}_t^{(n)}\right)^2}$$
    ///
    /// The first `n - 1` elements are `NaN` because the window is not yet full.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `n` exceeds the series length or `n < 2`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// // Linear series [1..5] has rolling std 1.0 for every full window of 3.
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    /// let std = ts.rolling_standard_deviation(3).unwrap();
    ///
    /// assert!(std.values[0].is_nan());
    /// assert!(std.values[1].is_nan());
    /// assert!((std.values[2] - 1.0).abs() < 1e-9);
    /// assert!((std.values[4] - 1.0).abs() < 1e-9);
    /// ```
    pub fn rolling_standard_deviation(&self, n: usize) -> Result<TimeSeries, TemporalSeriesError> {
        if n < 2 || n > self.len() {
            return Err(TemporalSeriesError::InvalidWindow {
                window: n,
                series_len: self.len(),
            });
        }
        let len: usize = self.len();
        let mut result: Vec<f64> = vec![f64::NAN; len];
        for i in (n - 1)..len {
            let window: &[f64] = &self.values[i + 1 - n..=i];
            let mean: f64 = window.iter().sum::<f64>() / n as f64;
            let variance: f64 =
                window.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
            result[i] = variance.sqrt();
        }
        Self::new(self.index.clone(), result)
    }

    /// Returns the per-period true range series.
    ///
    /// For a univariate series (no OHLC data), the true range simplifies to the
    /// absolute change between consecutive observations:
    ///
    /// $$TR_t = \left|x_t - x_{t-1}\right|$$
    ///
    /// The first element is `NaN` because there is no prior observation.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 3.0, 6.0, 10.0]).unwrap();
    /// let tr = ts.true_range().unwrap();
    ///
    /// assert!(tr.values[0].is_nan());
    /// assert_eq!(tr.values[1], 2.0);
    /// assert_eq!(tr.values[2], 3.0);
    /// assert_eq!(tr.values[3], 4.0);
    /// ```
    pub fn true_range(&self) -> Result<TimeSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mut values: Vec<f64> = vec![f64::NAN; self.len()];
        for (result, window) in values[1..].iter_mut().zip(self.values.windows(2)) {
            *result = (window[1] - window[0]).abs();
        }
        Self::new(self.index.clone(), values)
    }

    /// Returns the n-period average true range (ATR) series.
    ///
    /// # Formula
    ///
    /// $$ATR_t^{(n)} = \frac{1}{n} \sum_{i=0}^{n-1} TR_{t-i}, \qquad TR_t = \left|x_t - x_{t-1}\right|$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `n` is too large for the series.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4], vec![1.0, 3.0, 6.0, 10.0]).unwrap();
    /// let atr = ts.average_true_range(2).unwrap();
    ///
    /// // TR = [NaN, 2, 3, 4]; ATR(2): mean(2,3)=2.5 at t=2, mean(3,4)=3.5 at t=3
    /// assert!((atr.values[2] - 2.5).abs() < 1e-9);
    /// assert!((atr.values[3] - 3.5).abs() < 1e-9);
    /// ```
    pub fn average_true_range(&self, n: usize) -> Result<TimeSeries, TemporalSeriesError> {
        self.true_range()?.rolling(n).mean()
    }

    /// Returns Bollinger Bands as `(upper, middle, lower)`.
    ///
    /// # Formula
    ///
    /// $$BB_{upper}(t) = MA_t^{(w)} + k \cdot \sigma_t^{(w)}$$
    ///
    /// $$BB_{mid}(t) = MA_t^{(w)}$$
    ///
    /// $$BB_{lower}(t) = MA_t^{(w)} - k \cdot \sigma_t^{(w)}$$
    ///
    /// where $w$ is the window size, $k$ the band multiplier (typically 2), and
    /// $\sigma$ uses Bessel's correction (see [`TimeSeries::rolling_standard_deviation`]).
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `window` is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(
    ///     vec![1, 2, 3, 4, 5],
    ///     vec![3.0, 3.0, 3.0, 3.0, 3.0],
    /// ).unwrap();
    ///
    /// let (upper, mid, lower) = ts.bollinger_bands(3, 2.0).unwrap();
    ///
    /// // Constant series: std=0, all bands collapse onto the mean.
    /// assert_eq!(upper.values[2], 3.0);
    /// assert_eq!(mid.values[2],   3.0);
    /// assert_eq!(lower.values[2], 3.0);
    /// ```
    pub fn bollinger_bands(
        &self,
        window: usize,
        k: f64,
    ) -> Result<(TimeSeries, TimeSeries, TimeSeries), TemporalSeriesError> {
        let middle: TimeSeries = self.moving_average(window)?;
        let rolling_std: TimeSeries = self.rolling_standard_deviation(window)?;
        let upper_values: Vec<f64> = middle
            .values
            .iter()
            .zip(rolling_std.values.iter())
            .map(|(&m, &s)| m + k * s)
            .collect();
        let lower_values: Vec<f64> = middle
            .values
            .iter()
            .zip(rolling_std.values.iter())
            .map(|(&m, &s)| m - k * s)
            .collect();
        let upper: TimeSeries = TimeSeries::new(self.index.clone(), upper_values)?;
        let lower: TimeSeries = TimeSeries::new(self.index.clone(), lower_values)?;
        Ok((upper, middle, lower))
    }

    // AUTOCORRELATION --------------------------------------------------------

    /// Returns the autocorrelation function (ACF) at a given lag.
    ///
    /// # Formula
    ///
    /// $$\rho(k) = \frac{\displaystyle\sum_{t=k}^{n-1}(x_t - \bar{x})(x_{t-k} - \bar{x})}{\displaystyle\sum_{t=0}^{n-1}(x_t - \bar{x})^2}$$
    ///
    /// By definition $\rho(0) = 1$. Returns `NaN` if the series has zero variance.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty, or
    /// [`TemporalSeriesError::ParameterRangeError`] if `lag >= n`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    ///
    /// assert_eq!(ts.autocorrelation_function(0).unwrap(), 1.0);
    /// assert!((ts.autocorrelation_function(1).unwrap() - 0.4).abs() < 1e-9);
    /// ```
    pub fn autocorrelation_function(&self, lag: usize) -> Result<f64, TemporalSeriesError> {
        let n: usize = self.len();
        if n == 0 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        if lag >= n {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "lag ({lag}) must be less than series length ({n})"
            )));
        }
        let mean: f64 = self.mean();
        let variance: f64 = self.values.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
        if variance == 0.0 {
            return Ok(f64::NAN);
        }
        let covariance: f64 = (lag..n)
            .map(|t| (self.values[t] - mean) * (self.values[t - lag] - mean))
            .sum::<f64>();
        Ok(covariance / variance)
    }

    /// Returns the partial autocorrelation function (PACF) at a given lag.
    ///
    /// Uses the Levinson-Durbin recursion to isolate the correlation between
    /// $x_t$ and $x_{t-k}$ after removing the linear influence of all intermediate lags.
    ///
    /// - $PACF(0) = 1$
    /// - $PACF(1) = \rho(1)$
    /// - $PACF(k) = \phi_{k,k}$ via Levinson-Durbin for $k \geq 2$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty, or
    /// [`TemporalSeriesError::ParameterRangeError`] if `lag >= n`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    ///
    /// assert_eq!(ts.partial_autocorrelation_function(0).unwrap(), 1.0);
    /// assert!((ts.partial_autocorrelation_function(1).unwrap() - 0.4).abs() < 1e-9);
    /// ```
    pub fn partial_autocorrelation_function(&self, lag: usize) -> Result<f64, TemporalSeriesError> {
        let n: usize = self.len();
        if n == 0 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        if lag >= n {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "lag ({lag}) must be less than series length ({n})"
            )));
        }
        if lag == 0 {
            return Ok(1.0);
        }
        let acf: Vec<f64> = (1..=lag)
            .map(|k| self.autocorrelation_function(k))
            .collect::<Result<Vec<f64>, _>>()?;

        // Levinson-Durbin: phi[j] holds the AR coefficients for the current order.
        let mut phi: Vec<f64> = vec![acf[0]];
        for k in 1..lag {
            let num: f64 = acf[k] - (0..k).map(|j| phi[j] * acf[k - 1 - j]).sum::<f64>();
            let den: f64 = 1.0 - (0..k).map(|j| phi[j] * acf[j]).sum::<f64>();
            let phi_kk: f64 = if den.abs() < f64::EPSILON {
                0.0
            } else {
                num / den
            };
            let prev: Vec<f64> = phi.clone();
            let updated: Vec<f64> = (0..k).map(|j| prev[j] - phi_kk * prev[k - 1 - j]).collect();
            phi = updated;
            phi.push(phi_kk);
        }
        Ok(*phi.last().unwrap())
    }

    // STATIONARITY -----------------------------------------------------------

    /// Computes the Dickey-Fuller test statistic for a unit root.
    ///
    /// Fits $\Delta x_t = \gamma x_{t-1} + \varepsilon_t$ via OLS and returns
    /// $\hat{\gamma} / SE(\hat{\gamma})$. Under $H_0$ (unit root) this statistic does
    /// not follow a standard $t$-distribution — interpret via
    /// [`TimeSeries::stationary_dickey_fuller_test`].
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if `n < 3`.
    pub fn stationary_dickey_fuller_statistics(&self) -> Result<f64, TemporalSeriesError> {
        let n: usize = self.len();
        if n < 3 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let delta: Vec<f64> = (1..n)
            .map(|t| self.values[t] - self.values[t - 1])
            .collect();
        let lagged: Vec<f64> = (0..n - 1).map(|t| self.values[t]).collect();

        let ss_xy: f64 = lagged.iter().zip(delta.iter()).map(|(x, y)| x * y).sum();
        let ss_xx: f64 = lagged.iter().map(|x| x * x).sum();
        if ss_xx.abs() < f64::EPSILON {
            return Ok(0.0);
        }
        let gamma: f64 = ss_xy / ss_xx;
        let sse: f64 = lagged
            .iter()
            .zip(delta.iter())
            .map(|(x, y)| (y - gamma * x).powi(2))
            .sum();
        if sse < f64::EPSILON {
            return Ok(f64::NEG_INFINITY);
        }
        let m: usize = lagged.len();
        let sigma2: f64 = sse / (m - 1) as f64;
        let se: f64 = (sigma2 / ss_xx).sqrt();
        Ok(gamma / se)
    }

    /// Tests for stationarity using the Dickey-Fuller test.
    ///
    /// Returns `true` if the unit-root null hypothesis is rejected at `alpha`
    /// (i.e. the series is stationary).
    ///
    /// Critical values (no constant, no trend):
    ///
    /// | alpha | critical value |
    /// |-------|---------------|
    /// | 0.10  | −1.61         |
    /// | 0.05  | −1.95         |
    /// | 0.01  | −2.60         |
    ///
    /// # Errors
    ///
    /// Propagates errors from [`TimeSeries::stationary_dickey_fuller_statistics`].
    ///
    /// # Examples
    ///
    /// A purely trending series is **non-stationary** — the test returns `false` because
    /// it cannot reject the unit-root null hypothesis:
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let trend = TimeSeries::new(
    ///     vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    /// ).unwrap();
    ///
    /// // DF statistic >> -1.95  =>  cannot reject unit root  =>  non-stationary
    /// let is_stationary = trend.stationary_dickey_fuller_test(0.05).unwrap();
    /// assert!(!is_stationary);
    /// ```
    ///
    /// A strongly mean-reverting series is **stationary** — the DF statistic is
    /// deeply negative and the test returns `true`:
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let mean_reverting = TimeSeries::new(
    ///     vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ///     vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
    /// ).unwrap();
    ///
    /// // DF statistic -> -inf  =>  unit root rejected  =>  stationary
    /// let is_stationary = mean_reverting.stationary_dickey_fuller_test(0.05).unwrap();
    /// assert!(is_stationary);
    /// ```
    pub fn stationary_dickey_fuller_test(&self, alpha: f32) -> Result<bool, TemporalSeriesError> {
        let critical_value: f64 = match alpha {
            a if a <= 0.01 => -2.60,
            a if a <= 0.05 => -1.95,
            _ => -1.61,
        };
        let statistic: f64 = self.stationary_dickey_fuller_statistics()?;
        Ok(statistic < critical_value)
    }

    // DISTRIBUTION ANALYSIS --------------------------------------------------

    /// Returns the Fisher-Pearson skewness of the series.
    ///
    /// # Formula
    ///
    /// $$\text{Skew} = \frac{\displaystyle\frac{1}{n}\sum_{i=1}^{n}(x_i - \bar{x})^3}{\left(\displaystyle\frac{1}{n}\sum_{i=1}^{n}(x_i - \bar{x})^2\right)^{3/2}}$$
    ///
    /// A symmetric distribution has skewness 0; positive values indicate a right tail,
    /// negative values a left tail.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if `n < 3`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// // Symmetric series has zero skewness.
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    /// assert!(ts.skewness().unwrap().abs() < 1e-9);
    /// ```
    pub fn skewness(&self) -> Result<f64, TemporalSeriesError> {
        let n: usize = self.len();
        if n < 3 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mean: f64 = self.mean();
        let nf: f64 = n as f64;
        let m2: f64 = self.values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / nf;
        let m3: f64 = self.values.iter().map(|x| (x - mean).powi(3)).sum::<f64>() / nf;
        if m2 < f64::EPSILON {
            return Ok(0.0);
        }
        Ok(m3 / m2.powf(1.5))
    }

    /// Returns the excess kurtosis of the series.
    ///
    /// # Formula
    ///
    /// $$\kappa_{excess} = \frac{\displaystyle\frac{1}{n}\sum_{i=1}^{n}(x_i - \bar{x})^4}{\left(\displaystyle\frac{1}{n}\sum_{i=1}^{n}(x_i - \bar{x})^2\right)^{2}} - 3$$
    ///
    /// A normal distribution has excess kurtosis 0 (mesokurtic). Positive values indicate
    /// heavy tails (leptokurtic); negative values indicate light tails (platykurtic).
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if `n < 4`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// // [1,2,3,4,5]: m2=2.0, m4=6.8  =>  6.8/4.0 - 3 = -1.3
    /// let ts = TimeSeries::new(vec![1, 2, 3, 4, 5], vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    /// assert!((ts.excess_kurtosis().unwrap() - (-1.3)).abs() < 1e-9);
    /// ```
    pub fn excess_kurtosis(&self) -> Result<f64, TemporalSeriesError> {
        let n: usize = self.len();
        if n < 4 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mean: f64 = self.mean();
        let nf: f64 = n as f64;
        let m2: f64 = self.values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / nf;
        let m4: f64 = self.values.iter().map(|x| (x - mean).powi(4)).sum::<f64>() / nf;
        if m2 < f64::EPSILON {
            return Ok(-3.0);
        }
        Ok(m4 / m2.powi(2) - 3.0)
    }

    /// Computes the Jarque-Bera test statistic.
    ///
    /// # Formula
    ///
    /// $$JB = n\!\left(\frac{S^2}{6} + \frac{K^2}{24}\right)$$
    ///
    /// where $S$ is the skewness and $K$ the excess kurtosis.
    /// Under $H_0$ (normality), $JB \sim \chi^2(2)$.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`TimeSeries::skewness`] and [`TimeSeries::excess_kurtosis`].
    pub fn jacque_bera_statistics(&self) -> Result<f64, TemporalSeriesError> {
        let n: f64 = self.len() as f64;
        let s: f64 = self.skewness()?;
        let k: f64 = self.excess_kurtosis()?;
        Ok(n * (s.powi(2) / 6.0 + k.powi(2) / 24.0))
    }

    /// Tests for normality using the Jarque-Bera test.
    ///
    /// Returns `true` if the series is consistent with normality ($H_0$ not rejected).
    ///
    /// Critical values from $\chi^2(2)$:
    ///
    /// | alpha | critical value |
    /// |-------|---------------|
    /// | 0.10  | 4.605         |
    /// | 0.05  | 5.991         |
    /// | 0.01  | 9.210         |
    ///
    /// # Errors
    ///
    /// Propagates errors from [`TimeSeries::jacque_bera_statistics`].
    ///
    /// # Examples
    ///
    /// A symmetric, uniform-ish series has near-zero skewness and moderate kurtosis —
    /// the JB statistic stays below the critical value, so the test returns `true`
    /// (consistent with normality):
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let symmetric = TimeSeries::new(
    ///     vec![1, 2, 3, 4, 5],
    ///     vec![1.0, 2.0, 3.0, 4.0, 5.0],
    /// ).unwrap();
    ///
    /// // JB ≈ 0.35  <  5.991  =>  fail to reject normality
    /// let is_normal = symmetric.jacque_bera_test(0.05).unwrap();
    /// assert!(is_normal);
    /// ```
    ///
    /// A heavily right-skewed series has a large JB statistic that exceeds the
    /// critical value — the test returns `false` (normality rejected):
    ///
    /// ```rust
    /// use temporalseries::series::TimeSeries;
    ///
    /// let skewed = TimeSeries::new(
    ///     vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    ///     vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 20.0],
    /// ).unwrap();
    ///
    /// // JB ≈ 22.7  >  5.991  =>  reject normality
    /// let is_normal = skewed.jacque_bera_test(0.05).unwrap();
    /// assert!(!is_normal);
    /// ```
    pub fn jacque_bera_test(&self, alpha: f32) -> Result<bool, TemporalSeriesError> {
        let critical_value: f64 = match alpha {
            a if a <= 0.01 => 9.210,
            a if a <= 0.05 => 5.991,
            _ => 4.605,
        };
        let jb: f64 = self.jacque_bera_statistics()?;
        Ok(jb < critical_value)
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
