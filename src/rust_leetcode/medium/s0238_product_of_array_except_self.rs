//! https://leetcode.com/problems/product-of-array-except-self/
//! Medium - [array, prefix-sum]
//!
//! Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//! The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//! You must write an algorithm that runs inO(n)time and without using the division operation.
//!
//! Example 1:
//! Input: nums = [1,2,3,4]
//! Output: [24,12,8,6]
//! Example 2:
//! Input: nums = [-1,1,0,-3,3]
//! Output: [0,0,9,0,0]
//!
//! Constraints:
//! - 2 <= nums.length <= 10**5
//! - -30 <= nums[i] <= 30
//! - The input is generated such that answer[i] is guaranteed to fit in a 32-bit integer.
//!
//! Follow up:Can you solve the problem in O(1) extra space complexity?
//! (The output array does not count as extra space for space complexity analysis.)

use std::iter;

pub struct Solution {}

impl Solution {
    /// Solve in O(n**2) time and O(1) space.
    ///
    /// For every number, go through the all other numbers.
    fn product_except_self_naive(nums: &[i32]) -> Vec<i32> {
        (0..nums.len())
            .map(|i| nums[..i].iter().chain(nums[i + 1..].iter()).product())
            .collect()
    }

    /// Solve O(n) time and O(1) space.
    ///
    /// Use dynamic programming to reuse the previous calculations.
    fn product_except_self_dp(nums: &[i32]) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        // [1, 2, 3] -> [1, 2, 6]
        let cum_prod = |state: &mut i32, num: &i32| {
            *state *= num;
            Some(*state)
        };

        // Prefix 1 + cum_prod of all but last term: [1, 2, 3, 4] -> [1, 1, 2, 6]
        let left = iter::once(1).chain(nums.iter().take(nums.len() - 1).scan(1, cum_prod));
        // Cum_prod of all but first term + suffix 1: [1, 2, 3, 4] -> [24, 12, 4, 1]
        let mut right: Vec<i32> = iter::once(1)
            .chain(nums.iter().skip(1).rev().scan(1, cum_prod))
            // We need to collect because scan is not double-ended.
            // This makes sense as cum_prod has to be computed in order.
            .collect();
        // The prefix and suffixes of 1 are there to mimic
        // the cumulative product coming from the other end.
        // [1, 2, 3, 4] -> [1, 1, 2, 6] * [24, 12, 4, 1]
        // At index 0: 2 * 3 * 4.
        // At index 3: 1 * 2 * 3

        // Modify right in-place to avoid allocating another array.
        for (r, l) in right.iter_mut().rev().zip(left) {
            *r *= l;
        }
        right.reverse();
        right
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        Self::product_except_self_dp(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0238() {
        assert_eq!(
            Solution::product_except_self_dp(&[1, 2, 3, 4]),
            [24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self_dp(&[-1, 1, 0, -3, 3]),
            [0, 0, 9, 0, 0]
        );
        assert_eq!(
            Solution::product_except_self_dp(&Vec::<i32>::new()),
            Vec::<i32>::new()
        );
    }
}
