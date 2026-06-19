# Architecture

## Overview

`temporalseries` is a single Rust crate that exposes three public types —
`TimeSeries`, `TemporalSeries<T, B>`, and `Panel` — and a shared analytical
method surface that works identically across all three. The design is built
around two principles: **separation of concerns at every layer**, and
**testability as a first-class constraint**.

---

## Module layout

```
crates/
├── lib.rs                    # public re-exports only; no logic
├── errors/
│   └── types.rs              # TemporalSeriesError — all error variants live here
├── storage/
│   ├── backend.rs            # StorageBackend<T> trait
│   ├── columnar.rs           # ColumnarBackend<T>  — contiguous Vec<T>
│   ├── row.rs                # RowBackend<T>       — Vec<RowRecord<T>>
│   └── chunked.rs            # ChunkedBackend<T>   — future work
├── series/
│   ├── time_series.rs        # TimeSeries          — concrete Vec<i64>/Vec<f64> series
│   └── temporal_series.rs    # TemporalSeries<T,B> — generic over T and backend
├── panel/
│   └── core.rs               # Panel               — named columns on a shared index
├── rolling/
│   └── core.rs               # RollingSeries       — lazy rolling-window handle
├── io/
│   └── csv.rs                # read_csv / write_csv
└── time/
    └── unit.rs               # TimeUnit enum (chrono feature)

tests/
├── integration.rs            # single test binary entry point
├── errors/                   # one file per TemporalSeriesError variant
├── io/                       # read_csv / write_csv
├── rolling/                  # rolling().mean()
├── series/
│   ├── time_series/          # one file per TimeSeries method
│   └── temporal_series/      # one file per TemporalSeries method
├── panel/
│   ├── test_panel.rs         # Panel construction and structural methods
│   └── test_panel_methods.rs # all analytical methods on Panel
├── storage/                  # ColumnarBackend and RowBackend
└── time/                     # TimeUnit (chrono feature)
```

The test tree is a direct mirror of the source tree. Every crate module has a
corresponding test directory; every type has a corresponding test file per
method. Finding the test for a method is always a predictable path lookup, never
a search.

---

## Layer diagram

```
┌─────────────────────────────────────────────────────┐
│                       Panel                          │
│  delegates column-by-column to TimeSeries            │
└───────────────────────────┬─────────────────────────┘
                            │ calls
         ┌──────────────────┴──────────────────┐
         │            TimeSeries               │
         │  Vec<i64> index + Vec<f64> values   │
         │  owns all analytical logic          │
         └──────────────────┬──────────────────┘
                            │ mirrors API via
         ┌──────────────────┴──────────────────┐
         │     TemporalSeries<f64, B>          │
         │  generic over StorageBackend<f64>   │
         │  delegates to TimeSeries internally │
         └──────────────────┬──────────────────┘
                            │ backed by
         ┌──────────────────┴──────────────────┐
         │         StorageBackend<T> trait      │
         │  ColumnarBackend   │   RowBackend    │
         └─────────────────────────────────────┘
```

Each layer only depends on the layer immediately below it. `Panel` never touches
`StorageBackend`. `TemporalSeries` never knows about `Panel`. Errors flow up
through `TemporalSeriesError`, which is the only cross-cutting dependency.

---

## `StorageBackend<T>` — the decoupling seam

```rust
pub trait StorageBackend<T>: Send + Sync {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
    fn iter(&self) -> impl Iterator<Item = &T>;
    // ...
}
```

`TemporalSeries<T, B>` is generic over any `B: StorageBackend<T>`. This means:

- `ColumnarBackend<T>` — contiguous `Vec<T>`, best for column-scan workloads.
- `RowBackend<T>` — `Vec<RowRecord<T>>`, natural for record-at-a-time ingestion.
- Any future backend (chunked, memory-mapped, lazy) drops in without touching
  any analytical code or any test.

The trait boundary is the only contract between the storage layer and the series
layer. Nothing else bleeds across.

---

## Analytical method ownership

All analytical logic lives in exactly one place: `TimeSeries`. The other two
types are **delegation wrappers**, not reimplementations.

### `TemporalSeries<f64, B>`

Methods are defined on `impl<B: StorageBackend<f64>> TemporalSeries<f64, B>`.
Because `TemporalSeries` has no `Vec<f64>` field, it extracts values once via
`self.iter().copied().collect()` and forwards to the same algorithm as
`TimeSeries`. Series-returning methods always produce a concrete `ColSeries`
(= `TemporalSeries<f64, ColumnarBackend<f64>>`) rather than a generic `B`,
since there is no way to construct an arbitrary backend from a `Vec<f64>`
without coupling the trait to its implementors.

