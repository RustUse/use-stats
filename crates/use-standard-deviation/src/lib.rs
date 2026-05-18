//! Standard deviation helpers for `f64` slices.
//!
//! The functions are intentionally explicit and distinguish between population
//! and sample standard deviation.
//!
//! # Examples
//!
//! ```rust
//! use use_standard_deviation::population_standard_deviation;
//!
//! let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
//! assert_eq!(population_standard_deviation(&values).unwrap(), 2.0);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StandardDeviationError {
    EmptyInput,
    InsufficientData,
}

pub fn population_standard_deviation(values: &[f64]) -> Result<f64, StandardDeviationError> {
    if values.is_empty() {
        return Err(StandardDeviationError::EmptyInput);
    }

    Ok((sum_squared_deviations(values) / values.len() as f64).sqrt())
}

pub fn sample_standard_deviation(values: &[f64]) -> Result<f64, StandardDeviationError> {
    if values.len() < 2 {
        return Err(StandardDeviationError::InsufficientData);
    }

    Ok((sum_squared_deviations(values) / (values.len() as f64 - 1.0)).sqrt())
}

fn sum_squared_deviations(values: &[f64]) -> f64 {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    values.iter().map(|value| (value - mean).powi(2)).sum()
}

#[cfg(test)]
mod tests {
    use super::{StandardDeviationError, population_standard_deviation, sample_standard_deviation};

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_population_and_sample_standard_deviation() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let expected_sample_standard_deviation = (32.0_f64 / 7.0_f64).sqrt();

        approx_eq(population_standard_deviation(&values).unwrap(), 2.0);
        approx_eq(
            sample_standard_deviation(&values).unwrap(),
            expected_sample_standard_deviation,
        );
    }

    #[test]
    fn rejects_invalid_standard_deviation_inputs() {
        assert_eq!(
            population_standard_deviation(&[]),
            Err(StandardDeviationError::EmptyInput)
        );
        assert_eq!(
            sample_standard_deviation(&[5.0]),
            Err(StandardDeviationError::InsufficientData)
        );
    }

    #[test]
    fn handles_single_value_population_standard_deviation() {
        approx_eq(population_standard_deviation(&[5.0]).unwrap(), 0.0);
    }
}
