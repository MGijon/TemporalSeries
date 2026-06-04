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

## License

MIT
