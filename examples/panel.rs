//! Demonstrates building and querying a [`Panel`].
//!
//! A `Panel` aligns multiple named time series on a single shared index.
//! This example models three trading days of closing prices for two stocks,
//! then extracts one of the series and runs `pct_change` on it.
//!
//! # Layout
//!
//! ```text
//! index:  [ 1,      2,      3     ]
//! AAPL:   [ 150.0,  152.0,  149.0 ]
//! MSFT:   [ 300.0,  305.0,  298.0 ]
//! ```
//!
//! # Run
//!
//! ```bash
//! cargo run --example panel
//! ```

use temporalseries::panel::Panel;

fn main() {
    let index = vec![1_i64, 2, 3];
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string()];
    let values = vec![
        vec![150.0, 152.0, 149.0],
        vec![300.0, 305.0, 298.0],
    ];

    // Construction validates that every series length matches the index.
    let panel = Panel::new(index, symbols, values).unwrap();

    println!("Shape: {:?}", panel.shape()); // (n_timestamps, n_series)
    println!("Symbols: {:?}", panel.symbols());

    // Extract a named series and compute its daily returns.
    let aapl = panel.get_series("AAPL").unwrap();
    let returns = aapl.pct_change().unwrap();

    println!("AAPL prices:  {:?}", aapl.values);
    // The first return is NaN — no prior observation.
    println!("AAPL returns: {:?}", returns.values);

    // Unknown symbols return None.
    println!("GOOG present: {}", panel.get_series("GOOG").is_some());
}