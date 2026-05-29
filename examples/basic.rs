//! Demonstrates the core analytical operations of [`TimeSeries`].
//!
//! Starting from a short price series, this example chains three operations
//! that are common in quantitative finance:
//!
//! 1. [`TimeSeries::pct_change`] — converts prices into period returns.
//! 2. [`TimeSeries::rolling`] + [`RollingSeries::mean`] — smooths the returns
//!    with a rolling mean, producing a simple momentum signal.
//!
//! # NaN convention
//!
//! Both operations leave positions where no result can be computed set to
//! `NaN` rather than truncating the series. `pct_change` sets index 0 to `NaN`
//! (no prior observation); `rolling(2).mean()` sets index 0 to `NaN` as well
//! (window not yet full). The output length always equals the input length.
//!
//! # Expected output
//!
//! ```text
//! [NaN, NaN, 0.0099..., 0.0099..., 0.0099...]
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example basic
//! ```

use temporalseries::series::TimeSeries;

fn main() {
    let index: Vec<i64> = vec![1, 2, 3, 4, 5];
    // Uniformly spaced prices — each step is +1.0, i.e. ~0.99 % return.
    let values: Vec<f64> = vec![100.0, 101.0, 102.0, 103.0, 104.0];

    let time_serie: TimeSeries = TimeSeries::new(index, values).unwrap();

    // Each element becomes value[t] / value[t-1] - 1; index 0 is NaN.
    let returns: TimeSeries = time_serie.pct_change().unwrap();
    // Rolling mean of width 2; index 0 is NaN (window not yet full).
    let momentum: TimeSeries = returns.rolling(2).mean().unwrap();

    println!("{:?}", momentum.values);
}
