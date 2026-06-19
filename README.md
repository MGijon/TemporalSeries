# TemporalSeries

A Rust library for quantitative time-series analysis.

## Features

- Percentage change (`pct_change`)
- First-order difference (`diff`)
- Lag / forward shift (`shift`)
- Rolling window mean (`rolling().mean()`)

## Usage

```rust
use temporalseries::series::TimeSeries;

let ts = TimeSeries::new(
    vec![1, 2, 3, 4, 5],
    vec![100.0, 101.0, 102.0, 103.0, 104.0],
).unwrap();

// Simple returns
let returns = ts.pct_change().unwrap();

// Rolling mean with window of 3
let momentum = returns.rolling(3).mean().unwrap();
```

## Error handling

All fallible operations return `Result<TimeSeries, TemporalSeriesError>`.

```rust
use temporalseries::{series::TimeSeries, errors::TemporalSeriesError};

match TimeSeries::new(vec![1, 2], vec![1.0]) {
    Err(TemporalSeriesError::LengthMismatch { index_len, values_len }) => {
        eprintln!("index length {index_len} != values length {values_len}");
    }
    _ => {}
}
```

## Storage backends

`TemporalSeries<T, B>` is generic over its storage backend, so you can choose
the in-memory layout that best fits your workload.

### Columnar

Values are stored in a single contiguous `Vec<T>`, separate from the index.
This is the most cache-friendly layout for operations that scan the value
column alone (aggregations, rolling windows).

```rust
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;

let index: Vec<i64> = vec![1, 2, 3, 4, 5];
let values: Vec<f64> = vec![10.0, 20.0, 30.0, 25.0, 35.0];

let backend = ColumnarBackend::new(values);
let series = TemporalSeries::new(index, backend).unwrap();

println!("{:?}", series.get(2)); // Some(30.0)
```

### Row

Each observation is stored as a `RowRecord { timestamp, value }`. This layout
is a natural fit when data arrives record-by-record (database cursors, message
streams) and keeps timestamps co-located with their values.

```rust
use temporalseries::series::TemporalSeries;
use temporalseries::storage::{RowBackend, RowRecord};

let rows: Vec<RowRecord<f64>> = vec![
    RowRecord { timestamp: 1, value: 10.0 },
    RowRecord { timestamp: 2, value: 20.0 },
    RowRecord { timestamp: 3, value: 30.0 },
];

let index: Vec<i64> = rows.iter().map(|r| r.timestamp).collect();
let backend = RowBackend::new(rows);
let series = TemporalSeries::new(index, backend).unwrap();

println!("{:?}", series.get(2)); // Some(30.0)
```

| | Columnar | Row |
|---|---|---|
| Memory layout | values packed, index separate | timestamp + value interleaved per record |
| Best for | column scans, aggregations | record-at-a-time ingestion |
| Extra allocation | separate index `Vec` | none beyond the records themselves |

## Timestamp units (`chrono` feature)

Timestamps are always stored as `i64` integers. The optional `chrono` feature
adds [`TimeUnit`] — an enum that records the unit the integers are expressed in
— and two convenience methods on `TemporalSeries` that convert between `i64`
and [`chrono::DateTime<Utc>`].

Enable the feature in your `Cargo.toml`:

```toml
temporalseries = { version = "0.1", features = ["chrono"] }
```

### Build from `DateTime<Utc>` values

```rust
use chrono::{TimeZone, Utc};
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;
use temporalseries::time::TimeUnit;

let datetimes = vec![
    Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
    Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(),
];

let series = TemporalSeries::from_datetimes(
    datetimes,
    ColumnarBackend::new(vec![150.0_f64, 152.5]),
    TimeUnit::Seconds,
).unwrap();

println!("{:?}", series.index); // [1704067200, 1704153600]
```

### Attach a unit to an existing `i64` index

```rust
use temporalseries::series::TemporalSeries;
use temporalseries::storage::ColumnarBackend;
use temporalseries::time::TimeUnit;

let series = TemporalSeries::new(
    vec![0_i64, 1_000, 2_000],          // milliseconds
    ColumnarBackend::new(vec![1.0_f64, 2.0, 3.0]),
)
.unwrap()
.with_unit(TimeUnit::Milliseconds);

// Convert the index back to DateTime<Utc>
let dts = series.datetimes().unwrap();
println!("{}", dts[1]); // 1970-01-01 00:00:01 UTC
```

### Available units

| Variant | Epoch reference | Typical use |
|---|---|---|
| `TimeUnit::Seconds` | Unix seconds | Databases, REST APIs |
| `TimeUnit::Milliseconds` | Unix milliseconds | JavaScript dates, most financial feeds |
| `TimeUnit::Microseconds` | Unix microseconds | High-frequency trading, system logs |
| `TimeUnit::Nanoseconds` | Unix nanoseconds | Kernel tracing (overflows past ~year 2262) |

### Without the feature

`TimeUnit`, `with_unit`, and `time_unit()` are always available. Only
`from_datetimes` and `datetimes` require `--features chrono`.

## Statistical Tests

### Augmented Dickey-Fuller (stationarity)

`stationary_dickey_fuller_test(alpha)` tests whether a series is stationary — i.e., its statistical properties do not change over time. Stationarity is a prerequisite for many forecasting models.

**How it works:** the test fits an OLS regression of the form

