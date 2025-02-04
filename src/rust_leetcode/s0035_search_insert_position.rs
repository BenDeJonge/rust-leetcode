//! https://leetcode.com/problems/search-insert-position/
//! Easy - [array, binary search]
//! Given a sorted array of distinct integers and a target value, return the index if the target is found.
//! If not, return the index where it would be if it were inserted in order.
//! You must write an algorithm with O(log n) runtime complexity.

use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Initialize two pointer for the binary search.
        let mut left = 0i32;
        let mut right = (nums.len() - 1) as i32;
        // Start the search.
        while left <= right {
            // Compute the pivot point.
            let middle = left + (right - left) / 2;
            match nums[middle as usize].cmp(&target) {
                // We found the target.
                Ordering::Equal => return middle,
                // The number is bigger than the target, so we need to lower the upper bound.
                Ordering::Greater => right = middle - 1,
                // And conversely so if the number is smaller than the target.
                Ordering::Less => left = middle + 1,
            }
        }
        // The target was not in the list.
        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0035() {
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 1), 0);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 3), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 6), 3);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
