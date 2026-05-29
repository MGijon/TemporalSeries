use std::ops::Range;

/// Abstraction over the in-memory layout used to store time-series values.
///
/// A `StorageBackend<T>` holds an ordered sequence of values of type `T` and
/// exposes a uniform read/write interface regardless of how those values are
/// arranged in memory. [`TemporalSeries`](crate::series::TemporalSeries) is
/// generic over this trait, so the storage layout can be swapped without
/// changing the series API.
///
/// # Provided implementations
///
/// | Type | Layout | Best for |
/// |---|---|---|
/// | [`ColumnarBackend<T>`](crate::storage::ColumnarBackend) | Contiguous `Vec<T>` | Column scans, aggregations |
/// | [`RowBackend<T>`](crate::storage::RowBackend) | `Vec<RowRecord<T>>` (timestamp + value per record) | Record-at-a-time ingestion |
///
/// # Implementing a custom backend
///
/// ```rust
/// use std::ops::Range;
/// use temporalseries::storage::StorageBackend;
///
/// struct MyBackend<T>(Vec<T>);
///
/// impl<T: Clone + Send + Sync> StorageBackend<T> for MyBackend<T> {
///     fn len(&self) -> usize { self.0.len() }
///     fn is_empty(&self) -> bool { self.0.is_empty() }
///     fn get(&self, idx: usize) -> Option<&T> { self.0.get(idx) }
///     fn push(&mut self, value: T) { self.0.push(value); }
///     fn slice(&self, range: Range<usize>) -> Self { MyBackend(self.0[range].to_vec()) }
///     fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_> { Box::new(self.0.iter()) }
/// }
/// ```
///
/// # Bounds
///
/// The `Send + Sync` supertraits are required so that a
/// [`TemporalSeries`](crate::series::TemporalSeries) can be safely shared
/// across threads.
pub trait StorageBackend<T>: Send + Sync {
    /// Returns the number of values in the backend.
    fn len(&self) -> usize;

    /// Returns `true` if the backend contains no values.
    fn is_empty(&self) -> bool;

    /// Returns a reference to the value at position `idx`, or `None` if out of bounds.
    fn get(&self, idx: usize) -> Option<&T>;

    /// Appends `value` to the end of the backend.
    fn push(&mut self, value: T);

    /// Returns a new backend containing only the values in `range`.
    ///
    /// The returned backend is independent of `self`; modifying one does not
    /// affect the other.
    fn slice(&self, range: Range<usize>) -> Self;

    /// Returns an iterator over references to each value in order.
    fn iter(&self) -> Box<dyn Iterator<Item = &T> + '_>;
}
