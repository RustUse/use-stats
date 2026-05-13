pub use use_average::{
    arithmetic_mean, geometric_mean, harmonic_mean, moving_average, weighted_mean, AverageError,
};
pub use use_correlation::{covariance, pearson_correlation, CorrelationError};
pub use use_distribution::{
    frequency_counts, histogram, max, min, range, DistributionError, HistogramBucket,
};
pub use use_percentile::{
    median, percentile, percentile_rank, quartiles, PercentileError, Quartiles,
};
pub use use_standard_deviation::{
    population_standard_deviation, sample_standard_deviation, StandardDeviationError,
};
pub use use_variance::{
    population_variance, sample_variance, sum_squared_deviations, VarianceError,
};
pub use use_z_score::{normalize, z_score, ZScoreError};
