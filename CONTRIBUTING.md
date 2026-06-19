# Contributing

Thank you for your interest in contributing to `temporalseries`.

## Getting started

```bash
git clone https://github.com/manuelgijon/temporalseries
cd temporalseries
cargo build
cargo test
```

## Before opening a pull request

```bash
# All tests must pass
cargo test

# No clippy warnings
cargo clippy -- -D warnings

# Doc examples must compile and pass
cargo test --doc

# If you touched any public API, check the rendered docs
cargo doc --open
```

## Adding a new method

`TimeSeries`, `TemporalSeries`, and `Panel` share the same analytical surface.
When you add a method to one, add it to the others too:

1. **`crates/series/time_series.rs`** — implement the method on `TimeSeries`.
2. **`crates/series/temporal_series.rs`** — implement the same method on
   `impl<B: StorageBackend<f64>> TemporalSeries<f64, B>`.
3. **`crates/panel/core.rs`** — delegate to `TimeSeries` via `self.col_series(i)`
   and the appropriate private helper (`scalar_map`, `bool_map`, `panel_map`).
4. Add a `# Examples` doc section to each new public method.
5. Add unit tests under `tests/series/time_series/`, `tests/series/temporal_series/`,
   and `tests/panel/` following the existing `test__given_X__when_Y__then_Z` naming
   convention and `// Given / // When / // Then` comment structure.

## Test conventions

- Test function names follow `test__given_X__when_Y__then_Z`.
- Each test file is named `test_<method>.rs` and declared as a `mod` in the
  parent `mod.rs`.
- All test functions are annotated with `#[allow(non_snake_case)]`.
- Use `// Given`, `// When`, `// Then` comments to structure each test body.
- When comparing `Vec<f64>` results that may contain `NaN` sentinels, use the
  `assert_f64_vecs_eq` helper (see `tests/panel/test_panel_methods.rs`) rather
  than plain `assert_eq!`.

## Commit style

Short imperative subject line (≤ 72 characters), no trailing period.
Add a blank line and a brief body when the motivation is not obvious from the
diff. Reference issues with `Fixes #N` or `Closes #N`.

## License

By contributing you agree that your work will be released under the MIT licence.