```
Δxₜ = γ · xₜ₋₁ + εₜ
```

and computes the t-statistic `γ̂ / SE(γ̂)`. Under the null hypothesis (unit root, non-stationary), this statistic follows a non-standard Dickey-Fuller distribution. The null is rejected — and stationarity is concluded — when the statistic falls below the critical value for the chosen significance level.

| `alpha` | Critical value |
|---------|---------------|
| `0.01`  | −2.60         |
| `0.05`  | −1.95         |
| `0.10`  | −1.61         |

Returns `true` when the series is stationary (null rejected), `false` otherwise.

```rust
use temporalseries::series::TimeSeries;

// A trending series is NOT stationary
let trend = TimeSeries::new(
    vec![1, 2, 3, 4, 5, 6, 7, 8],
    vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
).unwrap();
assert!(!trend.stationary_dickey_fuller_test(0.05).unwrap());

// An alternating series IS stationary
let alternating = TimeSeries::new(
    vec![1, 2, 3, 4, 5, 6, 7, 8],
    vec![1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
).unwrap();
assert!(alternating.stationary_dickey_fuller_test(0.05).unwrap());
```

### Jarque-Bera (normality)

`jacque_bera_test(alpha)` tests whether a series follows a normal distribution, using its skewness and excess kurtosis. It is commonly applied to residuals from regression or time-series models.

**How it works:** the Jarque-Bera statistic is

```
JB = n · (S² / 6 + K² / 24)
```

where `S` is the Fisher-Pearson skewness and `K` is the excess kurtosis. Under the null hypothesis of normality, `JB` follows a χ²(2) distribution.

| `alpha` | χ²(2) critical value |
|---------|--------------------|
| `0.01`  | 9.210              |
| `0.05`  | 5.991              |
| `0.10`  | 4.605              |

Returns `true` when the series is consistent with normality (null not rejected), `false` when normality is rejected.

```rust
use temporalseries::series::TimeSeries;

// A near-symmetric series is consistent with normality
let normal_like = TimeSeries::new(
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    vec![-2.0, -1.0, -0.5, 0.0, 0.2, 0.3, 0.5, 1.0, 1.5, 2.0],
).unwrap();
assert!(normal_like.jacque_bera_test(0.05).unwrap());

// A heavily skewed series is NOT consistent with normality
let skewed = TimeSeries::new(
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 100.0],
).unwrap();
assert!(!skewed.jacque_bera_test(0.05).unwrap());
```

## NaN convention

Operations that cannot produce a value for a position (e.g. the first element
of `diff` or `pct_change`, or the first `window - 1` elements of a rolling mean)
return `NaN` at that position rather than truncating the series. This keeps the
output length equal to the input length and preserves index alignment.

## Examples

| Example | Command | Description |
|---|---|---|
| `basic` | `cargo run --example basic` | Constructs a series and computes `pct_change` followed by a rolling mean |
| `input_output` | `cargo run --example input_output` | Reads a series from `examples/input.csv`, writes it to `examples/output.csv`, and reads it back |
| `temporal_series_with_columnar_backend` | `cargo run --example temporal_series_with_columnar_backend` | Builds a `TemporalSeries` with a `ColumnarBackend` and demonstrates access and iteration |
| `temporal_series_with_row_backend` | `cargo run --example temporal_series_with_row_backend` | Builds a `TemporalSeries` with a `RowBackend` and demonstrates access and iteration |
| `panel` | `cargo run --example panel` | Builds a `Panel` of named series on a shared index and extracts one series for analysis |
| `temporal_series_with_chrono` | `cargo run --example temporal_series_with_chrono --features chrono` | Builds a `TemporalSeries` from `DateTime<Utc>` values and round-trips the index back to calendar dates |
| `dickey_fuller_test` | `cargo run --example dickey_fuller_test` | Contrasts a trending vs. alternating series with the Dickey-Fuller test, shown for both `TimeSeries` and `TemporalSeries` |
| `jarque_bera_test` | `cargo run --example jarque_bera_test` | Contrasts a near-symmetric vs. heavily skewed series with the Jarque-Bera test, shown for both `TimeSeries` and `TemporalSeries` |

## Development

```bash
# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Open API docs
cargo doc --open

# Run all benchmarks
cargo bench

# Run a single benchmark target
cargo bench --bench time_series
cargo bench --bench columnar_backend
cargo bench --bench row_backend

# Open the HTML report in the browser (macOS)
open target/criterion/report/index.html
```

Criterion writes an HTML report to `target/criterion/` after every run.
Each benchmark target has its own sub-report, and there is a combined index at
`target/criterion/report/index.html` that covers all targets.
To open a single target's report directly:

```bash
open target/criterion/columnar_backend/report/index.html
open target/criterion/row_backend/report/index.html
open target/criterion/time_series/report/index.html
```

### Coverage

Prerequisites (one-time setup):

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```

```bash
# Print a coverage summary to the terminal
cargo llvm-cov

# Generate an HTML report (all features)
cargo llvm-cov --html --features chrono

# Open the report in the browser (macOS)
open target/llvm-cov/html/index.html
```

The HTML report is also generated automatically in CI on every push and pull
request to `main`. It is uploaded as the `coverage-report` artifact and kept
for **15 days**. Download it from the **Actions** tab of the repository, open
the run, and grab the artifact from the summary page.

## License

MIT
