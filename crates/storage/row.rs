use std::ops::Range;

use super::StorageBackend;

/// A single time-series observation in row layout.
///
/// `RowRecord<T>` bundles a timestamp and its associated value into one
/// struct. This is the element type of [`RowBackend<T>`], and it is also
/// the natural unit of data for sources that deliver observations one at a
/// time — database cursors, message streams, CSV readers — because each row
/// is self-contained and requires no parallel index bookkeeping.
///
/// # Fields
///
/// - `timestamp` — the integer timestamp for this observation (e.g. a Unix
///   epoch in seconds, a trading-day offset, or a simple counter).
/// - `value` — the observed value at `timestamp`.
///
/// # Example
///
/// ```rust
/// use temporalseries::storage::RowRecord;
///
/// let record: RowRecord<f64> = RowRecord { timestamp: 42, value: 3.14 };
///
/// assert_eq!(record.timestamp, 42);
/// assert_eq!(record.value, 3.14);
/// ```
pub struct RowRecord<T> {
    /// Integer timestamp for this observation.
    pub timestamp: i64,
    /// Observed value at `timestamp`.
    pub value: T,
}

/// A [`StorageBackend`] that stores each observation as a [`RowRecord`].
///
/// `RowBackend<T>` holds a `Vec<RowRecord<T>>`, so every element carries its
/// own timestamp alongside its value. This interleaved layout is well-suited
/// for record-at-a-time ingestion: you build the backend directly from
/// whatever records your source produces, and the timestamps are preserved
/// without a separate index allocation.
///
/// For workloads that scan only values (aggregations, rolling windows),
/// [`ColumnarBackend<T>`](crate::storage::ColumnarBackend) is more
/// cache-friendly because there are no timestamp bytes interleaved between
/// values.
///
/// # Example
///
/// ```rust
/// use temporalseries::storage::{RowBackend, RowRecord, StorageBackend};
///
/// let rows = vec![
///     RowRecord { timestamp: 1, value: 10.0_f64 },
///     RowRecord { timestamp: 2, value: 20.0 },
///     RowRecord { timestamp: 3, value: 30.0 },
/// ];
///
/// let backend = RowBackend::new(rows);
///
/// assert_eq!(backend.len(), 3);
/// assert_eq!(backend.get(1), Some(&20.0));
/// ```
pub struct RowBackend<T> {
    rows: Vec<RowRecord<T>>,
}

impl<T> RowBackend<T> {
    /// Creates a new `RowBackend` from a vector of [`RowRecord`]s.
    ///
    /// The records are stored in the order they are provided; no sorting or
    /// deduplication is performed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use temporalseries::storage::{RowBackend, RowRecord, StorageBackend};
    ///
    /// let backend = RowBackend::new(vec![
    ///     RowRecord { timestamp: 1, value: 1.0_f64 },
    ///     RowRecord { timestamp: 2, value: 2.0 },
    /// ]);
    ///
    /// assert_eq!(backend.len(), 2);
    /// ```
    pub fn new(rows: Vec<RowRecord<T>>) -> Self {
        Self { rows }
    }
}

impl<T: Clone + Send + Sync> StorageBackend<T> for RowBackend<T> {
    /// Returns the number of records in the backend.
    fn len(&self) -> usize {
        self.rows.len()
    }

    /// Returns `true` if the backend contains no records.
    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns a reference to the value of the record at position `idx`,
    /// or `None` if out of bounds.
    fn get(&self, idx: usize) -> Option<&T> {
        self.rows.get(idx).map(|r| &r.value)
    }

    /// Appends a new record with `value` and an auto-assigned timestamp equal
    /// to the current length before insertion (0-based sequential).
    fn push(&mut self, value: T) {
        let timestamp = self.rows.len() as i64;
        self.rows.push(RowRecord { timestamp, value });
    }

    /// Returns a new `RowBackend` containing the records in `range`.
    ///
    /// Timestamps are preserved from the original records.
    fn slice(&self, range: Range<usize>) -> Self {
        Self {
            rows: self.rows[range]
                .iter()
                .map(|r| RowRecord {
                    timestamp: r.timestamp,
                    value: r.value.clone(),
                })
                .collect(),
        }
    }

    /// Returns an iterator over references to each record's value in order.
    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        Box::new(self.rows.iter().map(|r| &r.value))
    }
}
