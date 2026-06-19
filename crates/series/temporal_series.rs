use std::marker::PhantomData;

use crate::errors::TemporalSeriesError;
use crate::storage::StorageBackend;
use crate::time::TimeUnit;

/// A generic, backend-agnostic time series.
///
/// `TemporalSeries<T, B>` pairs an ordered sequence of integer timestamps
/// (`index`) with a pluggable storage backend `B` that holds values of type
/// `T`. The backend abstraction lets you choose the in-memory layout that
/// best fits your workload without changing the series API.
///
/// Timestamps are stored as `i64` values whose unit is tracked by an optional
/// [`TimeUnit`] field. When the unit is set, you can convert the index to and
/// from [`chrono::DateTime<Utc>`] values using [`TemporalSeries::from_datetimes`]
/// and [`TemporalSeries::datetimes`] (requires the `chrono` feature).
///
/// # Type parameters
///
/// - `T` — the element type stored in the series (e.g. `f64`, `i32`, or any
///   custom type that satisfies the bounds required by your chosen backend).
/// - `B` — the storage backend, which must implement [`StorageBackend<T>`].
///   Two implementations are provided out of the box:
///   - [`ColumnarBackend<T>`](crate::storage::ColumnarBackend) — values in a
///     single contiguous `Vec<T>`; best for column-scan workloads.
///   - [`RowBackend<T>`](crate::storage::RowBackend) — each observation is a
///     `RowRecord { timestamp, value }`; natural for record-at-a-time
///     ingestion.
///
/// # Invariant
///
/// `index.len() == storage.len()` at all times. This is enforced at
/// construction by [`TemporalSeries::new`], which returns
/// [`TemporalSeriesError::LengthMismatch`] if the lengths differ.
///
/// # Example
///
/// ```rust
/// use temporalseries::series::TemporalSeries;
/// use temporalseries::storage::ColumnarBackend;
/// use temporalseries::time::TimeUnit;
///
/// let index = vec![0_i64, 1_000, 2_000];
/// let backend = ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]);
/// let series = TemporalSeries::new(index, backend)
///     .unwrap()
///     .with_unit(TimeUnit::Milliseconds);
///
/// assert_eq!(series.len(), 3);
/// assert_eq!(series.time_unit(), Some(&TimeUnit::Milliseconds));
/// ```
pub struct TemporalSeries<T, B>
where
    B: StorageBackend<T>,
{
    /// Ordered sequence of integer timestamps, one per observation.
    pub index: Vec<i64>,
    storage: B,
    /// The unit in which `index` values are expressed, if known.
    time_unit: Option<TimeUnit>,
    /// Ties `T` to the struct without storing a `T` directly.
    _marker: PhantomData<T>,
}

impl<T, B: StorageBackend<T>> TemporalSeries<T, B> {
    /// Creates a new `TemporalSeries` pairing an index with a storage backend.
    ///
    /// The time unit is left unset; call [`with_unit`](Self::with_unit) to
    /// attach one.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::LengthMismatch`] if `index.len()` differs
    /// from `storage.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![1_i64, 2, 3],
    ///     ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]),
    /// ).unwrap();
    ///
    /// assert_eq!(series.len(), 3);
    /// ```
    pub fn new(index: Vec<i64>, storage: B) -> Result<Self, TemporalSeriesError> {
        if index.len() != storage.len() {
            return Err(TemporalSeriesError::LengthMismatch {
                index_len: index.len(),
                values_len: storage.len(),
            });
        }
        Ok(Self {
            index,
            storage,
            time_unit: None,
            _marker: PhantomData,
        })
    }

    /// Attaches a [`TimeUnit`] to the series, describing the unit of `index`.
    ///
    /// This is a consuming builder method — it returns `Self` so it can be
    /// chained directly after construction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    /// use temporalseries::time::TimeUnit;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![0_i64, 1_000, 2_000],
    ///     ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]),
    /// )
    /// .unwrap()
    /// .with_unit(TimeUnit::Milliseconds);
    ///
    /// assert_eq!(series.time_unit(), Some(&TimeUnit::Milliseconds));
    /// ```
    pub fn with_unit(mut self, unit: TimeUnit) -> Self {
        self.time_unit = Some(unit);
        self
    }

