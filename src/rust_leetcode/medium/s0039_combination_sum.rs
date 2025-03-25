//! https://leetcode.com/problems/combination-sum/
//! Medium - [array, backtracking]
//! Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
//! The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
//! The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.
//!
//! Example 1:
//! Input: candidates = [2,3,6,7], target = 7
//! Output: [[2,2,3],[7]]
//! Explanation:
//! 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
//! 7 is a candidate, and 7 = 7.
//! These are the only two combinations.
//! Example 2:
//! Input: candidates = [2,3,5], target = 8
//! Output: [[2,2,2,2],[2,3,3],[3,5]]
//! Example 3:
//! Input: candidates = [2], target = 1
//! Output: []
//!
//! Constraints:
//! - `1 <= candidates.length <= 30`
//! - `2 <= candidates[i] <= 40`
//! - All elements of candidates are distinct.
//! - `1 <= target <= 40`

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::<Vec<i32>>::new();
        let mut current = <Vec<i32>>::new();
        // Avoid external side effects by sorting a local copy.
        let mut candidates_sorted = candidates.clone();
        candidates_sorted.sort();
        Self::combination_sum_rec(&candidates_sorted, target, 0, &mut current, &mut results);
        results
    }

    fn combination_sum_rec(
        candidates_sorted: &[i32],
        target: i32,
        i: usize,
        current: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        // Assumes candidates is sorted, which has 2 advantages:
        // - duplicates can be avoided by only considering numbers equal or
        //   larger than the current for the next addition.
        // - enables early returns as soon as the current number is too large.
        for j in i..candidates_sorted.len() {
            let candidate = candidates_sorted[j];
            match (current.iter().sum::<i32>() + candidate).cmp(&target) {
                cmp::Ordering::Less => {
                    current.push(candidate);
                }
                cmp::Ordering::Equal => {
                    current.push(candidate);
                    results.push(current.clone());
                }
                cmp::Ordering::Greater => break,
            }
            // Continue from the last accepted number.
            Self::combination_sum_rec(candidates_sorted, target, j, current, results);
        }
        // Backtracking to try other numbers.
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0039() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::combination_sum(vec![8, 7, 4, 3], 11),
            vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]]
        );
    }
}
