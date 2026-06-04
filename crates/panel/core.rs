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

        Ok(Self { index, symbols, values })
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
}
