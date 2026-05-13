# use-stats

Composable statistical primitives for RustUse.

`use-stats` is the umbrella crate for the RustUse statistics workspace. It
reexports the focused crates and provides a `prelude` for the most common
helpers.

## Included crates

- `use-average`
- `use-variance`
- `use-standard-deviation`
- `use-percentile`
- `use-z-score`
- `use-correlation`
- `use-distribution`

## Example

```rust
use use_stats::prelude::{arithmetic_mean, covariance, histogram, median, normalize, range};

let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
let paired = [2.0, 4.0, 6.0, 8.0, 10.0];

assert_eq!(arithmetic_mean(&values).unwrap(), 5.0);
assert_eq!(median(&values).unwrap(), 4.5);
assert_eq!(range(&values).unwrap(), 7.0);
assert_eq!(normalize(&[1.0, 2.0, 3.0]).unwrap().len(), 3);
assert_eq!(covariance(&[1.0, 2.0, 3.0, 4.0, 5.0], &paired).unwrap(), 4.0);
assert_eq!(histogram(&[1.0, 2.0, 2.0, 3.0, 4.0], 3).unwrap().len(), 3);
```
