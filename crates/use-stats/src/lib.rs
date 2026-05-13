#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_average;
pub use use_correlation;
pub use use_distribution;
pub use use_percentile;
pub use use_standard_deviation;
pub use use_variance;
pub use use_z_score;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        arithmetic_mean, covariance, median, normalize, pearson_correlation,
        population_standard_deviation, population_variance, quartiles, range, z_score,
    };

    #[test]
    fn facade_exposes_common_statistics_helpers() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];

        assert_eq!(arithmetic_mean(&values).expect("mean should compute"), 5.0);
        assert_eq!(
            population_variance(&values).expect("variance should compute"),
            4.0
        );
        assert_eq!(
            population_standard_deviation(&values).expect("standard deviation should compute"),
            2.0
        );
        assert_eq!(median(&values).expect("median should compute"), 4.5);
        assert_eq!(range(&values).expect("range should compute"), 7.0);
        assert_eq!(covariance(&x, &y).expect("covariance should compute"), 4.0);
        assert!(
            (pearson_correlation(&x, &y).expect("correlation should compute") - 1.0).abs()
                < 1.0e-12
        );
        assert_eq!(
            z_score(80.0, 70.0, 5.0).expect("z-score should compute"),
            2.0
        );
        assert_eq!(
            normalize(&[1.0, 2.0, 3.0])
                .expect("normalization should compute")
                .len(),
            3
        );

        let summary = quartiles(&values).expect("quartiles should compute");
        assert_eq!(summary.lower, 4.0);
        assert_eq!(summary.median, 4.5);
        assert_eq!(summary.upper, 5.5);
    }
}
