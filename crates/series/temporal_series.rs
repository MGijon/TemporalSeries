use std::marker::PhantomData;

use crate::errors::TemporalSeriesError;
use crate::storage::StorageBackend;

/// A generic, backend-agnostic time series.
///
/// `TemporalSeries<T, B>` pairs an ordered sequence of integer timestamps
/// (`index`) with a pluggable storage backend `B` that holds values of type
/// `T`. The backend abstraction lets you choose the in-memory layout that
/// best fits your workload without changing the series API.
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
///
/// let index = vec![1_i64, 2, 3];
/// let backend = ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]);
/// let series = TemporalSeries::new(index, backend).unwrap();
///
/// assert_eq!(series.len(), 3);
/// assert_eq!(series.get(1), Some(&20.0));
/// ```
pub struct TemporalSeries<T, B>
where
    B: StorageBackend<T>,
{
    /// Ordered sequence of integer timestamps, one per observation.
    pub index: Vec<i64>,
    storage: B,
    /// Ties `T` to the struct without storing a `T` directly.
    _marker: PhantomData<T>,
}

impl<T, B: StorageBackend<T>> TemporalSeries<T, B> {
    /// Creates a new `TemporalSeries` pairing an index with a storage backend.
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
            _marker: PhantomData,
        })
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
}
