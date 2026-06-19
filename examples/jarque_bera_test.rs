//! Demonstrates the Jarque-Bera normality test.
//!
//! The Jarque-Bera test checks whether a series is consistent with a normal
//! distribution by measuring its skewness (`S`) and excess kurtosis (`K`).
//! It is commonly applied to residuals from regression or ARIMA models.
//!
//! [`TimeSeries::jacque_bera_test`] computes the statistic
//!
//! ```text
//! JB = n · (S² / 6 + K² / 24)
//! ```
//!
//! and compares it against the χ²(2) critical value for the chosen significance
//! level `alpha`. The null hypothesis is normality.
//!
//! This example contrasts two series:
//!
//! - A **near-symmetric** series — the test should return `true` because the
//!   series does not deviate significantly from a normal distribution.
//! - A **heavily skewed** series with a single large outlier — the test should
//!   return `false` because normality is clearly rejected.
//!
//! # Expected output
//!
//! ```text
//! Near-symmetric series → consistent with normality: true
//! Heavily skewed series → consistent with normality: false
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example jarque_bera_test
//! ```

use temporalseries::series::TimeSeries;

fn main() {
    // A roughly symmetric, spread-out series does not violate normality.
    let normal_like = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![-2.0, -1.0, -0.5, 0.0, 0.2, 0.3, 0.5, 1.0, 1.5, 2.0],
    )
    .unwrap();

    let nl_normal = normal_like.jacque_bera_test(0.05).unwrap();
    println!("Near-symmetric series → consistent with normality: {nl_normal}");

    // Nine identical values plus a massive outlier produces extreme skewness.
    let skewed = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 100.0],
    )
    .unwrap();

    let sk_normal = skewed.jacque_bera_test(0.05).unwrap();
    println!("Heavily skewed series → consistent with normality: {sk_normal}");
}