    /// Returns a reference to the [`TimeUnit`] if one has been set, or `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    /// use temporalseries::time::TimeUnit;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![0_i64],
    ///     ColumnarBackend::new(vec![1.0_f64]),
    /// )
    /// .unwrap();
    ///
    /// assert_eq!(series.time_unit(), None);
    ///
    /// let series = series.with_unit(TimeUnit::Seconds);
    /// assert_eq!(series.time_unit(), Some(&TimeUnit::Seconds));
    /// ```
    pub fn time_unit(&self) -> Option<&TimeUnit> {
        self.time_unit.as_ref()
    }

    /// Returns the number of observations in the series.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![1_i64, 2, 3],
    ///     ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]),
    /// ).unwrap();
    ///
    /// assert_eq!(series.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the series contains no observations.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    ///
    /// let empty: TemporalSeries<f64, ColumnarBackend<f64>> =
    ///     TemporalSeries::new(vec![], ColumnarBackend::new(vec![])).unwrap();
    ///
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns a reference to the value at position `idx`, or `None` if out of bounds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![1_i64, 2, 3],
    ///     ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]),
    /// ).unwrap();
    ///
    /// assert_eq!(series.get(1), Some(&20.0));
    /// assert_eq!(series.get(99), None);
    /// ```
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.storage.get(idx)
    }

    /// Returns an iterator over references to each value in order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![1_i64, 2, 3],
    ///     ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]),
    /// ).unwrap();
    ///
    /// let sum: f64 = series.iter().copied().sum();
    /// assert_eq!(sum, 6.0);
    /// ```
    pub fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        self.storage.iter()
    }

    /// Builds a `TemporalSeries` from a slice of [`chrono::DateTime<Utc>`] values.
    ///
    /// Each datetime is converted to `i64` using `unit`, which is also stored
    /// on the series so [`datetimes`](Self::datetimes) can round-trip back.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::LengthMismatch`] if `datetimes.len()`
    /// differs from `storage.len()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    /// use temporalseries::time::TimeUnit;
    /// use chrono::{DateTime, Utc, TimeZone};
    ///
    /// let datetimes: Vec<DateTime<Utc>> = vec![
    ///     Utc.timestamp_opt(0, 0).unwrap(),
    ///     Utc.timestamp_opt(1, 0).unwrap(),
    ///     Utc.timestamp_opt(2, 0).unwrap(),
    /// ];
    ///
    /// let series = TemporalSeries::from_datetimes(
    ///     datetimes,
    ///     ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]),
    ///     TimeUnit::Seconds,
    /// ).unwrap();
    ///
    /// assert_eq!(series.index, vec![0, 1, 2]);
    /// assert_eq!(series.time_unit(), Some(&TimeUnit::Seconds));
    /// ```
    #[cfg(feature = "chrono")]
    pub fn from_datetimes(
        datetimes: Vec<chrono::DateTime<chrono::Utc>>,
        storage: B,
        unit: TimeUnit,
    ) -> Result<Self, TemporalSeriesError> {
        let index: Vec<i64> = datetimes.iter().map(|dt| unit.from_datetime(*dt)).collect();
        Self::new(index, storage).map(|s| s.with_unit(unit))
    }

    /// Returns the index converted to [`chrono::DateTime<Utc>`] values.
    ///
    /// Returns `None` if no [`TimeUnit`] has been set or if any timestamp is
    /// out of the representable range.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::series::TemporalSeries;
    /// use temporalseries::storage::ColumnarBackend;
    /// use temporalseries::time::TimeUnit;
    ///
    /// let series = TemporalSeries::new(
    ///     vec![0_i64, 1, 2],
    ///     ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]),
    /// )
    /// .unwrap()
    /// .with_unit(TimeUnit::Seconds);
    ///
    /// let dts = series.datetimes().unwrap();
    /// assert_eq!(dts[0].timestamp(), 0);
    /// assert_eq!(dts[2].timestamp(), 2);
    /// ```
    #[cfg(feature = "chrono")]
    pub fn datetimes(&self) -> Option<Vec<chrono::DateTime<chrono::Utc>>> {
        let unit = self.time_unit.as_ref()?;
        self.index.iter().map(|&ts| unit.to_datetime(ts)).collect()
    }
}

// ---------------------------------------------------------------------------
// Analytical methods for f64-valued series
// ---------------------------------------------------------------------------

/// Concrete output type produced by series-returning analytical methods.
///
/// All methods that return a new series use [`crate::storage::ColumnarBackend`]
/// regardless of the input backend, since there is no generic way to construct
/// an arbitrary `B` from a `Vec<f64>`.
pub type ColSeries = TemporalSeries<f64, crate::storage::ColumnarBackend<f64>>;

