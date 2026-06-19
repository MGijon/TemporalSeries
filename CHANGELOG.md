# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on Keep a Changelog and Semantic Versioning.

## [0.1.2] - 2026-06-19

### Added

#### `TemporalSeries` analytical methods

- Full analytical impl block on `TemporalSeries<f64, B>` (for any `StorageBackend<f64>`),
  matching every method on `TimeSeries`:
  `mean`, `std_deviation`, `quantile`, `iqr`,
  `simple_return`, `log_return`, `cumulative_return`,
  `moving_average`, `exponential_moving_average`, `crossover_signal`,
  `rolling_standard_deviation`, `true_range`, `average_true_range`, `bollinger_bands`,
  `autocorrelation_function`, `partial_autocorrelation_function`,
  `stationary_dickey_fuller_statistics`, `stationary_dickey_fuller_test`,
  `skewness`, `excess_kurtosis`, `jacque_bera_statistics`, `jacque_bera_test`.
- `ColSeries` type alias (`TemporalSeries<f64, ColumnarBackend<f64>>`) — the concrete
  return type for series-returning methods on `TemporalSeries`.
- `# Examples` doc sections added to `stationary_dickey_fuller_test` and
  `jacque_bera_test` on `TemporalSeries`.

#### `Panel` analytical methods

- All analytical methods implemented on `Panel`, delegating column-by-column to
  `TimeSeries`. Scalar results returned as `HashMap<String, f64>` (or `bool`);
  series results returned as a new `Panel`:
  `mean`, `std_deviation`, `quantile`, `iqr`,
  `simple_return`, `log_return`, `cumulative_return`, `diff`, `pct_change`, `shift`,
  `moving_average`, `exponential_moving_average`, `crossover_signal`,
  `rolling_standard_deviation`, `true_range`, `average_true_range`,
  `bollinger_bands` (returns `(Panel, Panel, Panel)`),
  `autocorrelation_function`, `partial_autocorrelation_function`,
  `stationary_dickey_fuller_statistics`, `stationary_dickey_fuller_test`,
  `skewness`, `excess_kurtosis`, `jacque_bera_statistics`, `jacque_bera_test`.
- Private helpers `col_series`, `scalar_map`, `bool_map`, `panel_map` on `Panel`
  to eliminate boilerplate across all method implementations.

#### Test suite

- `tests/series/temporal_series/` — 20 unit test files covering every analytical
  method on `TemporalSeries` with `ColumnarBackend`.
- `tests/panel/test_panel_methods.rs` — 25 unit tests verifying that every
  `Panel` method produces results identical to calling the corresponding
  `TimeSeries` method directly on each column.
- `assert_f64_vecs_eq` helper in the panel test module — element-wise `Vec<f64>`
  comparison that treats `NaN` as equal (needed for methods that use `NaN` as
  a no-value sentinel at leading positions).

#### Examples

- `dickey_fuller_test` — expanded to show the Dickey-Fuller test on both
  `TimeSeries` and `TemporalSeries` (with `ColumnarBackend`).
- `jarque_bera_test` — expanded to show the Jarque-Bera test on both
  `TimeSeries` and `TemporalSeries` (with `ColumnarBackend`).
- `panel` — expanded to demonstrate all 25 analytical methods grouped by
  category (statistics, returns, moving averages, volatility, autocorrelation,
  stationarity, distribution analysis).

#### Documentation

- `README.md` Features section rewritten to enumerate all methods organised
  by category across all three types.
- `README.md` Statistical Tests section added, documenting the Dickey-Fuller
  and Jarque-Bera tests with formulas, critical-value tables, and examples.
- Examples table updated with expanded descriptions for `panel`,
  `dickey_fuller_test`, and `jarque_bera_test`.

### Changed

- Test directory restructured to mirror crate layout:
  `tests/series/time_series/` for `TimeSeries` tests and
  `tests/series/temporal_series/` for `TemporalSeries` tests,
  both nested under `tests/series/`.

### Fixed

- Five `cargo clippy` warnings in `crates/series/time_series.rs`:
  - `needless_return` in `std_deviation` — converted `if/else` to expression form.
  - `manual_range_contains` in `quantile` — `p < 0.0 || p > 1.0` replaced with
    `!(0.0..=1.0).contains(&p)`.
  - `needless_range_loop` in `crossover_signal` — index loop replaced with
    `signals.iter_mut().enumerate().skip(1)`.
  - `needless_range_loop` in `rolling_standard_deviation` — index loop replaced
    with `result.iter_mut().enumerate().skip(n - 1)`.

---

## [0.1.1] - 2026-06-05

### Added

#### Timestamp units (`chrono` feature)

- `crates/time/` module with a `TimeUnit` enum (`Seconds`, `Milliseconds`,
  `Microseconds`, `Nanoseconds`) that records the unit of the `i64` index.
- `TemporalSeries::with_unit` — consuming builder to attach a `TimeUnit` after
  construction; always available.
- `TemporalSeries::time_unit` — accessor that returns `Option<&TimeUnit>`;
  always available.
- `TemporalSeries::from_datetimes` — construct a series directly from
  `Vec<chrono::DateTime<Utc>>`, converting to `i64` automatically; requires
  `--features chrono`.
- `TemporalSeries::datetimes` — convert the stored `i64` index back to
  `Vec<DateTime<Utc>>`; requires `--features chrono`.
- `temporal_series_with_chrono` example — demonstrates `from_datetimes`,
  `datetimes`, and `with_unit`; run with
  `cargo run --example temporal_series_with_chrono --features chrono`.
