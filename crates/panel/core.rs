use std::collections::HashMap;

use crate::errors::TemporalSeriesError;
use crate::series::TimeSeries;

/// Integer timestamp type used as the shared time axis of a [`Panel`].
pub type Timestamp = i64;

/// A collection of named time series sharing a common index.
///
/// A `Panel` aligns multiple `f64` series — each identified by a symbol name —
/// on a single time axis. All series must have the same length as the index.
/// This is the natural structure for multi-asset market data, sensor grids, or
/// any dataset where several variables are observed at the same timestamps.
///
/// # Invariants
///
/// - `symbols.len() == values.len()` — one series per symbol.
/// - `values[i].len() == index.len()` for every `i` — all series are aligned
///   to the shared index.
///
/// Both invariants are enforced at construction time by [`Panel::new`].
///
/// # Analytical methods
///
/// Every analytical method available on [`TimeSeries`] is also available on
/// `Panel`. Methods are applied **column-by-column**: each column is wrapped
/// in a temporary `TimeSeries`, the corresponding method is called, and the
/// results are collected back into either a `HashMap<String, _>` (for scalar
/// results) or a new `Panel` (for series results).
///
/// This means `panel.mean()["AAPL"]` is always identical to
/// `panel.get_series("AAPL").unwrap().mean()`.
///
/// # Example
///
/// ```rust
/// use temporalseries::panel::Panel;
///
/// let panel = Panel::new(
///     vec![1, 2, 3],
///     vec!["AAPL".into(), "MSFT".into()],
///     vec![
///         vec![150.0, 152.0, 149.0],
///         vec![300.0, 305.0, 298.0],
///     ],
/// ).unwrap();
///
/// assert_eq!(panel.shape(), (3, 2));
/// ```
pub struct Panel {
    index: Vec<Timestamp>,
    symbols: Vec<String>,
    values: Vec<Vec<f64>>,
}

impl Panel {
    /// Creates a new `Panel` from a shared index, a list of symbol names, and
    /// a parallel list of value series.
    ///
    /// # Errors
    ///
    /// - [`TemporalSeriesError::LengthMismatch`] if `symbols.len() != values.len()`.
    /// - [`TemporalSeriesError::LengthMismatch`] if any series in `values` has a
    ///   different length than `index`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2, 3],
    ///     vec!["A".into(), "B".into()],
    ///     vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.n_series(), 2);
    /// ```
    pub fn new(
        index: Vec<Timestamp>,
        symbols: Vec<String>,
        values: Vec<Vec<f64>>,
    ) -> Result<Self, TemporalSeriesError> {
        if symbols.len() != values.len() {
            return Err(TemporalSeriesError::LengthMismatch {
                index_len: symbols.len(),
                values_len: values.len(),
            });
        }

        for series in &values {
            if series.len() != index.len() {
                return Err(TemporalSeriesError::LengthMismatch {
                    index_len: index.len(),
                    values_len: series.len(),
                });
            }
        }

        Ok(Self {
            index,
            symbols,
            values,
        })
    }

    /// Returns the number of time steps (rows) in the panel.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2, 3],
    ///     vec!["A".into()],
    ///     vec![vec![1.0, 2.0, 3.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Returns `true` if the panel contains no time steps.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel: Panel = Panel::new(vec![], vec![], vec![]).unwrap();
    /// assert!(panel.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Returns the number of series (columns) in the panel.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2],
    ///     vec!["A".into(), "B".into(), "C".into()],
    ///     vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.n_series(), 3);
    /// ```
    pub fn n_series(&self) -> usize {
        self.symbols.len()
    }

    /// Returns the symbol names as a slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1],
    ///     vec!["X".into(), "Y".into()],
    ///     vec![vec![1.0], vec![2.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.symbols(), &["X", "Y"]);
    /// ```
    pub fn symbols(&self) -> &[String] {
        &self.symbols
    }

    /// Returns `(n_timestamps, n_series)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2, 3],
    ///     vec!["A".into(), "B".into()],
    ///     vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.shape(), (3, 2));
    /// ```
    pub fn shape(&self) -> (usize, usize) {
        (self.len(), self.n_series())
    }

