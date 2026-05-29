//! Demonstrates building a [`TemporalSeries`] backed by a [`ColumnarBackend`].
//!
//! The columnar layout stores all values in a single contiguous `Vec<T>`,
//! separate from the index. This is the most cache-friendly layout when
//! operations scan the value column alone (e.g. aggregations, rolling windows)
//! because no timestamp bytes are interleaved between values.
//!
//! # Layout
//!
//! ```text
//! index:  [ 1,    2,    3,    4,    5  ]
//! values: [ 10.0, 20.0, 30.0, 25.0, 35.0 ]
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example temporal_series_with_columnar_backend
//! ```

use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

fn main() {
    // The index is a separate Vec of timestamps.
    let index: Vec<i64> = vec![1, 2, 3, 4, 5];
    // Values are stored contiguously — no per-row timestamp overhead.
    let values: Vec<f64> = vec![10.0, 20.0, 30.0, 25.0, 35.0];

    let backend = ColumnarBackend::new(values);
    // Construction validates that index.len() == storage.len().
    let series = TemporalSeries::new(index, backend).unwrap();

    println!("Length: {}", series.len());
    // get() returns Option<&T> — None if the index is out of bounds.
    println!("Value at position 0: {:?}", series.get(0));
    println!("Value at position 2: {:?}", series.get(2));

    // Zip the index and the value iterator to walk both in lock-step.
    print!("All values:");
    for (ts, val) in series.index.iter().zip(series.iter()) {
        print!("  t={ts} -> {val}");
    }
    println!();
}
