# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on Keep a Changelog and Semantic Versioning.

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