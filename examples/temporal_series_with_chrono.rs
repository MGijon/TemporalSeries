//! Demonstrates timestamp-aware [`TemporalSeries`] using the `chrono` feature.
//!
//! Timestamps are always stored as `i64` integers. The `chrono` feature adds
//! two things on top of that bare integer index:
//!
//! 1. **[`TimeUnit`]** — records the unit (`Seconds`, `Milliseconds`, …) so
//!    the integers are no longer ambiguous.
//! 2. **[`TemporalSeries::from_datetimes`]** / **[`TemporalSeries::datetimes`]**
//!    — constructors and converters that accept and return
//!    [`chrono::DateTime<Utc>`] values, converting to/from `i64` automatically.
//!
//! # What this example shows
//!
//! - Build a series directly from a [`Vec<DateTime<Utc>>`] via
//!   [`TemporalSeries::from_datetimes`].
//! - Attach a [`TimeUnit`] to an existing series with [`TemporalSeries::with_unit`].
//! - Round-trip the index back to calendar dates with [`TemporalSeries::datetimes`].
//!
//! # Run
//!
//! ```bash
//! cargo run --example temporal_series_with_chrono --features chrono
//! ```

use chrono::{TimeZone, Utc};
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;
use temporalseries::time::TimeUnit;

fn main() {
    // --- Build from DateTime<Utc> values -----------------------------------

    // Three daily closing prices at midnight UTC on consecutive days.
    let datetimes = vec![
        Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap(),
    ];

    let backend = ColumnarBackend::new(vec![150.0_f64, 152.5, 149.0]);

    // from_datetimes converts each DateTime to i64 seconds and stores the unit.
    let series =
        TemporalSeries::from_datetimes(datetimes.clone(), backend, TimeUnit::Seconds).unwrap();

    println!("Index (Unix seconds): {:?}", series.index);
    println!("Unit: {:?}", series.time_unit());

    // --- Round-trip back to DateTime<Utc> ----------------------------------

    // datetimes() converts the i64 index back using the stored TimeUnit.
    let recovered = series.datetimes().unwrap();
    for (original, recovered) in datetimes.iter().zip(&recovered) {
        assert_eq!(original, recovered);
        println!("  {}", recovered.format("%Y-%m-%d %H:%M UTC"));
    }

    // --- Attach a unit to a manually-built series --------------------------

    // When you already have an i64 index (e.g. from a CSV or database), use
    // with_unit() to label it without changing the stored values.
    let ms_index = vec![0_i64, 1_000, 2_000]; // 0 s, 1 s, 2 s expressed in ms
    let backend2 = ColumnarBackend::new(vec![10.0_f64, 20.0, 30.0]);

    let series2 = TemporalSeries::new(ms_index, backend2)
        .unwrap()
        .with_unit(TimeUnit::Milliseconds);

    let dts = series2.datetimes().unwrap();
    println!("\nMillisecond index round-tripped to DateTimes:");
    for dt in &dts {
        println!("  {} ms -> {}", dt.timestamp_millis(), dt);
    }
}