impl<B: StorageBackend<f64>> TemporalSeries<f64, B> {
    fn values_vec(&self) -> Vec<f64> {
        self.iter().copied().collect()
    }

    fn series_from(index: Vec<i64>, values: Vec<f64>) -> Result<ColSeries, TemporalSeriesError> {
        TemporalSeries::new(index, crate::storage::ColumnarBackend::new(values))
    }

    // STATISTICS -------------------------------------------------------------

    /// Returns the arithmetic mean of the series.
    ///
    /// $$\hat{\mu} = \frac{1}{n} \sum_{i=1}^{n} x_i$$
    pub fn mean(&self) -> f64 {
        let total: f64 = self.iter().sum();
        total / self.len() as f64
    }

    /// Returns the sample variance (Bessel-corrected, denominator `n − 1`).
    ///
    /// Named `std_deviation` to mirror [`crate::series::TimeSeries::std_deviation`],
    /// which computes the same quantity.
    pub fn std_deviation(&self) -> f64 {
        let mean = self.mean();
        let n = self.len() as f64;
        if n <= 1.0 {
            return 0.0;
        }
        let sum_sq: f64 = self.iter().map(|x| (x - mean).powi(2)).sum();
        sum_sq / (n - 1.0)
    }

    /// Returns the p-th quantile using linear interpolation (numpy `method='linear'`).
    ///
    /// # Errors
    ///
    /// - [`TemporalSeriesError::ParameterRangeError`] if `p` is outside `[0.0, 1.0]`.
    /// - [`TemporalSeriesError::EmptySeries`] if the series has no non-NaN values.
    pub fn quantile(&self, p: f32) -> Result<f64, TemporalSeriesError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "p must be in [0.0, 1.0], got {p}"
            )));
        }
        let mut sorted: Vec<f64> = self.iter().copied().filter(|v| !v.is_nan()).collect();
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

    /// Returns the Interquartile Range: Q3 − Q1.
    ///
    /// # Errors
    ///
    /// - [`TemporalSeriesError::EmptySeries`] if the series has no non-NaN values.
    pub fn iqr(&self) -> Result<f64, TemporalSeriesError> {
        Ok(self.quantile(0.75)? - self.quantile(0.25)?)
    }

    // RETURNS ----------------------------------------------------------------

    /// Returns the per-period simple return series.
    ///
    /// $$r_t = \frac{x_t - x_{t-1}}{x_{t-1}}$$
    ///
    /// The first element is `NaN`.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    pub fn simple_return(&self) -> Result<ColSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let vals = self.values_vec();
        let mut out = vec![f64::NAN; vals.len()];
        for (r, w) in out[1..].iter_mut().zip(vals.windows(2)) {
            *r = (w[1] - w[0]) / w[0];
        }
        Self::series_from(self.index.clone(), out)
    }

    /// Returns the per-period logarithmic return series.
    ///
    /// $$r_t^{log} = \ln\!\left(\frac{x_t}{x_{t-1}}\right)$$
    ///
    /// The first element is `NaN`.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    pub fn log_return(&self) -> Result<ColSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let vals = self.values_vec();
        let mut out = vec![f64::NAN; vals.len()];
        for (r, w) in out[1..].iter_mut().zip(vals.windows(2)) {
            *r = (w[1] / w[0]).ln();
        }
        Self::series_from(self.index.clone(), out)
    }

    /// Returns the total cumulative return from the first to the last observation.
    ///
    /// $$R_{cum} = \frac{x_T - x_0}{x_0}$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    pub fn cumulative_return(&self) -> Result<f64, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let x0 = *self.get(0).unwrap();
        let xt = *self.get(self.len() - 1).unwrap();
        Ok((xt - x0) / x0)
    }

    // MOVING AVERAGES --------------------------------------------------------

    /// Returns the n-period simple moving average series.
    ///
    /// $$MA_t^{(n)} = \frac{1}{n} \sum_{i=0}^{n-1} x_{t-i}$$
    ///
    /// The first `n − 1` elements are `NaN`.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `n` exceeds the series length.
    pub fn moving_average(&self, n: usize) -> Result<ColSeries, TemporalSeriesError> {
        if n > self.len() {
            return Err(TemporalSeriesError::InvalidWindow {
                window: n,
                series_len: self.len(),
            });
        }
        let vals = self.values_vec();
        let len = vals.len();
        let mut out = vec![f64::NAN; len];
        for (i, v) in out.iter_mut().enumerate().skip(n - 1) {
            let window = &vals[i + 1 - n..=i];
            *v = window.iter().sum::<f64>() / n as f64;
        }
        Self::series_from(self.index.clone(), out)
    }

    /// Returns the exponential moving average (EMA) series for a given span.
    ///
    /// $$\alpha = \frac{2}{span + 1}, \qquad EMA_t = \alpha \cdot x_t + (1 - \alpha) \cdot EMA_{t-1}$$
    ///
    /// Seeded with $EMA_0 = x_0$.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    pub fn exponential_moving_average(
        &self,
        span: usize,
    ) -> Result<ColSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let vals = self.values_vec();
        let alpha = 2.0 / (span as f64 + 1.0);
        let mut out = vec![0.0f64; vals.len()];
        out[0] = vals[0];
        for i in 1..vals.len() {
            out[i] = alpha * vals[i] + (1.0 - alpha) * out[i - 1];
        }
        Self::series_from(self.index.clone(), out)
    }

    /// Returns the MA crossover signal series.
    ///
    /// - `+1.0` — fast MA crosses **above** slow MA (bullish)
    /// - `-1.0` — fast MA crosses **below** slow MA (bearish)
    /// - `0.0` — no crossover
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::ParameterRangeError`] if `fast >= slow`, or
    /// [`TemporalSeriesError::InvalidWindow`] if either window exceeds the series length.
    pub fn crossover_signal(
        &self,
        fast: usize,
        slow: usize,
    ) -> Result<ColSeries, TemporalSeriesError> {
        if fast >= slow {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "fast window ({fast}) must be smaller than slow window ({slow})"
            )));
        }
        let fast_vals: Vec<f64> = self.moving_average(fast)?.iter().copied().collect();
        let slow_vals: Vec<f64> = self.moving_average(slow)?.iter().copied().collect();
        let n = self.len();
        let mut signals = vec![0.0f64; n];
        for (i, sig) in signals.iter_mut().enumerate().skip(1) {
            let prev = fast_vals[i - 1] - slow_vals[i - 1];
            let curr = fast_vals[i] - slow_vals[i];
            if prev.is_nan() || curr.is_nan() {
                continue;
            }
            if prev <= 0.0 && curr > 0.0 {
                *sig = 1.0;
            } else if prev >= 0.0 && curr < 0.0 {
                *sig = -1.0;
            }
        }
        Self::series_from(self.index.clone(), signals)
    }

    // VOLATILITY -------------------------------------------------------------

    /// Returns the n-period rolling standard deviation series (Bessel-corrected).
    ///
    /// $$\sigma_t^{(n)} = \sqrt{\frac{1}{n-1} \sum_{i=0}^{n-1} \left(x_{t-i} - \bar{x}_t^{(n)}\right)^2}$$
    ///
    /// The first `n − 1` elements are `NaN`.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `n > len` or `n < 2`.
    pub fn rolling_standard_deviation(&self, n: usize) -> Result<ColSeries, TemporalSeriesError> {
        if n < 2 || n > self.len() {
            return Err(TemporalSeriesError::InvalidWindow {
                window: n,
                series_len: self.len(),
            });
        }
        let vals = self.values_vec();
        let len = vals.len();
        let mut out = vec![f64::NAN; len];
        for (i, v) in out.iter_mut().enumerate().skip(n - 1) {
            let window = &vals[i + 1 - n..=i];
            let mean: f64 = window.iter().sum::<f64>() / n as f64;
            let variance: f64 =
                window.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64;
            *v = variance.sqrt();
        }
        Self::series_from(self.index.clone(), out)
    }

    /// Returns the per-period true range series.
    ///
    /// $$TR_t = \left|x_t - x_{t-1}\right|$$
    ///
    /// The first element is `NaN`.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty.
    pub fn true_range(&self) -> Result<ColSeries, TemporalSeriesError> {
        if self.is_empty() {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let vals = self.values_vec();
        let mut out = vec![f64::NAN; vals.len()];
        for (r, w) in out[1..].iter_mut().zip(vals.windows(2)) {
            *r = (w[1] - w[0]).abs();
        }
        Self::series_from(self.index.clone(), out)
    }

    /// Returns the n-period average true range (ATR) series.
    ///
    /// $$ATR_t^{(n)} = \frac{1}{n} \sum_{i=0}^{n-1} TR_{t-i}, \qquad TR_t = \left|x_t - x_{t-1}\right|$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `n` is too large for the series.
    pub fn average_true_range(&self, n: usize) -> Result<ColSeries, TemporalSeriesError> {
        self.true_range()?.moving_average(n)
    }

    /// Returns Bollinger Bands as `(upper, middle, lower)`.
    ///
    /// $$BB_{upper}(t) = MA_t^{(w)} + k \cdot \sigma_t^{(w)}, \quad BB_{lower}(t) = MA_t^{(w)} - k \cdot \sigma_t^{(w)}$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::InvalidWindow`] if `window` is invalid.
    pub fn bollinger_bands(
        &self,
        window: usize,
        k: f64,
    ) -> Result<(ColSeries, ColSeries, ColSeries), TemporalSeriesError> {
        let middle = self.moving_average(window)?;
        let rolling_std = self.rolling_standard_deviation(window)?;
        let upper_values: Vec<f64> = middle
            .iter()
            .copied()
            .zip(rolling_std.iter().copied())
            .map(|(m, s)| m + k * s)
            .collect();
        let lower_values: Vec<f64> = middle
            .iter()
            .copied()
            .zip(rolling_std.iter().copied())
            .map(|(m, s)| m - k * s)
            .collect();
        let upper = Self::series_from(self.index.clone(), upper_values)?;
        let lower = Self::series_from(self.index.clone(), lower_values)?;
        Ok((upper, middle, lower))
    }

    // AUTOCORRELATION --------------------------------------------------------

    /// Returns the autocorrelation function (ACF) at a given lag.
    ///
    /// $$\rho(k) = \frac{\displaystyle\sum_{t=k}^{n-1}(x_t - \bar{x})(x_{t-k} - \bar{x})}{\displaystyle\sum_{t=0}^{n-1}(x_t - \bar{x})^2}$$
    ///
    /// By definition $\rho(0) = 1$. Returns `NaN` if the series has zero variance.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty, or
    /// [`TemporalSeriesError::ParameterRangeError`] if `lag >= n`.
    pub fn autocorrelation_function(&self, lag: usize) -> Result<f64, TemporalSeriesError> {
        let n = self.len();
        if n == 0 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        if lag >= n {
            return Err(TemporalSeriesError::ParameterRangeError(format!(
                "lag ({lag}) must be less than series length ({n})"
            )));
        }
        let vals = self.values_vec();
        let mean = self.mean();
        let variance: f64 = vals.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
        if variance == 0.0 {
            return Ok(f64::NAN);
        }
        let covariance: f64 = (lag..n)
            .map(|t| (vals[t] - mean) * (vals[t - lag] - mean))
            .sum::<f64>();
        Ok(covariance / variance)
    }

    /// Returns the partial autocorrelation function (PACF) at a given lag
    /// via Levinson-Durbin recursion.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if the series is empty, or
    /// [`TemporalSeriesError::ParameterRangeError`] if `lag >= n`.
    pub fn partial_autocorrelation_function(&self, lag: usize) -> Result<f64, TemporalSeriesError> {
        let n = self.len();
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
        let mut phi: Vec<f64> = vec![acf[0]];
        for k in 1..lag {
            let num: f64 = acf[k] - (0..k).map(|j| phi[j] * acf[k - 1 - j]).sum::<f64>();
            let den: f64 = 1.0 - (0..k).map(|j| phi[j] * acf[j]).sum::<f64>();
            let phi_kk = if den.abs() < f64::EPSILON {
                0.0
            } else {
                num / den
            };
            let prev = phi.clone();
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
    /// $\hat{\gamma} / SE(\hat{\gamma})$.
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if `n < 3`.
    pub fn stationary_dickey_fuller_statistics(&self) -> Result<f64, TemporalSeriesError> {
        let n = self.len();
        if n < 3 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let vals = self.values_vec();
        let delta: Vec<f64> = (1..n).map(|t| vals[t] - vals[t - 1]).collect();
        let lagged: Vec<f64> = (0..n - 1).map(|t| vals[t]).collect();
        let ss_xy: f64 = lagged.iter().zip(delta.iter()).map(|(x, y)| x * y).sum();
        let ss_xx: f64 = lagged.iter().map(|x| x * x).sum();
        if ss_xx.abs() < f64::EPSILON {
            return Ok(0.0);
        }
        let gamma = ss_xy / ss_xx;
        let sse: f64 = lagged
            .iter()
            .zip(delta.iter())
            .map(|(x, y)| (y - gamma * x).powi(2))
            .sum();
        if sse < f64::EPSILON {
            return Ok(f64::NEG_INFINITY);
        }
        let m = lagged.len();
        let sigma2 = sse / (m - 1) as f64;
        let se = (sigma2 / ss_xx).sqrt();
        Ok(gamma / se)
    }

    /// Tests for stationarity using the Dickey-Fuller test.
    ///
    /// Returns `true` if the unit-root null hypothesis is rejected at `alpha`
    /// (i.e. the series is stationary).
    ///
    /// | `alpha` | Critical value |
    /// |---------|---------------|
    /// | 0.01    | −2.60         |
    /// | 0.05    | −1.95         |
    /// | 0.10    | −1.61         |
    ///
    /// # Errors
    ///
    /// Propagates errors from [`Self::stationary_dickey_fuller_statistics`].
    pub fn stationary_dickey_fuller_test(&self, alpha: f32) -> Result<bool, TemporalSeriesError> {
        let cv = match alpha {
            a if a <= 0.01 => -2.60,
            a if a <= 0.05 => -1.95,
            _ => -1.61,
        };
        Ok(self.stationary_dickey_fuller_statistics()? < cv)
    }

    // DISTRIBUTION ANALYSIS --------------------------------------------------

    /// Returns the Fisher-Pearson skewness of the series.
    ///
    /// $$\text{Skew} = \frac{m_3}{m_2^{3/2}}, \quad m_k = \frac{1}{n}\sum_{i=1}^{n}(x_i - \bar{x})^k$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if `n < 3`.
    pub fn skewness(&self) -> Result<f64, TemporalSeriesError> {
        let n = self.len();
        if n < 3 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mean = self.mean();
        let nf = n as f64;
        let m2: f64 = self.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / nf;
        let m3: f64 = self.iter().map(|x| (x - mean).powi(3)).sum::<f64>() / nf;
        if m2 < f64::EPSILON {
            return Ok(0.0);
        }
        Ok(m3 / m2.powf(1.5))
    }

    /// Returns the excess kurtosis of the series.
    ///
    /// $$\kappa_{excess} = \frac{m_4}{m_2^2} - 3$$
    ///
    /// # Errors
    ///
    /// Returns [`TemporalSeriesError::EmptySeries`] if `n < 4`.
    pub fn excess_kurtosis(&self) -> Result<f64, TemporalSeriesError> {
        let n = self.len();
        if n < 4 {
            return Err(TemporalSeriesError::EmptySeries);
        }
        let mean = self.mean();
        let nf = n as f64;
        let m2: f64 = self.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / nf;
        let m4: f64 = self.iter().map(|x| (x - mean).powi(4)).sum::<f64>() / nf;
        if m2 < f64::EPSILON {
            return Ok(-3.0);
        }
        Ok(m4 / m2.powi(2) - 3.0)
    }

    /// Computes the Jarque-Bera test statistic.
    ///
    /// $$JB = n\!\left(\frac{S^2}{6} + \frac{K^2}{24}\right) \sim \chi^2(2)$$
    ///
    /// # Errors
    ///
    /// Propagates errors from [`Self::skewness`] and [`Self::excess_kurtosis`].
    pub fn jacque_bera_statistics(&self) -> Result<f64, TemporalSeriesError> {
        let n = self.len() as f64;
        let s = self.skewness()?;
        let k = self.excess_kurtosis()?;
        Ok(n * (s.powi(2) / 6.0 + k.powi(2) / 24.0))
    }

    /// Tests for normality using the Jarque-Bera test.
    ///
    /// Returns `true` when the series is consistent with normality (null not rejected).
    ///
    /// | `alpha` | χ²(2) critical value |
    /// |---------|---------------------|
    /// | 0.01    | 9.210               |
    /// | 0.05    | 5.991               |
    /// | 0.10    | 4.605               |
    ///
    /// # Errors
    ///
    /// Propagates errors from [`Self::jacque_bera_statistics`].
    pub fn jacque_bera_test(&self, alpha: f32) -> Result<bool, TemporalSeriesError> {
        let cv = match alpha {
            a if a <= 0.01 => 9.210,
            a if a <= 0.05 => 5.991,
            _ => 4.605,
        };
        Ok(self.jacque_bera_statistics()? < cv)
    }
}
