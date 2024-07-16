/// https://leetcode.com/problems/binary-search/
/// Easy - [array, binary search]
/// Given an array of integers nums which is sorted in ascending order, and an integer target,
/// write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
/// You must write an algorithm with O(log n) runtime complexity.
use std::cmp::Ordering;
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Initialize two pointer for the binary search.
        // Type casting is required if middle becomes 0 and we subtract 1 to get right.
        let mut left = 0i32;
        let mut right = (nums.len() - 1) as i32;
        while left <= right {
            let middle = left + (right - left) / 2;
            match nums[middle as usize].cmp(&target) {
                // We have found the target.
                Ordering::Equal => return middle,
                // The number is bigger than the target, so we need to decrease the upper bound.
                Ordering::Greater => right = middle - 1,
                // The number is smaller than the target, so we need to increase the lower bound.
                Ordering::Less => left = middle + 1,
            }
        }
        // The target is not in the list.
        -1
    }

    pub fn search_idiomatic(nums: Vec<i32>, target: i32) -> i32 {
        nums.iter()
            .enumerate()
            .find(|(_, &x)| x == target)
            .map_or(-1, |(i, _)| i as i32)
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0704() {
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