### `Panel`

Three private helpers keep every public method a one-liner:

| Helper | Purpose |
|---|---|
| `col_series(i)` | wraps column `i` as a `TimeSeries` |
| `scalar_map(f)` | applies `f` to each column, collects into `HashMap<String, f64>` |
| `bool_map(f)` | same for `bool` results |
| `panel_map(f)` | applies `f` to each column, assembles results into a new `Panel` |

`bollinger_bands` is the only exception — it returns three panels and handles
the triple manually. Every other method is a single `self.scalar_map(...)` or
`self.panel_map(...)` call.

---

## Error handling

`TemporalSeriesError` is the single error type for the entire library:

```
TemporalSeriesError
├── LengthMismatch { index_len, values_len }
├── EmptySeries
├── InvalidWindow { window, series_len }
├── IoError(std::io::Error)
└── ParseError(String)
```

All fallible public methods return `Result<_, TemporalSeriesError>`. There are
no panics in library code — all `unwrap` calls are confined to test bodies and
examples. This makes the error surface predictable and exhaustively testable:
each variant has its own test file under `tests/errors/`.

---

## How the architecture enables full testing

### One-to-one source/test mapping

Every source module has a direct test counterpart:

```
crates/series/time_series.rs
  → tests/series/time_series/test_mean.rs
  → tests/series/time_series/test_std_deviation.rs
  → ...  (one file per method)

crates/series/temporal_series.rs
  → tests/series/temporal_series/test_mean.rs
  → ...

crates/panel/core.rs
  → tests/panel/test_panel.rs          (structural)
  → tests/panel/test_panel_methods.rs  (analytical)
```

Adding a new method means adding a new test file with a known name in a known
location. There is no discovery problem.

### `Panel` tests verify delegation, not logic

`Panel` holds no analytical logic of its own. Its tests therefore do not
duplicate the mathematical assertions from the `TimeSeries` tests — they verify
only that delegation is wired correctly:

```rust
assert_eq!(
    panel.mean()["AAPL"],
    panel.get_series("AAPL").unwrap().mean()
);
```

If the underlying `TimeSeries::mean` is correct (verified by its own tests) and
the Panel's routing is correct (verified by this assertion), the system is
correct. No mathematical property needs to be tested twice.

### `TemporalSeries` tests verify the generic path

`TemporalSeries` tests use `ColumnarBackend` as the concrete backend — not
because the tests are backend-specific, but because any `StorageBackend<f64>`
implementation exercises the same generic path. If a new backend is added, it
only needs to satisfy the `StorageBackend` contract; all analytical correctness
is already covered.

### Isolated error tests

Each `TemporalSeriesError` variant is tested in isolation:

```
tests/errors/test_length_mismatch.rs   → LengthMismatch
tests/errors/test_empty_series.rs      → EmptySeries
tests/errors/test_invalid_window.rs    → InvalidWindow
tests/errors/test_io_error.rs          → IoError
tests/errors/test_parse_error.rs       → ParseError
```

Each file covers `Display`, `Debug`, and `std::error::Error::source()`. Error
behaviour is a contract, and it is tested like one.

### Single integration binary

All tests compile into a single binary via `tests/integration.rs`, which
declares one `mod` per test directory. This keeps compilation fast, avoids
linker overhead from many small test binaries, and makes `cargo test` output a
flat list that mirrors the module hierarchy:

```
test series::time_series::test_mean::...
test series::temporal_series::test_mean::...
test panel::test_panel_methods::...
```

---

## Optional features

The `chrono` feature is the only optional dependency. It gates two methods on
`TemporalSeries` (`from_datetimes`, `datetimes`) and one test module
(`tests/time/`). The `TimeUnit` type and `with_unit`/`time_unit` accessors are
always compiled in, so the feature boundary is narrow: only the `DateTime<Utc>`
conversion crosses it. This keeps the default build free of heavy dependencies
while making the feature easy to test in isolation with `--features chrono`.

---

## Invariants enforced at the boundary

- `TimeSeries::new` — rejects `index.len() != values.len()`.
- `Panel::new` — rejects `symbols.len() != values.len()` and any series whose
  length differs from the index.
- `TemporalSeries::new` — rejects `index.len() != backend.len()`.

Once constructed, all three types are internally consistent. No method needs to
re-validate lengths; the invariant is established once and trusted everywhere
downstream. This is the standard Rust pattern of making invalid state
unrepresentable, and it is what allows test bodies to call `.unwrap()` on
construction without it being sloppy — the construction is the test of the
boundary, and the rest of the test exercises behaviour, not validation.
