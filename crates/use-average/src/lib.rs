//! Average helpers for `f64` slices.
//!
//! The crate stays intentionally small and focuses on the most common mean-style
//! summaries without introducing a broader statistics framework.
//!
//! # Examples
//!
//! ```rust
//! use use_average::{arithmetic_mean, moving_average};
//!
//! assert_eq!(arithmetic_mean(&[2.0, 4.0, 6.0]).unwrap(), 4.0);
//! assert_eq!(moving_average(&[1.0, 2.0, 3.0, 4.0], 2).unwrap(), vec![1.5, 2.5, 3.5]);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AverageError {
    EmptyInput,
    MismatchedLengths,
    ZeroWeightSum,
    NegativeValue,
    ZeroValue,
    InvalidWindow,
}

pub fn arithmetic_mean(values: &[f64]) -> Result<f64, AverageError> {
    if values.is_empty() {
        return Err(AverageError::EmptyInput);
    }

    Ok(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn weighted_mean(values: &[f64], weights: &[f64]) -> Result<f64, AverageError> {
    if values.is_empty() || weights.is_empty() {
        return Err(AverageError::EmptyInput);
    }

    if values.len() != weights.len() {
        return Err(AverageError::MismatchedLengths);
    }

    let weight_sum: f64 = weights.iter().sum();
    if weight_sum == 0.0 {
        return Err(AverageError::ZeroWeightSum);
    }

    let weighted_sum: f64 = values
        .iter()
        .zip(weights.iter())
        .map(|(value, weight)| value * weight)
        .sum();

    Ok(weighted_sum / weight_sum)
}

pub fn geometric_mean(values: &[f64]) -> Result<f64, AverageError> {
    if values.is_empty() {
        return Err(AverageError::EmptyInput);
    }

    if values.iter().any(|value| *value < 0.0) {
        return Err(AverageError::NegativeValue);
    }

    if values.contains(&0.0) {
        return Ok(0.0);
    }

    let log_sum: f64 = values.iter().map(|value| value.ln()).sum();
    Ok((log_sum / values.len() as f64).exp())
}

pub fn harmonic_mean(values: &[f64]) -> Result<f64, AverageError> {
    if values.is_empty() {
        return Err(AverageError::EmptyInput);
    }

    if values.contains(&0.0) {
        return Err(AverageError::ZeroValue);
    }

    let reciprocal_sum: f64 = values.iter().map(|value| 1.0 / value).sum();
    Ok(values.len() as f64 / reciprocal_sum)
}

pub fn moving_average(values: &[f64], window_size: usize) -> Result<Vec<f64>, AverageError> {
    if values.is_empty() {
        return Err(AverageError::EmptyInput);
    }

    if window_size == 0 || window_size > values.len() {
        return Err(AverageError::InvalidWindow);
    }

    values
        .windows(window_size)
        .map(arithmetic_mean)
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::{
        AverageError, arithmetic_mean, geometric_mean, harmonic_mean, moving_average, weighted_mean,
    };

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_arithmetic_mean() {
        approx_eq(arithmetic_mean(&[2.0, 4.0, 6.0, 8.0]).unwrap(), 5.0);
    }

    #[test]
    fn computes_weighted_mean() {
        approx_eq(
            weighted_mean(&[80.0, 90.0, 70.0], &[0.2, 0.5, 0.3]).unwrap(),
            82.0,
        );
    }

    #[test]
    fn computes_geometric_mean() {
        approx_eq(geometric_mean(&[1.0, 4.0, 16.0]).unwrap(), 4.0);
    }

    #[test]
    fn computes_harmonic_mean() {
        approx_eq(harmonic_mean(&[1.0, 2.0, 4.0]).unwrap(), 12.0 / 7.0);
    }

    #[test]
    fn computes_moving_average() {
        assert_eq!(
            moving_average(&[1.0, 2.0, 3.0, 4.0, 5.0], 3).unwrap(),
            vec![2.0, 3.0, 4.0]
        );
    }

    #[test]
    fn returns_errors_for_invalid_average_inputs() {
        assert_eq!(arithmetic_mean(&[]), Err(AverageError::EmptyInput));
        assert_eq!(
            weighted_mean(&[1.0, 2.0], &[1.0]),
            Err(AverageError::MismatchedLengths)
        );
        assert_eq!(
            weighted_mean(&[1.0, 2.0], &[0.0, 0.0]),
            Err(AverageError::ZeroWeightSum)
        );
        assert_eq!(
            geometric_mean(&[-1.0, 2.0]),
            Err(AverageError::NegativeValue)
        );
        assert_eq!(harmonic_mean(&[1.0, 0.0]), Err(AverageError::ZeroValue));
        assert_eq!(
            moving_average(&[1.0, 2.0], 0),
            Err(AverageError::InvalidWindow)
        );
    }

    #[test]
    fn handles_single_value_inputs() {
        approx_eq(arithmetic_mean(&[9.0]).unwrap(), 9.0);
        approx_eq(weighted_mean(&[9.0], &[3.0]).unwrap(), 9.0);
        approx_eq(geometric_mean(&[9.0]).unwrap(), 9.0);
        approx_eq(harmonic_mean(&[9.0]).unwrap(), 9.0);
        assert_eq!(moving_average(&[9.0], 1).unwrap(), vec![9.0]);
    }
}
