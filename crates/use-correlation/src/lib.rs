//! Correlation helpers for `f64` slices.
//!
//! The crate provides a population covariance helper and a Pearson correlation
//! helper for paired slices of equal length.
//!
//! # Examples
//!
//! ```rust
//! use use_correlation::{covariance, pearson_correlation};
//!
//! let x = [1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = [2.0, 4.0, 6.0, 8.0, 10.0];
//!
//! assert_eq!(covariance(&x, &y).unwrap(), 4.0);
//! assert!((pearson_correlation(&x, &y).unwrap() - 1.0).abs() < 1.0e-12);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorrelationError {
    EmptyInput,
    MismatchedLengths,
    ZeroVariance,
}

pub fn covariance(x_values: &[f64], y_values: &[f64]) -> Result<f64, CorrelationError> {
    validate_pair(x_values, y_values)?;

    let x_mean = mean(x_values);
    let y_mean = mean(y_values);
    let covariance_sum: f64 = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(x_value, y_value)| (x_value - x_mean) * (y_value - y_mean))
        .sum();

    Ok(covariance_sum / x_values.len() as f64)
}

pub fn pearson_correlation(x_values: &[f64], y_values: &[f64]) -> Result<f64, CorrelationError> {
    validate_pair(x_values, y_values)?;

    let x_mean = mean(x_values);
    let y_mean = mean(y_values);

    let numerator: f64 = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(x_value, y_value)| (x_value - x_mean) * (y_value - y_mean))
        .sum();
    let x_squared_deviation_sum: f64 = x_values.iter().map(|value| (value - x_mean).powi(2)).sum();
    let y_squared_deviation_sum: f64 = y_values.iter().map(|value| (value - y_mean).powi(2)).sum();
    let denominator = x_squared_deviation_sum.sqrt() * y_squared_deviation_sum.sqrt();

    if denominator == 0.0 {
        return Err(CorrelationError::ZeroVariance);
    }

    Ok(numerator / denominator)
}

fn validate_pair(x_values: &[f64], y_values: &[f64]) -> Result<(), CorrelationError> {
    if x_values.is_empty() || y_values.is_empty() {
        return Err(CorrelationError::EmptyInput);
    }

    if x_values.len() != y_values.len() {
        return Err(CorrelationError::MismatchedLengths);
    }

    Ok(())
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

#[cfg(test)]
mod tests {
    use super::{covariance, pearson_correlation, CorrelationError};

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_covariance_and_positive_correlation() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];

        approx_eq(covariance(&x, &y).unwrap(), 4.0);
        approx_eq(pearson_correlation(&x, &y).unwrap(), 1.0);
    }

    #[test]
    fn computes_negative_correlation() {
        let x = [1.0, 2.0, 3.0];
        let y = [3.0, 2.0, 1.0];

        approx_eq(pearson_correlation(&x, &y).unwrap(), -1.0);
    }

    #[test]
    fn handles_single_value_covariance() {
        approx_eq(covariance(&[5.0], &[8.0]).unwrap(), 0.0);
        assert_eq!(
            pearson_correlation(&[5.0], &[8.0]),
            Err(CorrelationError::ZeroVariance)
        );
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(covariance(&[], &[]), Err(CorrelationError::EmptyInput));
        assert_eq!(
            covariance(&[1.0, 2.0], &[1.0]),
            Err(CorrelationError::MismatchedLengths)
        );
        assert_eq!(
            pearson_correlation(&[2.0, 2.0], &[1.0, 2.0]),
            Err(CorrelationError::ZeroVariance)
        );
    }
}
