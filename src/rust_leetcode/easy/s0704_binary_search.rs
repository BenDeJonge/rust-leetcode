//! <https://leetcode.com/problems/binary-search/>
//! Easy - [array, binary search]
//! Given an array of integers nums which is sorted in ascending order, and an integer target,
//! write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
//! You must write an algorithm with O(log n) runtime complexity.

use std::cmp::Ordering;
pub struct Solution {}

impl Solution {
    pub fn search(nums: &[i32], target: i32) -> Option<usize> {
        // Initialize two pointer for the binary search.
        // Type casting is required if middle becomes 0 and we subtract 1 to get right.
        let mut left = 0;
        let mut right = Some(nums.len() - 1);
        while let Some(r) = right
            && left <= r
        {
            let middle = left.midpoint(r);
            match nums[middle].cmp(&target) {
                // We have found the target.
                Ordering::Equal => return Some(middle),
                // The number is bigger than the target, so we need to decrease the upper bound.
                Ordering::Greater => right = middle.checked_sub(1),
                // The number is smaller than the target, so we need to increase the lower bound.
                Ordering::Less => left = middle + 1,
            }
        }
        // The target is not in the list.
        None
    }

    pub fn search_idiomatic(nums: &[i32], target: i32) -> Option<usize> {
        nums.binary_search(&target).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0704() {
        assert_eq!(Solution::search(&[5], -5), None);
        assert_eq!(Solution::search(&[4], 5), None);
        assert_eq!(Solution::search(&[-1, 0, 3, 5, 9, 12], 9), Some(4));
        assert_eq!(Solution::search(&[-1, 0, 3, 5, 9, 12], 2), None);
    }
}
