use std::ops::Range;

use super::StorageBackend;

/// A [`StorageBackend`] that stores values in a single contiguous `Vec<T>`.
///
/// `ColumnarBackend<T>` keeps all values packed together in memory, separate
/// from the index held by [`TemporalSeries`](crate::series::TemporalSeries).
/// This layout is maximally cache-friendly when an operation iterates over
/// values alone — aggregations, rolling windows, `pct_change`, `diff` — because
/// no timestamp bytes are interleaved between consecutive values.
///
/// When timestamps must travel with their values (e.g. for record-at-a-time
/// ingestion or serialisation), prefer
/// [`RowBackend<T>`](crate::storage::RowBackend).
///
/// # Example
///
/// ```rust
/// use temporalseries::storage::{ColumnarBackend, StorageBackend};
///
/// let backend = ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]);
///
/// assert_eq!(backend.len(), 3);
/// assert_eq!(backend.get(1), Some(&20.0));
/// ```
pub struct ColumnarBackend<T> {
    data: Vec<T>,
}

impl<T> ColumnarBackend<T> {
    /// Creates a new `ColumnarBackend` from a vector of values.
    ///
    /// The values are stored in the order they are provided.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::storage::{ColumnarBackend, StorageBackend};
    ///
    /// let backend = ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]);
    ///
    /// assert_eq!(backend.len(), 3);
    /// ```
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Clone + Send + Sync> StorageBackend<T> for ColumnarBackend<T> {
    /// Returns the number of values in the backend.
    fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the backend contains no values.
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a reference to the value at position `idx`,
    /// or `None` if out of bounds.
    fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    /// Appends `value` to the end of the backend.
    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Returns a new `ColumnarBackend` containing the values in `range`.
    fn slice(&self, range: Range<usize>) -> Self {
        Self {
            data: self.data[range].to_vec(),
        }
    }

    /// Returns an iterator over references to each value in order.
    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.data.iter())
    }
}
