//! https://leetcode.com/problems/subsets/
//! Medium - [array, backtracking, bit-manipulation]
//!
//! Given an integer array nums of unique elements, return all possible subsets (the power set).
//! The solution set must not contain duplicate subsets. Return the solution in any order.
//!
//! Example 1:
//! Input: nums = [1,2,3]
//! Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//! Example 2:
//! Input: nums = [0]
//! Output: [[],[0]]
//!
//! Constraints:
//! - `1 <= nums.length <= 10`
//! - `-10 <= nums[i] <= 10`
//! - All the numbers ofnums are unique.

pub struct Solution {}

impl Solution {
    /// Recursively generate the power set of nums in:
    /// - time complexity: `O(n 2^n)` because there are `n` starting points
    ///   and every number can either be in the set or not (`2^n`).
    /// - space complexity: `O(n)`, as the working array contains at most `n`
    ///   elements. The output array (which grows at `O(n 2^n)` is not counted.
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut solutions = vec![];
        Solution::helper(&nums, 0, &mut current, &mut solutions);
        Vec::from_iter(solutions)
    }

    fn helper(nums: &[i32], start: usize, current: &mut Vec<i32>, solutions: &mut Vec<Vec<i32>>) {
        // Do not add the current number.
        solutions.push(current.to_vec());
        for end in start..nums.len() {
            // Add the current number.
            current.push(nums[end]);
            Solution::helper(nums, end + 1, current, solutions);
            current.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0078() {
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
        let mut solution = Solution::subsets(vec![1, 2, 3]);
        solution.sort();
        assert_eq!(
            solution,
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ]
        );
    }
}
