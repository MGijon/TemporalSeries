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
