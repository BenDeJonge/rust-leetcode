//! https://leetcode.com/problems/first-missing-positive/
//! Hard - [array, hash-table]
//!
//! Given an unsorted integer array nums.
//! Return the smallest positive integer that is not present in nums.
//! You must implement an algorithm that:
//! - runs in `O(n)` time; and
//! - uses `O(1)` auxiliary space.
//!
//! Example 1:
//! Input: nums = [1,2,0]
//! Output: 3
//! Explanation: The numbers in the range [1,2] are all in the array.
//! Example 2:
//! Input: nums = [3,4,-1,1]
//! Output: 2
//! Explanation: 1 is in the array but 2 is missing.
//! Example 3:
//! Input: nums = [7,8,9,11,12]
//! Output: 1
//! Explanation: The smallest positive integer 1 is missing.
//!
//! Constraints:
//! - `1 <= nums.length <= 10^5`
//! - `-2^31 <= nums[i] <= 2^31 - 1`

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    /// An efficient solution analogous to a [cycle sort](
    /// https://en.wikipedia.org/wiki/Cycle_sort).
    /// - Time complexity: `O_t(n)`
    /// - Space complexity: `O_s(1)`
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let out_of_bounds = n + 1;

        // A range of length n can at most contain all numbers from 1 to n,
        // if there are no duplicates or numbers less than 1.
        // So, the answer must then live in the range 1..(n + 1).
        for num in nums.iter_mut() {
            if !(1i32..=n).contains(num) {
                *num = out_of_bounds;
            }
        }

        // Every number in this list is in the range 1..n.
        // For every number, encode the fact it is present inside the index.
        // We will make the value at its (zero-based, hence -1) index stand
        // out by making it negative.
        // Double negatives, which would toggle an index back on,
        // are avoided by the if clause.
        for i in 0..nums.len() {
            let num = nums[i].abs();
            if num == out_of_bounds {
                continue;
            }
            let idx = (num - 1) as usize;
            if nums[idx] > 0 {
                nums[idx] *= -1;
            }
        }

        // Every occuring number is now encoded by a negative value at the index
        // corresponding to its value - 1.
        // To find the first number that is missing, simply look for the first
        // non-negative index.
        for (i, num) in nums.into_iter().enumerate() {
            if num >= 0 {
                return (i + 1) as i32;
            }
        }
        // If no non-negative index was found, the array had exactly all numbers
        // from 1..n. So, the first missing number is the next one.
        n + 1
    }

    /// An efficient solution that tracks the previously seen numbers
    /// as indices in an auxiliary boolean array.
    /// - Time complexity: `O_t(n)`
    /// - Space complexity: `O_s(n)`
    pub fn first_missing_positive_boolean_array(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; nums.len() + 1];
        let accepted = 1i32..=(nums.len() as i32);

        for num in nums {
            if accepted.contains(&num) {
                seen[num as usize] = true;
            }
        }

        for (i, s) in seen.iter().enumerate() {
            if !s {
                return i as i32 + 1;
            }
        }
        (seen.len() + 1) as i32
    }

    /// A naive solution that drains the numbers into a set and
    /// iterates over the possible number space.
    /// - Time complexity: `O_t(n log(n))`
    /// - Space complexity: `O_s(n)`
    pub fn first_missing_positive_naive(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let set: HashSet<i32> = HashSet::from_iter(nums);
        for i in 1i32..=(len + 1) {
            if !set.contains(&i) {
                return i;
            }
        }
        unreachable!("loops over entire length of set")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0041() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![2, 2, 2]), 1);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
