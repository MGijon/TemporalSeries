//! Demonstrates reading and writing a [`TimeSeries`] via CSV.
//!
//! The CSV format used by this library has a mandatory header row followed by
//! one observation per line:
//!
//! ```text
//! index,value
//! 1,100.0
//! 2,101.5
//! …
//! ```
//!
//! This example exercises the full round-trip:
//!
//! 1. [`read_csv`] — parse `examples/input.csv` into a `TimeSeries`.
//! 2. [`write_csv`] — serialize it back out to `examples/output.csv`.
//! 3. [`read_csv`] again — read the written file and confirm the values match.
//!
//! # Files
//!
//! - **`examples/input.csv`** — source data (10 observations, already present
//!   in the repository).
//! - **`examples/output.csv`** — created by this example at runtime; safe to
//!   delete.
//!
//! # Run
//!
//! ```bash
//! cargo run --example input_output
//! ```

use temporalseries::io::{read_csv, write_csv};
use temporalseries::series::TimeSeries;

fn main() {
    // Paths are relative to the workspace root (where `cargo run` is invoked).
    let time_serie: TimeSeries = read_csv("examples/input.csv").unwrap();
    println!("values: {:?}", time_serie.values);

    // Writes a header row followed by one `index,value` line per observation.
    write_csv(&time_serie, "examples/output.csv").unwrap();

    // Read the file back to verify the round-trip is lossless.
    let read_back: TimeSeries = read_csv("examples/output.csv").unwrap();
    println!("round-trip: {:?}", read_back.values);
}
