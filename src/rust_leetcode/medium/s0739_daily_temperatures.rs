//! https://leetcode.com/problems/daily-temperatures/
//! Medium - [array, stack, monotonic-stack]
//!
//! Given an array of integers temperatures represents the daily temperatures,
//! return an array answer such that answer[i] is the number of days you have
//! to wait after the ith day to get a warmer temperature.
//! If there is no future day for which this is possible, keep answer[i] == 0 instead.
//!
//! Example 1:
//! Input: temperatures = [73,74,75,71,69,72,76,73]
//! Output: [1,1,4,2,1,1,0,0]
//! Example 2:
//! Input: temperatures = [30,40,50,60]
//! Output: [1,1,1,0]
//! Example 3:
//! Input: temperatures = [30,60,90]
//! Output: [1,1,0]
//!
//! Constraints:
//! - 1 <=temperatures.length <= 10**5
//! - 30 <=temperatures[i] <= 100

pub struct Solution {}

impl Solution {
    /// Use a monotonic stack to track the temperatures and indices.
    /// Ignore all values that are not strictly larger than the current.
    /// If the stack has been exhausted, there is no larger temperature.
    /// If not, use the indices to compute the distance to the larger one.
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut distances: Vec<i32> = Vec::with_capacity(temperatures.len());
        let mut stack: Vec<(usize, i32)> = vec![];

        for (i, current) in temperatures.iter().rev().enumerate() {
            while stack.pop_if(|(_, next)| *next <= *current).is_some() {
                continue;
            }
            let count = stack.last().map(|last| i - last.0).unwrap_or(0);
            stack.push((i, *current));
            distances.push(count as i32);
        }
        distances.reverse();
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0739() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            [1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            [1, 1, 1, 0]
        );
        assert_eq!(Solution::daily_temperatures(vec![30, 60, 90]), [1, 1, 0]);
    }
}
