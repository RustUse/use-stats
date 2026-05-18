//! Percentile helpers for `f64` slices.
//!
//! Percentiles use linear interpolation over the sorted values with a rank in the
//! inclusive range from `0` to `100`.
//!
//! # Examples
//!
//! ```rust
//! use use_percentile::{median, percentile, percentile_rank};
//!
//! let values = [15.0, 20.0, 35.0, 40.0, 50.0];
//! assert_eq!(median(&values).unwrap(), 35.0);
//! assert_eq!(percentile(&values, 25.0).unwrap(), 20.0);
//! assert_eq!(percentile_rank(&values, 40.0).unwrap(), 80.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quartiles {
    pub lower: f64,
    pub median: f64,
    pub upper: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PercentileError {
    EmptyInput,
    InvalidPercentile,
}

pub fn median(values: &[f64]) -> Result<f64, PercentileError> {
    percentile(values, 50.0)
}

pub fn quartiles(values: &[f64]) -> Result<Quartiles, PercentileError> {
    Ok(Quartiles {
        lower: percentile(values, 25.0)?,
        median: percentile(values, 50.0)?,
        upper: percentile(values, 75.0)?,
    })
}

pub fn percentile(values: &[f64], percentile: f64) -> Result<f64, PercentileError> {
    if values.is_empty() {
        return Err(PercentileError::EmptyInput);
    }

    if !(0.0..=100.0).contains(&percentile) {
        return Err(PercentileError::InvalidPercentile);
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);

    let last_index = sorted.len() - 1;
    let rank = percentile / 100.0 * last_index as f64;
    let lower_index = rank.floor() as usize;
    let upper_index = rank.ceil() as usize;

    if lower_index == upper_index {
        return Ok(sorted[lower_index]);
    }

    let weight = rank - lower_index as f64;
    let lower = sorted[lower_index];
    let upper = sorted[upper_index];
    Ok(lower + (upper - lower) * weight)
}

pub fn percentile_rank(values: &[f64], value: f64) -> Result<f64, PercentileError> {
    if values.is_empty() {
        return Err(PercentileError::EmptyInput);
    }

    let count = values.iter().filter(|item| **item <= value).count();
    Ok(count as f64 / values.len() as f64 * 100.0)
}

#[cfg(test)]
mod tests {
    use super::{PercentileError, Quartiles, median, percentile, percentile_rank, quartiles};

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_median_and_percentiles() {
        let values = [15.0, 20.0, 35.0, 40.0, 50.0];

        approx_eq(median(&values).unwrap(), 35.0);
        approx_eq(percentile(&values, 40.0).unwrap(), 29.0);
        approx_eq(percentile_rank(&values, 40.0).unwrap(), 80.0);
    }

    #[test]
    fn computes_quartiles() {
        let values = [15.0, 20.0, 35.0, 40.0, 50.0];

        assert_eq!(
            quartiles(&values).unwrap(),
            Quartiles {
                lower: 20.0,
                median: 35.0,
                upper: 40.0,
            }
        );
    }

    #[test]
    fn handles_single_value_input() {
        assert_eq!(median(&[4.0]).unwrap(), 4.0);
        assert_eq!(
            quartiles(&[4.0]).unwrap(),
            Quartiles {
                lower: 4.0,
                median: 4.0,
                upper: 4.0,
            }
        );
        assert_eq!(percentile_rank(&[4.0], 4.0).unwrap(), 100.0);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(median(&[]), Err(PercentileError::EmptyInput));
        assert_eq!(
            percentile(&[1.0, 2.0], -1.0),
            Err(PercentileError::InvalidPercentile)
        );
        assert_eq!(
            percentile(&[1.0, 2.0], 101.0),
            Err(PercentileError::InvalidPercentile)
        );
    }
}
