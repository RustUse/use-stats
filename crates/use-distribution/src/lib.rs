//! Basic distribution summaries for `f64` slices.
//!
//! The helpers cover minimum and maximum lookup, range calculation, frequency
//! counts, and a small equal-width histogram representation.
//!
//! # Examples
//!
//! ```rust
//! use use_distribution::{frequency_counts, histogram, range};
//!
//! assert_eq!(range(&[1.0, 3.0, 5.0]).unwrap(), 4.0);
//! assert_eq!(frequency_counts(&[1.0, 1.0, 2.0]).unwrap(), vec![(1.0, 2), (2.0, 1)]);
//! assert_eq!(histogram(&[1.0, 2.0, 2.0, 3.0, 4.0], 3).unwrap().len(), 3);
//! ```

use core::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct HistogramBucket {
    pub start: f64,
    pub end: f64,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributionError {
    EmptyInput,
    InvalidBucketCount,
}

pub fn min(values: &[f64]) -> Result<f64, DistributionError> {
    values
        .iter()
        .copied()
        .min_by(f64::total_cmp)
        .ok_or(DistributionError::EmptyInput)
}

pub fn max(values: &[f64]) -> Result<f64, DistributionError> {
    values
        .iter()
        .copied()
        .max_by(f64::total_cmp)
        .ok_or(DistributionError::EmptyInput)
}

pub fn range(values: &[f64]) -> Result<f64, DistributionError> {
    Ok(max(values)? - min(values)?)
}

pub fn frequency_counts(values: &[f64]) -> Result<Vec<(f64, usize)>, DistributionError> {
    if values.is_empty() {
        return Err(DistributionError::EmptyInput);
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);

    let mut counts: Vec<(f64, usize)> = Vec::new();
    for value in sorted {
        match counts.last_mut() {
            Some((existing, count)) if existing.total_cmp(&value) == Ordering::Equal => *count += 1,
            _ => counts.push((value, 1)),
        }
    }

    Ok(counts)
}

pub fn histogram(
    values: &[f64],
    bucket_count: usize,
) -> Result<Vec<HistogramBucket>, DistributionError> {
    if values.is_empty() {
        return Err(DistributionError::EmptyInput);
    }

    if bucket_count == 0 {
        return Err(DistributionError::InvalidBucketCount);
    }

    let min_value = min(values)?;
    let max_value = max(values)?;

    if min_value == max_value {
        return Ok(vec![HistogramBucket {
            start: min_value,
            end: max_value,
            count: values.len(),
        }]);
    }

    let width = (max_value - min_value) / bucket_count as f64;
    let mut buckets = (0..bucket_count)
        .map(|index| HistogramBucket {
            start: min_value + index as f64 * width,
            end: if index + 1 == bucket_count {
                max_value
            } else {
                min_value + (index + 1) as f64 * width
            },
            count: 0,
        })
        .collect::<Vec<_>>();

    for value in values {
        let raw_index = ((value - min_value) / width).floor() as usize;
        let index = raw_index.min(bucket_count - 1);
        buckets[index].count += 1;
    }

    Ok(buckets)
}

#[cfg(test)]
mod tests {
    use super::{DistributionError, HistogramBucket, frequency_counts, histogram, max, min, range};

    #[test]
    fn computes_min_max_and_range() {
        assert_eq!(min(&[4.0, 1.0, 7.0, 3.0]).unwrap(), 1.0);
        assert_eq!(max(&[4.0, 1.0, 7.0, 3.0]).unwrap(), 7.0);
        assert_eq!(range(&[4.0, 1.0, 7.0, 3.0]).unwrap(), 6.0);
    }

    #[test]
    fn computes_frequency_counts() {
        assert_eq!(
            frequency_counts(&[3.0, 1.0, 3.0, 2.0, 1.0, 3.0]).unwrap(),
            vec![(1.0, 2), (2.0, 1), (3.0, 3)]
        );
    }

    #[test]
    fn computes_histogram_buckets() {
        assert_eq!(
            histogram(&[1.0, 2.0, 2.0, 3.0, 4.0], 3).unwrap(),
            vec![
                HistogramBucket {
                    start: 1.0,
                    end: 2.0,
                    count: 1,
                },
                HistogramBucket {
                    start: 2.0,
                    end: 3.0,
                    count: 2,
                },
                HistogramBucket {
                    start: 3.0,
                    end: 4.0,
                    count: 2,
                },
            ]
        );
    }

    #[test]
    fn handles_single_value_inputs() {
        assert_eq!(range(&[5.0]).unwrap(), 0.0);
        assert_eq!(
            histogram(&[5.0], 4).unwrap(),
            vec![HistogramBucket {
                start: 5.0,
                end: 5.0,
                count: 1,
            }]
        );
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(min(&[]), Err(DistributionError::EmptyInput));
        assert_eq!(max(&[]), Err(DistributionError::EmptyInput));
        assert_eq!(range(&[]), Err(DistributionError::EmptyInput));
        assert_eq!(frequency_counts(&[]), Err(DistributionError::EmptyInput));
        assert_eq!(histogram(&[], 3), Err(DistributionError::EmptyInput));
        assert_eq!(
            histogram(&[1.0, 2.0], 0),
            Err(DistributionError::InvalidBucketCount)
        );
    }
}
