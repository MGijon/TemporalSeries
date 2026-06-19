//! Demonstrates the Augmented Dickey-Fuller stationarity test.
//!
//! A time series is *stationary* when its statistical properties (mean,
//! variance, autocorrelation) do not change over time. Many forecasting
//! and econometric models require stationarity as a precondition.
//!
//! [`TimeSeries::stationary_dickey_fuller_test`] fits an OLS regression of
//! the form
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
//! This example contrasts two series:
//!
//! - A **linear trend** (always growing) — the test should return `false`
//!   because the mean is not constant.
//! - An **alternating** series (mean-reverting) — the test should return `true`
//!   because the series strongly reverts to its mean.
//!
//! # Expected output
//!
//! ```text
//! Trending series   → stationary: false
//! Alternating series → stationary: true
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example dickey_fuller_test
//! ```

use temporalseries::series::TimeSeries;

fn main() {
    // A linearly increasing series has a unit root — it is NOT stationary.
    let trend = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    )
    .unwrap();

    let trend_stationary = trend.stationary_dickey_fuller_test(0.05).unwrap();
    println!("Trending series   → stationary: {trend_stationary}");

    // An alternating series mean-reverts strongly — it IS stationary.
    let alternating = TimeSeries::new(
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
    )
    .unwrap();

    let alt_stationary = alternating.stationary_dickey_fuller_test(0.05).unwrap();
    println!("Alternating series → stationary: {alt_stationary}");
}
