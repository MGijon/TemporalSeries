//! Demonstrates the Jarque-Bera normality test on both [`TimeSeries`] and
//! [`TemporalSeries`].
//!
//! The Jarque-Bera test checks whether a series is consistent with a normal
//! distribution by measuring its skewness (`S`) and excess kurtosis (`K`).
//! It is commonly applied to residuals from regression or ARIMA models.
//!
//! Both types expose `jacque_bera_test`, which computes the statistic
//!
//! ```text
//! JB = n · (S² / 6 + K² / 24)
//! ```
//!
//! and compares it against the χ²(2) critical value for the chosen significance
//! level `alpha`. The null hypothesis is normality.
//!
//! Each section contrasts two series:
//!
//! - A **near-symmetric** series — the test should return `true` because the
//!   series does not deviate significantly from a normal distribution.
//! - A **heavily skewed** series with a single large outlier — the test should
//!   return `false` because normality is clearly rejected.
//!
//! # Expected output
//!
//! ```text
//! --- TimeSeries ---
//! Near-symmetric series → consistent with normality: true
//! Heavily skewed series → consistent with normality: false
//!
//! --- TemporalSeries (ColumnarBackend) ---
//! Near-symmetric series → consistent with normality: true
//! Heavily skewed series → consistent with normality: false
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example jarque_bera_test
//! ```

use temporalseries::series::{TemporalSeries, TimeSeries};
use temporalseries::storage::ColumnarBackend;

fn main() {
    // -----------------------------------------------------------------------
    // TimeSeries
    // -----------------------------------------------------------------------
    println!("--- TimeSeries ---");

    let normal_ts = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![-2.0, -1.0, -0.5, 0.0, 0.2, 0.3, 0.5, 1.0, 1.5, 2.0],
    )
    .unwrap();
    println!(
        "Near-symmetric series → consistent with normality: {}",
        normal_ts.jacque_bera_test(0.05).unwrap()
    );

    let skewed_ts = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 100.0],
    )
    .unwrap();
    println!(
        "Heavily skewed series → consistent with normality: {}",
        skewed_ts.jacque_bera_test(0.05).unwrap()
    );

    // -----------------------------------------------------------------------
    // TemporalSeries (ColumnarBackend)
    // -----------------------------------------------------------------------
    println!("\n--- TemporalSeries (ColumnarBackend) ---");

    let normal_col = TemporalSeries::new(
        vec![1_i64, 2, 3, 4, 5],
        ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0, 4.0, 5.0]),
    )
    .unwrap();
    println!(
        "Near-symmetric series → consistent with normality: {}",
        normal_col.jacque_bera_test(0.05).unwrap()
    );

    let skewed_col = TemporalSeries::new(
        vec![1_i64, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        ColumnarBackend::new(vec![1.0_f64, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 100.0]),
    )
    .unwrap();
    println!(
        "Heavily skewed series → consistent with normality: {}",
        skewed_col.jacque_bera_test(0.05).unwrap()
    );
}