- `chrono` is an optional dependency behind the `chrono` feature flag; not
  compiled into the library unless explicitly enabled.

#### Test suite

- Error tests — one file per `TemporalSeriesError` variant
  (`test_length_mismatch`, `test_empty_series`, `test_invalid_window`,
  `test_io_error`, `test_parse_error`), each covering `Display`, `Debug`,
  and `source()`.
- Storage tests — `test_columnar` and `test_row` covering construction,
  `get`, `push`, `slice`, `iter`, and `is_empty` for both backends.
- Panel tests — construction, length invariant validation, `get_series`,
  `symbols`, `shape`, and `is_empty`.
- Rolling tests — `rolling().mean()` correctness, output length, index
  preservation, and `NaN` fill for the warm-up window.
- I/O tests — `read_csv` happy path and missing-file error; `write_csv`
  happy path and invalid-path error.
- Time tests — `TimeUnit::from_datetime` and `to_datetime` for all four
  variants, round-trips, and negative timestamps (before the Unix epoch);
  gated with `#![cfg(feature = "chrono")]`.

#### Code coverage

- `cargo-llvm-cov` integration: `cargo llvm-cov --html --features chrono`
  generates an HTML report at `target/llvm-cov/html/index.html`.
- CI `coverage` job generates the report on every push and pull request to
  `main`, uploading it as the `coverage-report` artifact with a 15-day
  retention period.

### Changed

- `.gitignore` extended with macOS cloud-sync artifact patterns
  (`.DS_Store`, `._*`, `.AppleDouble`, `.LSOverride`, `.icloud`, `*.icloud`).

### Fixed

- `tests/errors/test_parse_error` — temp file written to `std::env::temp_dir()`
  instead of a hard-coded relative path, fixing the test on CI where the
  `tests/fixtures/` directory does not exist.

---

## [0.1.0] - 2026-06-04

### Added

#### Core series

- `TimeSeries` struct — pairs a `Vec<i64>` index with a `Vec<f64>` value series.
- `TimeSeries::pct_change` — computes period-over-period percentage returns.
- `TimeSeries::diff` — computes first-order differences.
- `TimeSeries::shift` — shifts the series forward by N positions, filling with `NaN`.
- `TimeSeries::rolling(window)` — returns a lazy `RollingSeries` handle.
- `RollingSeries::mean` — computes the rolling mean over the configured window.
- `NaN` sentinel convention: operations that cannot produce a value at a position
  set that position to `NaN` rather than truncating the series, preserving index
  alignment.

#### Generic storage backend

- `StorageBackend<T>` trait — abstracts the in-memory layout for time-series
  values; requires `Send + Sync`.
- `ColumnarBackend<T>` — stores values in a contiguous `Vec<T>`, separate from
  the index; optimal for column-scan workloads.
- `RowBackend<T>` — stores each observation as a `RowRecord { timestamp, value }`;
  natural for record-at-a-time ingestion.
- `TemporalSeries<T, B>` — generic series backed by any `StorageBackend<T>`,
  exposing `new`, `len`, `is_empty`, `get`, and `iter`.

#### Panel

- `Panel` struct — aligns multiple named `f64` series on a shared `Vec<Timestamp>`
  index; enforces length invariants at construction time.
- `Panel::new` — validates that `symbols.len() == values.len()` and that every
  series matches the index length.
- `Panel::get_series` — extracts a named series as a `TimeSeries`.
- `Panel::shape` — returns `(n_timestamps, n_series)`.
- `Timestamp` type alias (`i64`) exposed from the `panel` module.

#### I/O

- `read_csv` — reads a two-column (`index,value`) CSV file into a `TimeSeries`.
- `write_csv` — serializes a `TimeSeries` to a two-column CSV file.

#### Error handling

- `TemporalSeriesError` enum with variants: `LengthMismatch`, `EmptySeries`,
  `InvalidWindow`, `IoError`, `ParseError`.
- All fallible operations return `Result<_, TemporalSeriesError>`.

#### Examples

- `basic` — constructs a `TimeSeries` and chains `pct_change` and `rolling().mean()`.
- `input_output` — demonstrates the CSV read/write round-trip.
- `temporal_series_with_columnar_backend` — builds a `TemporalSeries` with a
  `ColumnarBackend`.
- `temporal_series_with_row_backend` — builds a `TemporalSeries` with a
  `RowBackend`.
- `panel` — builds a `Panel`, extracts a named series, and computes returns.

#### Benchmarks

- `time_series` — benchmarks for `pct_change` and `rolling().mean()` on 1 000
  elements.
- `columnar_backend` — benchmarks for `ColumnarBackend::new`, iter sum, and `get`
  on 1 000 elements.
- `row_backend` — benchmarks for `RowBackend::new`, iter sum, and `get` on
  1 000 elements.
- `panel` — benchmarks for `Panel::new`, `get_series` (worst-case linear scan),
  and `shape` on a 1 000 × 10 panel.
- HTML reports generated automatically via Criterion's `html_reports` feature.

### Changed

- `series/core.rs` split into `series/time_series.rs` (`TimeSeries`) and
  `series/temporal_series.rs` (`TemporalSeries`) to keep each type in its own
  file.
- `panel/panel.rs` renamed to `panel/core.rs` to resolve the
  `clippy::module_inception` warning.
- Benchmarks refactored from a single `benchmark/series.rs` into four separate
  Criterion targets, each registered as its own `[[bench]]` entry in
  `Cargo.toml`.