//! Demonstrates building, querying, and analysing a [`Panel`].
//!
//! A `Panel` aligns multiple named time series on a single shared index.
//! Every analytical method available on [`TimeSeries`] is exposed on `Panel`
//! and applied column-by-column. Scalar results are returned as
//! `HashMap<String, f64>` (or `bool`), series results as a new `Panel`.
//!
//! # Layout
//!
//! ```text
//! index:  [ 1,      2,      3,      4,      5,      6,      7,      8,      9,      10    ]
//! AAPL:   [ 150.0,  152.0,  149.0,  153.0,  155.0,  154.0,  156.0,  158.0,  157.0,  160.0 ]
//! MSFT:   [ 300.0,  305.0,  298.0,  310.0,  308.0,  312.0,  315.0,  313.0,  318.0,  320.0 ]
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example panel
//! ```

use temporalseries::panel::Panel;

fn main() {
    let index: Vec<i64> = (1..=10).collect();
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string()];
    let values = vec![
        vec![
            150.0, 152.0, 149.0, 153.0, 155.0, 154.0, 156.0, 158.0, 157.0, 160.0,
        ],
        vec![
            300.0, 305.0, 298.0, 310.0, 308.0, 312.0, 315.0, 313.0, 318.0, 320.0,
        ],
    ];

    let panel = Panel::new(index, symbols, values).unwrap();

    println!("Shape:   {:?}", panel.shape());
    println!("Symbols: {:?}", panel.symbols());

    // -----------------------------------------------------------------------
    // Basic access
    // -----------------------------------------------------------------------

    let aapl = panel.get_series("AAPL").unwrap();
    println!("\nAAPL prices: {:?}", aapl.values);

    // Unknown symbols return None.
    println!("GOOG present: {}", panel.get_series("GOOG").is_some());

    // -----------------------------------------------------------------------
    // Statistics
    // -----------------------------------------------------------------------

    let means = panel.mean();
    println!("\n--- mean ---");
    println!("AAPL: {:.4}", means["AAPL"]);
    println!("MSFT: {:.4}", means["MSFT"]);

    let stds = panel.std_deviation();
    println!("\n--- std_deviation ---");
    println!("AAPL: {:.4}", stds["AAPL"]);
    println!("MSFT: {:.4}", stds["MSFT"]);

    let medians = panel.quantile(0.5).unwrap();
    println!("\n--- quantile(0.5) ---");
    println!("AAPL: {:.4}", medians["AAPL"]);
    println!("MSFT: {:.4}", medians["MSFT"]);

    let iqrs = panel.iqr().unwrap();
    println!("\n--- iqr ---");
    println!("AAPL: {:.4}", iqrs["AAPL"]);
    println!("MSFT: {:.4}", iqrs["MSFT"]);

    // -----------------------------------------------------------------------
    // Returns
    // -----------------------------------------------------------------------

    let simple = panel.simple_return().unwrap();
    println!("\n--- simple_return ---");
    println!("AAPL: {:?}", simple.get_series("AAPL").unwrap().values);

    let log_ret = panel.log_return().unwrap();
    println!("\n--- log_return ---");
    println!("AAPL: {:?}", log_ret.get_series("AAPL").unwrap().values);

    let cum_ret = panel.cumulative_return().unwrap();
    println!("\n--- cumulative_return ---");
    println!("AAPL: {:.4}", cum_ret["AAPL"]);
    println!("MSFT: {:.4}", cum_ret["MSFT"]);

    let diff = panel.diff().unwrap();
    println!("\n--- diff ---");
    println!("AAPL: {:?}", diff.get_series("AAPL").unwrap().values);

    let pct = panel.pct_change().unwrap();
    println!("\n--- pct_change ---");
    println!("AAPL: {:?}", pct.get_series("AAPL").unwrap().values);

    let shifted = panel.shift(1).unwrap();
    println!("\n--- shift(1) ---");
    println!("AAPL: {:?}", shifted.get_series("AAPL").unwrap().values);

    // -----------------------------------------------------------------------
    // Moving averages
    // -----------------------------------------------------------------------

    let ma = panel.moving_average(3).unwrap();
    println!("\n--- moving_average(3) ---");
    println!("AAPL: {:?}", ma.get_series("AAPL").unwrap().values);

    let ema = panel.exponential_moving_average(3).unwrap();
    println!("\n--- exponential_moving_average(3) ---");
    println!("AAPL: {:?}", ema.get_series("AAPL").unwrap().values);

    let signals = panel.crossover_signal(2, 5).unwrap();
    println!("\n--- crossover_signal(fast=2, slow=5) ---");
    println!("AAPL: {:?}", signals.get_series("AAPL").unwrap().values);

    // -----------------------------------------------------------------------
    // Volatility
    // -----------------------------------------------------------------------

    let rolling_std = panel.rolling_standard_deviation(3).unwrap();
    println!("\n--- rolling_standard_deviation(3) ---");
    println!("AAPL: {:?}", rolling_std.get_series("AAPL").unwrap().values);

    let tr = panel.true_range().unwrap();
    println!("\n--- true_range ---");
    println!("AAPL: {:?}", tr.get_series("AAPL").unwrap().values);

    let atr = panel.average_true_range(3).unwrap();
    println!("\n--- average_true_range(3) ---");
    println!("AAPL: {:?}", atr.get_series("AAPL").unwrap().values);

    let (bb_upper, bb_mid, bb_lower) = panel.bollinger_bands(3, 2.0).unwrap();
    println!("\n--- bollinger_bands(window=3, k=2.0) ---");
    println!(
        "AAPL upper: {:?}",
        bb_upper.get_series("AAPL").unwrap().values
    );
    println!(
        "AAPL mid:   {:?}",
        bb_mid.get_series("AAPL").unwrap().values
    );
    println!(
        "AAPL lower: {:?}",
        bb_lower.get_series("AAPL").unwrap().values
    );

    // -----------------------------------------------------------------------
    // Autocorrelation
    // -----------------------------------------------------------------------

    let acf = panel.autocorrelation_function(1).unwrap();
    println!("\n--- autocorrelation_function(lag=1) ---");
    println!("AAPL: {:.4}", acf["AAPL"]);
    println!("MSFT: {:.4}", acf["MSFT"]);

    let pacf = panel.partial_autocorrelation_function(2).unwrap();
    println!("\n--- partial_autocorrelation_function(lag=2) ---");
    println!("AAPL: {:.4}", pacf["AAPL"]);
    println!("MSFT: {:.4}", pacf["MSFT"]);

    // -----------------------------------------------------------------------
    // Stationarity (Dickey-Fuller)
    // -----------------------------------------------------------------------

    let df_stat = panel.stationary_dickey_fuller_statistics().unwrap();
    println!("\n--- stationary_dickey_fuller_statistics ---");
    println!("AAPL stat: {:.4}", df_stat["AAPL"]);
    println!("MSFT stat: {:.4}", df_stat["MSFT"]);

    let df_test = panel.stationary_dickey_fuller_test(0.05).unwrap();
    println!("\n--- stationary_dickey_fuller_test(alpha=0.05) ---");
    println!("AAPL stationary: {}", df_test["AAPL"]);
    println!("MSFT stationary: {}", df_test["MSFT"]);

    // -----------------------------------------------------------------------
    // Distribution analysis (Jarque-Bera)
    // -----------------------------------------------------------------------

    let skew = panel.skewness().unwrap();
    println!("\n--- skewness ---");
    println!("AAPL: {:.4}", skew["AAPL"]);
    println!("MSFT: {:.4}", skew["MSFT"]);

    let kurt = panel.excess_kurtosis().unwrap();
    println!("\n--- excess_kurtosis ---");
    println!("AAPL: {:.4}", kurt["AAPL"]);
    println!("MSFT: {:.4}", kurt["MSFT"]);

    let jb_stat = panel.jacque_bera_statistics().unwrap();
    println!("\n--- jacque_bera_statistics ---");
    println!("AAPL JB stat: {:.4}", jb_stat["AAPL"]);
    println!("MSFT JB stat: {:.4}", jb_stat["MSFT"]);

    let jb_test = panel.jacque_bera_test(0.05).unwrap();
    println!("\n--- jacque_bera_test(alpha=0.05) ---");
    println!("AAPL normal: {}", jb_test["AAPL"]);
    println!("MSFT normal: {}", jb_test["MSFT"]);
}
