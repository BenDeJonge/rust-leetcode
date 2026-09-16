//! <https://leetcode.com/problems/statistics-from-a-large-sample/>
//! Medium - [array, math, probability-and-statistics]
//!
//! You are given a large sample of integers in the range [0, 255]. Since the sample is so large,
//! it is represented by an array countwhere count[k] is the number of times that k appears in the sample.
//! Calculate the following statistics:
//! - minimum: The minimum element in the sample.
//! - maximum: The maximum element in the sample.
//! - mean: The average of the sample, calculated as the total sum of all elements divided by the total number of elements.
//! - median:
//!   - If the sample has an odd number of elements, then the median is the middle element once the sample is sorted.
//!   - If the sample has an even number of elements, then the median is the average of the two middle elements once the sample is sorted.
//! - mode: The number that appears the most in the sample. It is guaranteed to be unique.
//!
//! Return the statistics of the sample as an array of floating-point numbers [minimum, maximum, mean, median, mode].
//! Answers within 10**-5 of the actual answer will be accepted.
//!
//! Example 1:
//! Input: count = [0,1,3,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
//! Output: [1.0,3.0,2.375,2.5,3.0]
//! Explanation: The sample represented by count is [1,2,2,2,3,3,3,3].
//! The minimum and maximum are 1 and 3 respectively.
//! The mean is (1+2+2+2+3+3+3+3) / 8 = 19 / 8 = 2.375.
//! Since the size of the sample is even, the median is the average of the two middle elements 2 and 3, which is 2.5.
//! The mode is 3 as it appears the most in the sample.
//! Example 2:
//! Input: count = [0,4,3,2,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
//! Output: [1.0,4.0,2.18182,2.0,1.0]
//! Explanation: The sample represented by count is [1,1,1,1,2,2,2,3,3,4,4].
//! The minimum and maximum are 1 and 4 respectively.
//! The mean is (1+1+1+1+2+2+2+3+3+4+4) / 11 = 24 / 11 = 2.18181818... (for display purposes, the output shows the rounded number 2.18182).
//! Since the size of the sample is odd, the median is the middle element 2.
//! The mode is 1 as it appears the most in the sample.
//!
//! Constraints:
//! - count.length == 256
//! - 0 <= count[i] <= 10**9
//! - 1 <= sum(count) <= 10**9
//! - The mode of the sample that count represents is unique.

use std::cmp::Ordering;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Statistics {
    minimum: f64,
    maximum: f64,
    mean: f64,
    median: f64,
    mode: f64,
}

impl From<Statistics> for Vec<f64> {
    fn from(val: Statistics) -> Self {
        vec![val.minimum, val.maximum, val.mean, val.median, val.mode]
    }
}

impl IntoIterator for Statistics {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<f64>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::<f64>::from(self).into_iter()
    }
}

pub struct Solution {}

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        Self::sample_statistics(&count).unwrap().into()
    }

    /// - Time complexity: O(n)
    /// - Space complexity: O(1)
    ///
    /// Computation of the median requires a second pass after knowing the
    /// total number of elements. The effective runtime is O(2n) = O(n).
    fn sample_statistics(count: &[i32]) -> Option<Statistics> {
        let mut statistics = Statistics::default();
        let mut minimum = false;
        let mut mode = [0, 0];
        let mut mean = [0, 0];

        for (value, &counts) in Self::iter_nonzero(count) {
            let value_f64 = value as f64;
            if !minimum {
                statistics.minimum = value_f64;
                minimum = true;
            }

            statistics.maximum = value_f64;

            if counts > mode[1] {
                mode = [value as i32, counts];
            }

            // Worst case: [1e9; 255] = (255**2 + 255)/2 * 1e9 = 32 385e9 = 3e13.
            // This does not fit in i32 but does fit in u64.
            let counts_u64 = counts as u64;
            mean[0] += counts_u64 * value as u64;
            mean[1] += counts_u64;
        }

        if mean[1] == 0 {
            return None;
        }

        statistics.mean = mean[0] as f64 / mean[1] as f64;
        statistics.mode = mode[0] as f64;
        statistics.median = Self::median(count, mean[1]);

        Some(statistics)
    }

    fn median(count: &[i32], n: u64) -> f64 {
        let half = n.div_ceil(2);
        let mut current = 0;
        let mut median = [None, None];

        for (value, count) in Self::iter_nonzero(count) {
            current += (*count) as u64;
            match current.cmp(&half) {
                Ordering::Less => {}
                _ => {
                    if n.is_multiple_of(2) {
                        if current == half {
                            median[0] = Some(value);
                        } else {
                            match median {
                                [Some(_), None] => median[1] = Some(value),
                                [None, None] => median = [Some(value), Some(value)],
                                _ => unreachable!(),
                            }
                            break;
                        }
                    } else {
                        median = [Some(value), Some(value)];
                        break;
                    }
                }
            }
        }
        (median[0].unwrap() + median[1].unwrap()) as f64 / 2.0
    }

    fn iter_nonzero(count: &[i32]) -> impl Iterator<Item = (usize, &i32)> {
        count.iter().enumerate().filter(|(_, count)| **count > 0)
    }
}

