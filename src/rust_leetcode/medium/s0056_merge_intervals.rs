//! https://leetcode.com/problems/merge-intervals/
//! Medium - [array, sorting]
//! Given an array of intervals where `intervals[i] = [start_i, end_i]`,
//! merge all overlapping intervals, and return an array of the non-overlapping
//! intervals that cover all the intervals in the input.
//!
//! Example 1:
//! Input: `intervals = [[1,3],[2,6],[8,10],[15,18]]`
//! Output: `[[1,6],[8,10],[15,18]]`
//! Explanation: Since intervals `[1,3]` and `[2,6]` overlap, merge them into `[1,6]`.
//!
//! Example 2:
//! Input: intervals = `[[1,4],[4,5]]`
//! Output: `[[1,5]]`
//! Explanation: Intervals `[1,4]` and `[4,5]` are considered overlapping.
//!
//! Constraints:
//! - `1 <= intervals.length <= 10^4`
//! - `intervals[i].length == 2`
//! - `0 <= start_i <= end_i <= 10^4`

pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1 {
            return intervals;
        }

        let mut intervals_sorted = intervals.clone();
        intervals_sorted.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut merged = Vec::<Vec<i32>>::new();
        let mut previous = intervals_sorted[0].clone();

        for interval in intervals_sorted.iter().skip(1) {
            // The current interval overlaps with the previous one,
            // so set the previous right boundary to the furthest extented one.
            if previous[1] >= interval[0] {
                previous[1] = previous[1].max(interval[1])
            }
            // There is no overlap and by sorting, future intervals won't
            // have one either. We can get rid of the previous interval and
            // replace it by the current one, for the next iteration.
            else {
                merged.push(previous);
                previous = interval.to_vec();
            }
        }
        merged.push(previous);
        merged
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0056() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 3]]),
            vec![vec![1, 4]]
        );
    }
}