    /// Returns the series for `symbol` as a [`TimeSeries`], or `None` if the
    /// symbol is not present in the panel.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2, 3],
    ///     vec!["AAPL".into()],
    ///     vec![vec![150.0, 152.0, 149.0]],
    /// ).unwrap();
    ///
    /// let ts = panel.get_series("AAPL").unwrap();
    /// assert_eq!(ts.values, vec![150.0, 152.0, 149.0]);
    ///
    /// assert!(panel.get_series("GOOG").is_none());
    /// ```
    pub fn get_series(&self, symbol: &str) -> Option<TimeSeries> {
        let pos = self.symbols.iter().position(|s| s == symbol)?;
        TimeSeries::new(self.index.clone(), self.values[pos].clone()).ok()
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Wraps column `col` as a [`TimeSeries`]. Panics if the Panel is invalid
    /// (impossible by construction, since `Panel::new` validates lengths).
    fn col_series(&self, col: usize) -> TimeSeries {
        TimeSeries::new(self.index.clone(), self.values[col].clone()).unwrap()
    }

    /// Applies a fallible scalar operation to every column and collects the
    /// results into a `HashMap`. Returns the first error encountered.
    fn scalar_map<F>(&self, f: F) -> Result<HashMap<String, f64>, TemporalSeriesError>
    where
        F: Fn(&TimeSeries) -> Result<f64, TemporalSeriesError>,
    {
        self.symbols
            .iter()
            .enumerate()
            .map(|(i, sym)| Ok((sym.clone(), f(&self.col_series(i))?)))
            .collect()
    }

    /// Applies a fallible bool operation to every column and collects the
    /// results into a `HashMap`. Returns the first error encountered.
    fn bool_map<F>(&self, f: F) -> Result<HashMap<String, bool>, TemporalSeriesError>
    where
        F: Fn(&TimeSeries) -> Result<bool, TemporalSeriesError>,
    {
        self.symbols
            .iter()
            .enumerate()
            .map(|(i, sym)| Ok((sym.clone(), f(&self.col_series(i))?)))
            .collect()
    }

    /// Applies a fallible series operation to every column and assembles the
    /// results into a new `Panel` with the same index and symbols.
    /// Returns the first error encountered.
    fn panel_map<F>(&self, f: F) -> Result<Panel, TemporalSeriesError>
    where
        F: Fn(&TimeSeries) -> Result<TimeSeries, TemporalSeriesError>,
    {
        let mut new_values = Vec::with_capacity(self.n_series());
        for i in 0..self.n_series() {
            new_values.push(f(&self.col_series(i))?.values);
        }
        Panel::new(self.index.clone(), self.symbols.clone(), new_values)
    }

    // -----------------------------------------------------------------------
    // STATISTICS
    // -----------------------------------------------------------------------

    /// Returns the arithmetic mean of each column.
    ///
    /// Delegates to [`TimeSeries::mean`] for every column.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2, 3],
    ///     vec!["A".into()],
    ///     vec![vec![1.0, 2.0, 3.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.mean()["A"], 2.0);
    /// ```
    pub fn mean(&self) -> HashMap<String, f64> {
        self.symbols
            .iter()
            .enumerate()
            .map(|(i, sym)| (sym.clone(), self.col_series(i).mean()))
            .collect()
    }

    /// Returns the sample variance (Bessel-corrected) of each column.
    ///
    /// Delegates to [`TimeSeries::std_deviation`] for every column.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::panel::Panel;
    ///
    /// let panel = Panel::new(
    ///     vec![1, 2, 3],
    ///     vec!["A".into()],
    ///     vec![vec![5.0, 5.0, 5.0]],
    /// ).unwrap();
    ///
    /// assert_eq!(panel.std_deviation()["A"], 0.0);
    /// ```
    pub fn std_deviation(&self) -> HashMap<String, f64> {
        self.symbols
            .iter()
            .enumerate()
            .map(|(i, sym)| (sym.clone(), self.col_series(i).std_deviation()))
            .collect()
    }

    /// Returns the p-th quantile of each column using linear interpolation.
    ///
    /// Delegates to [`TimeSeries::quantile`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn quantile(&self, p: f32) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.quantile(p))
    }