#[cfg(test)]
mod tests {

    use std::ops::Sub;

    use crate::rust_leetcode::medium::s1183_statistics_from_a_large_sample::Statistics;

    use super::Solution;

    const TOLERANCE: f64 = 1e-5;

    fn is_close(stats1: Statistics, stats2: Statistics) -> bool {
        stats1
            .into_iter()
            .zip(stats2)
            .all(|(v1, v2)| v1.sub(v2).abs().le(&TOLERANCE))
    }

    fn helper(count: &[i32], expected: Option<Statistics>) {
        let actual = Solution::sample_statistics(count);
        match [actual, expected] {
            [Some(a), Some(e)] => assert!(is_close(a, e)),
            [None, None] => {}
            _ => panic!("actual and expected are different option"),
        }
    }

    #[test]
    fn test_1183_1() {
        helper(
            &[
                0, 1, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            Some(Statistics {
                minimum: 1.0,
                maximum: 3.0,
                mean: 2.375,
                median: 2.5,
                mode: 3.0,
            }),
        );
    }

    #[test]
    fn test_1183_2() {
        helper(
            &[
                0, 4, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            Some(Statistics {
                minimum: 1.0,
                maximum: 4.0,
                mean: 2.18182,
                median: 2.0,
                mode: 1.0,
            }),
        );
    }

    #[test]
    fn test_1183_3() {
        helper(
            &[
                264, 912, 1416, 1903, 2515, 3080, 3598, 4099, 4757, 5270, 5748, 6451, 7074, 7367,
                7847, 8653, 9318, 9601, 10481, 10787, 11563, 11869, 12278, 12939, 13737, 13909,
                14621, 15264, 15833, 16562, 17135, 17491, 17982, 18731, 19127, 19579, 20524, 20941,
                21347, 21800, 22342, 21567, 21063, 20683, 20204, 19818, 19351, 18971, 18496, 17974,
                17677, 17034, 16701, 16223, 15671, 15167, 14718, 14552, 14061, 13448, 13199, 12539,
                12265, 11912, 10931, 10947, 10516, 10177, 9582, 9102, 8699, 8091, 7864, 7330, 6915,
                6492, 6013, 5513, 5140, 4701, 4111, 3725, 3321, 2947, 2357, 1988, 1535, 1124, 664,
                206, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            Some(Statistics {
                minimum: 0.0,
                maximum: 89.0,
                mean: 42.82683,
                median: 42.0,
                mode: 40.0,
            }),
        );
    }

    #[test]
    fn test_median_even() {
        helper(
            &[
                0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            Some(Statistics {
                minimum: 1.0,
                maximum: 3.0,
                mean: 2.0,
                median: 2.0,
                mode: 1.0,
            }),
        );
    }

    #[test]
    fn test_median_odd() {
        helper(
            &[
                0, 2, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            Some(Statistics {
                minimum: 1.0,
                maximum: 3.0,
                mean: 2.0,
                median: 2.0,
                mode: 1.0,
            }),
        );
    }

    #[test]
    fn test_maximum_values() {
        helper(
            &[1_000_000_000; 256],
            Some(Statistics {
                minimum: 0.0,
                maximum: 255.0,
                mean: 127.5,
                median: 127.5,
                mode: 0.0,
            }),
        );
    }

    #[test]
    fn test_zeros() {
        helper(&[0; 256], None);
    }

    #[test]
    fn test_single_value() {
        let mut count = [0; 256];
        count[1] = 1;
        helper(
            &count,
            Some(Statistics {
                minimum: 1.0,
                maximum: 1.0,
                mean: 1.0,
                median: 1.0,
                mode: 1.0,
            }),
        );
    }
}
