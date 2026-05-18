//! Variance helpers for `f64` slices.
//!
//! This crate exposes population and sample variance calculations together with a
//! reusable sum-of-squared-deviations helper.
//!
//! # Examples
//!
//! ```rust
//! use use_variance::{population_variance, sample_variance};
//!
//! let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
//! assert_eq!(population_variance(&values).unwrap(), 4.0);
//! assert!((sample_variance(&values).unwrap() - (32.0 / 7.0)).abs() < 1.0e-10);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarianceError {
    EmptyInput,
    InsufficientData,
}

pub fn sum_squared_deviations(values: &[f64]) -> Result<f64, VarianceError> {
    if values.is_empty() {
        return Err(VarianceError::EmptyInput);
    }

    let mean = mean(values);
    Ok(values.iter().map(|value| (value - mean).powi(2)).sum())
}

pub fn population_variance(values: &[f64]) -> Result<f64, VarianceError> {
    let squared_deviation_sum = sum_squared_deviations(values)?;
    Ok(squared_deviation_sum / values.len() as f64)
}

pub fn sample_variance(values: &[f64]) -> Result<f64, VarianceError> {
    if values.len() < 2 {
        return Err(VarianceError::InsufficientData);
    }

    let squared_deviation_sum = sum_squared_deviations(values)?;
    Ok(squared_deviation_sum / (values.len() as f64 - 1.0))
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

#[cfg(test)]
mod tests {
    use super::{VarianceError, population_variance, sample_variance, sum_squared_deviations};

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-10, "left={left}, right={right}");
    }

    #[test]
    fn computes_population_and_sample_variance() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

        approx_eq(sum_squared_deviations(&values).unwrap(), 32.0);
        approx_eq(population_variance(&values).unwrap(), 4.0);
        approx_eq(sample_variance(&values).unwrap(), 32.0 / 7.0);
    }

    #[test]
    fn rejects_empty_input() {
        assert_eq!(sum_squared_deviations(&[]), Err(VarianceError::EmptyInput));
        assert_eq!(population_variance(&[]), Err(VarianceError::EmptyInput));
    }

    #[test]
    fn handles_single_value_population_variance() {
        approx_eq(population_variance(&[5.0]).unwrap(), 0.0);
        assert_eq!(
            sample_variance(&[5.0]),
            Err(VarianceError::InsufficientData)
        );
    }
}
