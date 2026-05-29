//! Demonstrates building a [`TemporalSeries`] backed by a [`RowBackend`].
//!
//! The row layout stores each observation as a [`RowRecord`] — a struct that
//! bundles a timestamp and a value together. This makes it natural to
//! construct a series from data sources that already deliver records one at a
//! time (e.g. a database cursor, a CSV reader, or a message stream), because
//! no separate index bookkeeping is required at ingestion time.
//!
//! # Layout
//!
//! ```text
//! rows: [ {t:1, v:10.0}, {t:2, v:20.0}, {t:3, v:30.0}, … ]
//! ```
//!
//! The index passed to [`TemporalSeries::new`] is derived from the timestamps
//! already embedded in each row, so the two stay in sync by construction.
//!
//! # Trade-off vs columnar
//!
//! Scanning only values requires skipping over the timestamp bytes of each
//! record, which can reduce cache efficiency compared to a [`ColumnarBackend`].
//! In return, the row layout avoids the extra allocation of a parallel index
//! vector and keeps related data co-located.
//!
//! # Run
//!
//! ```bash
//! cargo run --example temporal_series_with_row_backend
//! ```

use temporalseries::series::TemporalSeries;
use temporalseries::storage::{RowBackend, RowRecord};

fn main() {
    // Each record carries its own timestamp — no separate index vector needed
    // at the source.
    let rows: Vec<RowRecord<f64>> = vec![
        RowRecord {
            timestamp: 1,
            value: 10.0,
        },
        RowRecord {
            timestamp: 2,
            value: 20.0,
        },
        RowRecord {
            timestamp: 3,
            value: 30.0,
        },
        RowRecord {
            timestamp: 4,
            value: 25.0,
        },
        RowRecord {
            timestamp: 5,
            value: 35.0,
        },
    ];

    // Derive the index from the embedded timestamps before moving rows into
    // the backend.
    let index: Vec<i64> = rows.iter().map(|r| r.timestamp).collect();
    let backend = RowBackend::new(rows);
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
