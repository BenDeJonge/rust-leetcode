//! https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
//! Medium - [array, binary-search]
//!
//! Given an array of integers nums sorted in non-decreasing order,
//! find the starting and ending position of a given target value.
//! If target is not found in the array, return `[-1, -1]`.
//! You mustwrite an algorithm with `O(log n)` runtime complexity.
//!
//! Example 1:
//! Input: nums = [5,7,7,8,8,10], target = 8
//! Output: [3,4]
//! Example 2:
//! Input: nums = [5,7,7,8,8,10], target = 6
//! Output: [-1,-1]
//! Example 3:
//! Input: nums = [], target = 0
//! Output: [-1,-1]
//!
//! Constraints:
//! - `0 <= nums.length <= 10^5`
//! - `-10^9<= nums[i]<= 10^9`
//! - nums is a non-decreasing array.
//! - `-10^9<= target<= 10^9`

use std::cmp::Ordering;

enum Bound {
    Left,
    Right,
}

pub struct Solution {}

impl Solution {
    /// Use a manual binary search to identify the ranges within which
    /// to search for the upper and lower bounds.
    /// - Time complexity: `O(log(n)) + 2 * O(log(n / 2)) = O(log(n))`
    /// - Space complexity: `O(1)`
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::search_range_ref(&nums, target).to_vec()
    }

    // Leetcode enforces poor Rust typing.
    fn search_range_ref(nums: &[i32], target: i32) -> [i32; 2] {
        if nums.is_empty() {
            return [-1, -1];
        }

        let mut left = 0usize;
        let mut right = nums.len() - 1;
        let mut i = None;
        let mut i_min = left;
        let mut i_max = right;

        while left <= right {
            let middle = left + (right - left) / 2;
            match nums[middle].cmp(&target) {
                Ordering::Less => {
                    left = middle + 1;
                    i_min = i_min.max(middle);
                }
                Ordering::Equal => {
                    i = Some(middle);
                    break;
                }
                Ordering::Greater => {
                    right = middle - 1;
                    i_max = i_max.min(middle);
                }
            }
        }

        if let Some(i) = i {
            [
                (i_min + nums[i_min..i].partition_point(|&num| num < target)) as i32,
                (i + nums[i..i_max].partition_point(|&num| num <= target) - 1) as i32,
            ]
        } else {
            [-1, -1]
        }
    }

    fn search_range_manual(nums: &[i32], target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut left = 0usize;
        let mut right = nums.len() - 1;
        let mut i = None;
        let mut i_min = left;
        let mut i_max = right;

        while left <= right {
            let middle = left + (right - left) / 2;
            match nums[middle].cmp(&target) {
                Ordering::Less => {
                    left = middle + 1;
                    i_min = i_min.max(middle);
                }
                Ordering::Equal => {
                    i = Some(middle);
                    break;
                }
                Ordering::Greater => {
                    right = middle - 1;
                    i_max = i_max.min(middle);
                }
            }
        }

        if let Some(i) = i {
            vec![
                (i_min + Solution::get_bound(&nums[i_min..i], target, Bound::Left)) as i32,
                (i + Solution::get_bound(&nums[i..i_max], target, Bound::Right)) as i32,
            ]
        } else {
            vec![-1, -1]
        }
    }

    fn get_bound(nums: &[i32], target: i32, bound: Bound) -> usize {
        if nums.is_empty() {
            return 0;
        }
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left < right {
            let middle = left + (right - left) / 2;
            match bound {
                Bound::Left => match nums[middle].cmp(&target) {
                    Ordering::Less => left = middle + 1,
                    Ordering::Equal | Ordering::Greater => right = middle - 1,
                },
                Bound::Right => match nums[middle].cmp(&target) {
                    Ordering::Less | Ordering::Equal => left = middle + 1,
                    Ordering::Greater => right = middle - 1,
                },
            }
        }
        match bound {
            Bound::Left => left,
            Bound::Right => right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0034() {
        for (nums, target, expected) in [
            // (vec![5, 7, 7, 8, 8, 10], 8, [3, 4]),
            // (vec![7, 7, 8, 8, 9, 10], 7, [0, 1]),
            // (vec![5, 7, 7, 8, 8, 10], 6, [-1, -1]),
            // (vec![], 0, [-1, -1]),
            (vec![1], 0, [-1, -1]),
        ] {
            helper(&nums, target, expected);
        }
    }

    fn helper(nums: &[i32], target: i32, expected: [i32; 2]) {
        assert_eq!(Solution::search_range_ref(nums, target), expected);
        assert_eq!(Solution::search_range_manual(nums, target), expected);
    }
}
