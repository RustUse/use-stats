# use-stats

Composable statistical primitives for Rust.

`use-stats` is a RustUse sibling workspace beside repositories such as `use-math`, `use-color`, `use-text`, `use-time`, and `use-units`. It groups small, focused crates for averages, variance, standard deviation, percentiles, z-scores, correlation, and distribution helpers.

The RustUse approach here is intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor plain `f64` helpers over framework-style abstractions
- dependencies stay minimal so each crate is easy to audit and adopt

## Workspace crates

- `use-stats`: umbrella crate that reexports the full workspace with a shared prelude
- `use-average`: arithmetic, weighted, geometric, harmonic, and moving averages
- `use-variance`: population variance, sample variance, and squared-deviation helpers
- `use-standard-deviation`: population and sample standard deviation helpers
- `use-percentile`: median, quartiles, percentiles, and percentile-rank helpers
- `use-z-score`: standard-score and z-score normalization helpers
- `use-correlation`: covariance and Pearson correlation helpers
- `use-distribution`: min, max, range, frequency counts, and histogram helpers

## Umbrella crate

If you want a single dependency for the whole workspace, use `use-stats`. It
reexports each focused crate and provides a `prelude` for the most common
statistics helpers.

```rust
use use_stats::prelude::{arithmetic_mean, median, population_standard_deviation, range};

let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

assert_eq!(arithmetic_mean(&values).unwrap(), 5.0);
assert_eq!(median(&values).unwrap(), 4.5);
assert_eq!(population_standard_deviation(&values).unwrap(), 2.0);
assert_eq!(range(&values).unwrap(), 7.0);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the public API to stay small and practical, but still evolve as the RustUse statistics surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```