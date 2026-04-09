//! https://leetcode.com/problems/subarray-sum-equals-k/
//! Medium - [array, hash-table, prefix-sum]
//!
//! Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
//! A subarray is a contiguous non-empty sequence of elements within an array.
//!
//! Example 1:
//! Input: nums = [1,1,1], k = 2
//! Output: 2
//! Example 2:
//! Input: nums = [1,2,3], k = 3
//! Output: 2
//!
//! Constraints:
//! - 1 <= nums.length <= 2 * 104
//! - -1000 <= nums[i] <= 1000
//! - -107 <= k <= 107

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarray_sum_ref(&nums, k) as i32
    }

    /// Solve in O(N) time and O(N) space.
    ///
    /// We compute sums of contiguous subarrays [:j].
    /// For each sum, we can write: S(:j) = S(:i) + S(i:j).
    /// If a specific subarray equals the target:
    /// S(:j) = S(:i) + k <=> S(:i) = S(:j) - k
    ///
    /// We can store all previous results in a HashMap.
    pub fn subarray_sum_ref(nums: &[i32], k: i32) -> usize {
        let mut counter = 0;
        let mut sums = HashMap::<i32, usize>::with_capacity(nums.len());
        // Already insert S(:0) in case another S(i:j) = k exactly.
        sums.insert(0, 1);
        nums.iter().fold(0, |mut acc, &i| {
            acc += i;
            if let Some(previous) = sums.get(&(acc - k)) {
                counter += previous;
            }
            sums.entry(acc)
                .and_modify(|counts| *counts += 1)
                .or_insert(1);
            acc
        });
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0560() {
        // [1, 1] [1, 1]
        assert_eq!(Solution::subarray_sum_ref(&[1, 1, 1], 2), 2);
        // [1, 2] [3]
        assert_eq!(Solution::subarray_sum_ref(&[1, 2, 3], 3), 2);
        // [1, 2] [1, 2, 0] [0, 3] [3]
        assert_eq!(Solution::subarray_sum_ref(&[1, 2, 0, 3], 3), 4);

        let one_to_ten = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // [3, 4, 5, 6, 7]
        assert_eq!(Solution::subarray_sum_ref(one_to_ten, 25), 1);
        // [2, 3, 4, 5, 6, 7] [8, 9, 10]
        assert_eq!(Solution::subarray_sum_ref(one_to_ten, 27), 2);
    }
}
