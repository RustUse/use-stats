//! Z-score helpers for `f64` slices.
//!
//! Normalization uses the population standard deviation of the provided sample.
//!
//! # Examples
//!
//! ```rust
//! use use_z_score::{normalize, z_score};
//!
//! assert_eq!(z_score(80.0, 70.0, 5.0).unwrap(), 2.0);
//! assert_eq!(normalize(&[1.0, 2.0, 3.0]).unwrap().len(), 3);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZScoreError {
    EmptyInput,
    ZeroStandardDeviation,
}

pub fn z_score(value: f64, mean: f64, standard_deviation: f64) -> Result<f64, ZScoreError> {
    if standard_deviation == 0.0 {
        return Err(ZScoreError::ZeroStandardDeviation);
    }

    Ok((value - mean) / standard_deviation)
}

pub fn normalize(values: &[f64]) -> Result<Vec<f64>, ZScoreError> {
    if values.is_empty() {
        return Err(ZScoreError::EmptyInput);
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;
    let standard_deviation = variance.sqrt();

    if standard_deviation == 0.0 {
        return Err(ZScoreError::ZeroStandardDeviation);
    }

    values
        .iter()
        .map(|value| z_score(*value, mean, standard_deviation))
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::{ZScoreError, normalize, z_score};

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_single_z_score() {
        approx_eq(z_score(80.0, 70.0, 5.0).unwrap(), 2.0);
    }

    #[test]
    fn normalizes_known_values() {
        let normalized = normalize(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).unwrap();
        let expected = [-1.5, -0.5, -0.5, -0.5, 0.0, 0.0, 1.0, 2.0];

        for (left, right) in normalized.iter().zip(expected) {
            approx_eq(*left, right);
        }
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(normalize(&[]), Err(ZScoreError::EmptyInput));
        assert_eq!(
            normalize(&[3.0, 3.0, 3.0]),
            Err(ZScoreError::ZeroStandardDeviation)
        );
        assert_eq!(
            z_score(10.0, 10.0, 0.0),
            Err(ZScoreError::ZeroStandardDeviation)
        );
    }

    #[test]
    fn rejects_single_value_normalization() {
        assert_eq!(normalize(&[5.0]), Err(ZScoreError::ZeroStandardDeviation));
    }
}
