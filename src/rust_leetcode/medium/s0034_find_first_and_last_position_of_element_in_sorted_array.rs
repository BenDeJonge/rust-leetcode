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
    /// An ideomatic solution using standard functions.
    pub fn search_range_ideomatic<T: Ord>(nums: &[T], target: T) -> Option<[usize; 2]> {
        nums.binary_search(&target)
            .map(|i| {
                [
                    nums[..i].partition_point(|val| val < &target),
                    i + nums[i..].partition_point(|val| val <= &target) - 1,
                ]
            })
            .ok()
    }

    /// Use a manual binary search to identify the ranges within which
    /// to search for the upper and lower bounds.
    /// - Time complexity: `O(log(n)) + 2 * O(log(n / 2)) = O(log(n))`
    /// - Space complexity: `O(1)`  
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::search_range_manual(&nums, target)
            .map(|arr| vec![arr[0] as i32, arr[1] as i32])
            .unwrap_or(vec![-1, -1])
    }

    // An manual version that implements binary search and partition point.
    fn search_range_manual<T: Ord>(nums: &[T], target: T) -> Option<[usize; 2]> {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;
        let mut i = None;
        let mut i_min = left;
        let mut i_max = right;

        while left <= right {
            let middle = (left + right) / 2;
            match nums[middle as usize].cmp(&target) {
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
            let i_usize = i as usize;
            let i_min_usize = i_min as usize;
            let i_max_usize = i_max as usize;
            Some([
                i_min_usize
                    + (Solution::get_bound(&nums[i_min_usize..=i_usize], &target, Bound::Left)),
                i_usize
                    + (Solution::get_bound(&nums[i_usize..=i_max_usize], &target, Bound::Right)),
            ])
        } else {
            None
        }
    }

    fn get_bound<T: Ord>(nums: &[T], target: &T, bound: Bound) -> usize {
        if nums.is_empty() {
            return 0;
        }
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left < right {
            let mut middle = (left + right) / 2;
            match bound {
                Bound::Left => match nums[middle].cmp(target) {
                    Ordering::Less => left = middle + 1,
                    Ordering::Equal | Ordering::Greater => right = middle,
                },
                Bound::Right => {
                    middle += 1;
                    match nums[middle].cmp(target) {
                        Ordering::Less | Ordering::Equal => left = middle,
                        Ordering::Greater => right = middle - 1,
                    }
                }
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
            (vec![5, 7, 7, 8, 8, 10], 8, Some([3, 4])),
            (vec![7, 7, 8, 8, 9, 10], 7, Some([0, 1])),
            (vec![5, 7, 7, 8, 8, 10], 6, None),
            (vec![2, 2], 2, Some([0, 1])),
            (vec![], 0, None),
            (vec![1], 0, None),
        ] {
            helper(&nums, target, expected);
        }
    }

    fn helper(nums: &[i32], target: i32, expected: Option<[usize; 2]>) {
        assert_eq!(Solution::search_range_manual(nums, target), expected);
        assert_eq!(Solution::search_range_ideomatic(nums, target), expected);
    }
}
