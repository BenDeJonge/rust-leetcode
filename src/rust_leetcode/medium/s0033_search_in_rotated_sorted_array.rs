//! https://leetcode.com/problems/search-in-rotated-sorted-array/
//! Medium - [array, binary-search]
//!
//! There is an integer array nums sorted in strictly ascending order.
//! Prior to being passed to your function, nums is possibly left rotated at an
//! unknown index `k` (`1 <= k < nums.length`) such that the resulting array is
//! `[nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]`.
//! For example, `[0,1,2,4,5,6,7]` -> `[4,5,6,7,0,1,2]` (rotated at index 3).
//!
//! Given the array nums after the possible rotation and an integer target,
//! return the index of target if it is in `nums`, or `-1` if it is not.
//! You must write an algorithm with `O(log n)` runtime complexity.
//!
//! Example 1:
//! Input: nums = [4,5,6,7,0,1,2], target = 0
//! Output: 4
//! Example 2:
//! Input: nums = [4,5,6,7,0,1,2], target = 3
//! Output: -1
//! Example 3:
//! Input: nums = [1], target = 0
//! Output: -1
//!
//! Constraints:
//! - `1 <= nums.length <= 5000`
//! - `-10^4 <= nums[i] <= 10^4`
//! - All values of nums are unique.
//! - `nums` is an ascending array that is possibly rotated.
//! - `-10^4 <= target <= 10^4`

use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    /// Compute the array shift to offset the midpoint of a binary search.
    /// - Time complexity: `O(n) + O(log(n)) = O(n)`
    /// - Space complexity: `O(1)`
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let i_min = nums
            .windows(2)
            .enumerate()
            .find_map(|(i, window)| {
                if window[0] > window[1] {
                    Some(i as i32 + 1)
                } else {
                    None
                }
            })
            .unwrap_or(0);
        let len = nums.len() as i32;
        let shift = (len - i_min) % len;

        let mut left = 0i32;
        let mut right = len - 1;
        while left <= right {
            let middle = left + (right - left) / 2;
            // https://github.com/rust-lang/rust/issues/13909
            // -2 % 7 == -2
            // (-2).rem_euclid(7) == 5
            let i_shifted = (middle - shift).rem_euclid(len);
            match nums[i_shifted as usize].cmp(&target) {
                Ordering::Less => left = middle + 1,
                Ordering::Equal => return i_shifted,
                Ordering::Greater => right = middle - 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0033() {
        assert_eq!(Solution::search(vec![0, 1, 2, 4, 5, 6, 7], 2), 2);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 6), 2);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
