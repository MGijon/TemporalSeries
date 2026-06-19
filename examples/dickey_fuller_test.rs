//! Demonstrates the Augmented Dickey-Fuller stationarity test on both
//! [`TimeSeries`] and [`TemporalSeries`].
//!
//! A time series is *stationary* when its statistical properties (mean,
//! variance, autocorrelation) do not change over time. Many forecasting
//! and econometric models require stationarity as a precondition.
//!
//! Both types expose `stationary_dickey_fuller_test`, which fits an OLS
//! regression of the form
//!
//! ```text
//! Δxₜ = γ · xₜ₋₁ + εₜ
//! ```
//!
//! and computes the t-statistic `γ̂ / SE(γ̂)`. The null hypothesis is the
//! presence of a unit root (non-stationary). The null is rejected — and
//! stationarity is concluded — when the statistic falls below the critical
//! value for the chosen significance level `alpha`.
//!
//! Each section contrasts two series:
//!
//! - A **linear trend** (always growing) — the test should return `false`
//!   because the mean is not constant.
//! - An **alternating** series (mean-reverting) — the test should return `true`
//!   because the series strongly reverts to its mean.
//!
//! # Expected output
//!
//! ```text
//! --- TimeSeries ---
//! Trending series    → stationary: false
//! Alternating series → stationary: true
//!
//! --- TemporalSeries (ColumnarBackend) ---
//! Trending series    → stationary: false
//! Alternating series → stationary: true
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example dickey_fuller_test
//! ```

use temporalseries::series::{TemporalSeries, TimeSeries};
use temporalseries::storage::ColumnarBackend;

fn main() {
    // -----------------------------------------------------------------------
    // TimeSeries
    // -----------------------------------------------------------------------
    println!("--- TimeSeries ---");

    let trend_ts = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    )
    .unwrap();
    println!(
        "Trending series    → stationary: {}",
        trend_ts.stationary_dickey_fuller_test(0.05).unwrap()
    );

    let alternating_ts = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
    )
    .unwrap();
    println!(
        "Alternating series → stationary: {}",
        alternating_ts.stationary_dickey_fuller_test(0.05).unwrap()
    );

    // -----------------------------------------------------------------------
    // TemporalSeries (ColumnarBackend)
    // -----------------------------------------------------------------------
    println!("\n--- TemporalSeries (ColumnarBackend) ---");

    let trend_col = TemporalSeries::new(
        vec![1_i64, 2, 3, 4, 5, 6, 7, 8],
        ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
    )
    .unwrap();
    println!(
        "Trending series    → stationary: {}",
        trend_col.stationary_dickey_fuller_test(0.05).unwrap()
    );

    let alternating_col = TemporalSeries::new(
        vec![1_i64, 2, 3, 4, 5, 6, 7, 8],
        ColumnarBackend::new(vec![1.0_f64, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0]),
    )
    .unwrap();
    println!(
        "Alternating series → stationary: {}",
        alternating_col.stationary_dickey_fuller_test(0.05).unwrap()
    );
}
