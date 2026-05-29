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

## NaN convention

Operations that cannot produce a value for a position (e.g. the first element
of `diff` or `pct_change`, or the first `window - 1` elements of a rolling mean)
return `NaN` at that position rather than truncating the series. This keeps the
output length equal to the input length and preserves index alignment.

## Development

```bash
# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Open API docs
cargo doc --open

# Run benchmarks
cargo bench
```

## License

MIT
