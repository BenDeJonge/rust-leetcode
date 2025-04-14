//! https://leetcode.com/problems/find-all-duplicates-in-an-array/
//! Medium - [array, hash-table]
//!
//! Given an integer array nums of length n where all the integers of nums are in the range [1, n] and each integer appears at most twice, return an array of all the integers that appears twice.
//! You must write an algorithm that runs in O(n) time and uses only constant auxiliary space, excluding the space needed to store the output
//!
//! Example 1:
//! Input: nums = [4,3,2,7,8,2,3,1]
//! Output: [2,3]
//! Example 2:
//! Input: nums = [1,1,2]
//! Output: [1]
//! Example 3:
//! Input: nums = [1]
//! Output: []
//!
//! Constraints:
//! - `n == nums.length`
//! - `1 <= n <= 10^5`
//! - `1 <= nums[i] <= n`
//! - Each element in nums appears once or twice.

pub struct Solution {}

impl Solution {
    /// - Time complexity: `O_t = O(n)`.
    /// - Auxiliary space: `O_s = O(1)`.
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        // Worst case, every element appears twice.
        let mut duplicates = Vec::with_capacity(nums.len() / 2);
        for i in 0..nums.len() {
            // The value of the number acts as an index.
            let idx = nums[i].unsigned_abs() as usize - 1;
            // The sign of a number acts as a flag for being a duplicate.
            // Avoid toggling a number off again.
            if nums[idx].is_positive() {
                nums[idx] *= -1
            } else {
                duplicates.push(idx as i32 + 1);
            }
        }
        duplicates
    }

    /// - Time complexity: `O_t = O(n)`.
    /// - Auxiliary space: `O_s = O(n)`.
    pub fn find_duplicates_naive(nums: Vec<i32>) -> Vec<i32> {
        let mut is_duplicate = vec![false; nums.len()];
        nums.iter()
            .for_each(|num| is_duplicate[(num - 1) as usize] = true);
        is_duplicate
            .iter()
            .enumerate()
            .filter(|(_, duplicate)| **duplicate)
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0442() {
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(Solution::find_duplicates(vec![1]), <Vec<i32>>::new());
    }
}