    /// Returns the Interquartile Range (Q3 − Q1) of each column.
    ///
    /// Delegates to [`TimeSeries::iqr`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn iqr(&self) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.iqr())
    }

    // -----------------------------------------------------------------------
    // RETURNS
    // -----------------------------------------------------------------------

    /// Returns a new `Panel` with the per-period simple return of each column.
    ///
    /// $$r_t = \frac{x_t - x_{t-1}}{x_{t-1}}$$
    ///
    /// Delegates to [`TimeSeries::simple_return`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn simple_return(&self) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.simple_return())
    }

    /// Returns a new `Panel` with the per-period logarithmic return of each column.
    ///
    /// $$r_t^{log} = \ln\!\left(\frac{x_t}{x_{t-1}}\right)$$
    ///
    /// Delegates to [`TimeSeries::log_return`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn log_return(&self) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.log_return())
    }

    /// Returns the total cumulative return of each column.
    ///
    /// $$R_{cum} = \frac{x_T - x_0}{x_0}$$
    ///
    /// Delegates to [`TimeSeries::cumulative_return`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn cumulative_return(&self) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.cumulative_return())
    }

    /// Returns a new `Panel` with the first-order difference of each column.
    ///
    /// Delegates to [`TimeSeries::diff`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn diff(&self) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.diff())
    }

    /// Returns a new `Panel` with the percentage change of each column.
    ///
    /// Delegates to [`TimeSeries::pct_change`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn pct_change(&self) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.pct_change())
    }

    /// Returns a new `Panel` with each column shifted forward by `periods`.
    ///
    /// Delegates to [`TimeSeries::shift`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn shift(&self, periods: usize) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.shift(periods))
    }

    // -----------------------------------------------------------------------
    // MOVING AVERAGES
    // -----------------------------------------------------------------------

    /// Returns a new `Panel` with the n-period simple moving average of each column.
    ///
    /// $$MA_t^{(n)} = \frac{1}{n} \sum_{i=0}^{n-1} x_{t-i}$$
    ///
    /// Delegates to [`TimeSeries::moving_average`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn moving_average(&self, n: usize) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.moving_average(n))
    }

    /// Returns a new `Panel` with the exponential moving average of each column.
    ///
    /// $$\alpha = \frac{2}{span + 1}, \qquad EMA_t = \alpha \cdot x_t + (1 - \alpha) \cdot EMA_{t-1}$$
    ///
    /// Delegates to [`TimeSeries::exponential_moving_average`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn exponential_moving_average(&self, span: usize) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.exponential_moving_average(span))
    }

    /// Returns a new `Panel` with the MA crossover signal of each column.
    ///
    /// - `+1.0` — fast MA crosses **above** slow MA (bullish)
    /// - `-1.0` — fast MA crosses **below** slow MA (bearish)
    /// - `0.0`  — no crossover
    ///
    /// Delegates to [`TimeSeries::crossover_signal`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn crossover_signal(&self, fast: usize, slow: usize) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.crossover_signal(fast, slow))
    }

    // -----------------------------------------------------------------------
    // VOLATILITY
    // -----------------------------------------------------------------------

    /// Returns a new `Panel` with the n-period rolling standard deviation of each column
    /// (Bessel-corrected).
    ///
    /// $$\sigma_t^{(n)} = \sqrt{\frac{1}{n-1} \sum_{i=0}^{n-1} \left(x_{t-i} - \bar{x}_t^{(n)}\right)^2}$$
    ///
    /// Delegates to [`TimeSeries::rolling_standard_deviation`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn rolling_standard_deviation(&self, n: usize) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.rolling_standard_deviation(n))
    }

    /// Returns a new `Panel` with the per-period true range of each column.
    ///
    /// $$TR_t = \left|x_t - x_{t-1}\right|$$
    ///
    /// Delegates to [`TimeSeries::true_range`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn true_range(&self) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.true_range())
    }

    /// Returns a new `Panel` with the n-period average true range of each column.
    ///
    /// $$ATR_t^{(n)} = \frac{1}{n} \sum_{i=0}^{n-1} TR_{t-i}$$
    ///
    /// Delegates to [`TimeSeries::average_true_range`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn average_true_range(&self, n: usize) -> Result<Panel, TemporalSeriesError> {
        self.panel_map(|ts| ts.average_true_range(n))
    }

    /// Returns Bollinger Bands across all columns as `(upper, middle, lower)` panels.
    ///
    /// $$BB_{upper}(t) = MA_t^{(w)} + k \cdot \sigma_t^{(w)}, \quad BB_{lower}(t) = MA_t^{(w)} - k \cdot \sigma_t^{(w)}$$
    ///
    /// Delegates to [`TimeSeries::bollinger_bands`] for every column. The result
    /// is three `Panel` values sharing the same index and symbols as the source.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn bollinger_bands(
        &self,
        window: usize,
        k: f64,
    ) -> Result<(Panel, Panel, Panel), TemporalSeriesError> {
        let mut upper_vals = Vec::with_capacity(self.n_series());
        let mut mid_vals = Vec::with_capacity(self.n_series());
        let mut lower_vals = Vec::with_capacity(self.n_series());

        for i in 0..self.n_series() {
            let (u, m, l) = self.col_series(i).bollinger_bands(window, k)?;
            upper_vals.push(u.values);
            mid_vals.push(m.values);
            lower_vals.push(l.values);
        }

        let upper = Panel::new(self.index.clone(), self.symbols.clone(), upper_vals)?;
        let mid = Panel::new(self.index.clone(), self.symbols.clone(), mid_vals)?;
        let lower = Panel::new(self.index.clone(), self.symbols.clone(), lower_vals)?;
        Ok((upper, mid, lower))
    }

    // -----------------------------------------------------------------------
    // AUTOCORRELATION
    // -----------------------------------------------------------------------

    /// Returns the autocorrelation function (ACF) at `lag` for each column.
    ///
    /// $$\rho(k) = \frac{\sum_{t=k}^{n-1}(x_t - \bar{x})(x_{t-k} - \bar{x})}{\sum_{t=0}^{n-1}(x_t - \bar{x})^2}$$
    ///
    /// Delegates to [`TimeSeries::autocorrelation_function`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn autocorrelation_function(
        &self,
        lag: usize,
    ) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.autocorrelation_function(lag))
    }

    /// Returns the partial autocorrelation function (PACF) at `lag` for each column
    /// via Levinson-Durbin recursion.
    ///
    /// Delegates to [`TimeSeries::partial_autocorrelation_function`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn partial_autocorrelation_function(
        &self,
        lag: usize,
    ) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.partial_autocorrelation_function(lag))
    }

    // -----------------------------------------------------------------------
    // STATIONARITY
    // -----------------------------------------------------------------------

    /// Returns the Dickey-Fuller test statistic for each column.
    ///
    /// Delegates to [`TimeSeries::stationary_dickey_fuller_statistics`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn stationary_dickey_fuller_statistics(
        &self,
    ) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.stationary_dickey_fuller_statistics())
    }

    /// Tests for stationarity using the Dickey-Fuller test for each column.
    ///
    /// Returns `true` per column when the unit-root null is rejected at `alpha`.
    ///
    /// | `alpha` | Critical value |
    /// |---------|---------------|
    /// | 0.01    | −2.60         |
    /// | 0.05    | −1.95         |
    /// | 0.10    | −1.61         |
    ///
    /// Delegates to [`TimeSeries::stationary_dickey_fuller_test`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn stationary_dickey_fuller_test(
        &self,
        alpha: f32,
    ) -> Result<HashMap<String, bool>, TemporalSeriesError> {
        self.bool_map(|ts| ts.stationary_dickey_fuller_test(alpha))
    }

    // -----------------------------------------------------------------------
    // DISTRIBUTION ANALYSIS
    // -----------------------------------------------------------------------

    /// Returns the Fisher-Pearson skewness of each column.
    ///
    /// $$\text{Skew} = \frac{m_3}{m_2^{3/2}}, \quad m_k = \frac{1}{n}\sum_{i=1}^{n}(x_i - \bar{x})^k$$
    ///
    /// Delegates to [`TimeSeries::skewness`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn skewness(&self) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.skewness())
    }

    /// Returns the excess kurtosis of each column.
    ///
    /// $$\kappa_{excess} = \frac{m_4}{m_2^2} - 3$$
    ///
    /// Delegates to [`TimeSeries::excess_kurtosis`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn excess_kurtosis(&self) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.excess_kurtosis())
    }

    /// Computes the Jarque-Bera test statistic for each column.
    ///
    /// $$JB = n\!\left(\frac{S^2}{6} + \frac{K^2}{24}\right) \sim \chi^2(2)$$
    ///
    /// Delegates to [`TimeSeries::jacque_bera_statistics`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn jacque_bera_statistics(&self) -> Result<HashMap<String, f64>, TemporalSeriesError> {
        self.scalar_map(|ts| ts.jacque_bera_statistics())
    }

    /// Tests for normality using the Jarque-Bera test for each column.
    ///
    /// Returns `true` per column when consistent with normality (null not rejected).
    ///
    /// | `alpha` | χ²(2) critical value |
    /// |---------|---------------------|
    /// | 0.01    | 9.210               |
    /// | 0.05    | 5.991               |
    /// | 0.10    | 4.605               |
    ///
    /// Delegates to [`TimeSeries::jacque_bera_test`] for every column.
    ///
    /// # Errors
    ///
    /// Returns the first [`TemporalSeriesError`] encountered across columns.
    pub fn jacque_bera_test(
        &self,
        alpha: f32,
    ) -> Result<HashMap<String, bool>, TemporalSeriesError> {
        self.bool_map(|ts| ts.jacque_bera_test(alpha))
    }
}
