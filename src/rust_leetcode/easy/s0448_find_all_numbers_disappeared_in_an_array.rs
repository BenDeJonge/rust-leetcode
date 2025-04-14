//! https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
//! Easy - [array, hash-table]
//!
//! Given an array nums of n integers where nums[i] is in the range [1, n],
//! return an array of all the integers in the range [1, n]
//! that do not appear in nums.
//!
//! Example 1:
//! Input: `nums = [4,3,2,7,8,2,3,1]`
//! Output: `[5,6]`
//! Example 2:
//! Input: `nums = [1,1]`
//! Output: `[2]`
//!
//! Constraints:
//! - `n == nums.length`
//! - `1 <= n <= 10^5`
//! - `1 <= nums[i] <= n`
//!
//! Follow up: Could you do it without extra space and in O(n) runtime?
//! You may assume the returned list does not count as extra space.

pub struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut is_absent = vec![true; nums.len()];
        nums.into_iter()
            .for_each(|num| is_absent[(num - 1) as usize] = false);
        is_absent
            .iter()
            .enumerate()
            .filter(|(_, present)| **present)
            .map(|(i, _)| (i + 1) as i32)
            .collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0448() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
        assert_eq!(
            Solution::find_disappeared_numbers(<Vec<i32>>::new()),
            <Vec<i32>>::new()
        );
        assert_eq!(
            Solution::find_disappeared_numbers(vec![1, 2, 3, 4]),
            <Vec<i32>>::new()
        );
        assert_eq!(
            Solution::find_disappeared_numbers(vec![1]),
            <Vec<i32>>::new()
        );
        assert_eq!(
            Solution::find_disappeared_numbers(vec![1, 1, 1, 1, 1]),
            vec![2, 3, 4, 5]
        );
    }
}
