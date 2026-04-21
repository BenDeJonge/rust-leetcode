//! https://leetcode.com/problems/next-greater-element-ii/
//! Medium - [array, stack, monotonic-stack]
//!
//! Given a circular integer array nums (i.e., the next element of nums[nums.length - 1] is nums[0]),
//! return the next greater number for every element in nums.
//! The next greater number of a number x is the first greater number to its
//! traversing-order next in the array, which means you could search circularly
//! to find its next greater number. If it doesn't exist, return -1 for this number.
//!
//! Example 1:
//! Input: nums = [1,2,1]
//! Output: [2,-1,2]
//! Explanation: The first 1's next greater number is 2;
//! The number 2 can't find next greater number.
//! The second 1's next greater number needs to search circularly, which is also 2.
//! Example 2:
//! Input: nums = [1,2,3,4,3]
//! Output: [2,3,4,-1,4]
//!
//! Constraints:
//! - 1 <= nums.length <= 104
//! - -109 <= nums[i] <= 109

pub struct Solution {}

/// Solve in O(2n(nums)).
///
/// Create a monotonically decreasing stack.
/// The current largest number is kept at the top.
/// For every new number, keep popping the stack while the top is smaller.
/// Mimic circularity by doing a second pass.
impl Solution {
    pub fn next_greater_elements<T: Ord + Copy>(nums: &[T]) -> Vec<Option<T>> {
        let mut sol = vec![None; nums.len()];
        let mut stack = Vec::with_capacity(nums.len());

        for i in (0..(nums.len())).cycle().take(nums.len() * 2) {
            while let Some(j) = stack.pop_if(|top| nums[*top] < nums[i]) {
                sol[j] = Some(nums[i]);
            }
            stack.push(i);
        }
        sol
    }

    /// Complying to LeetCodes stupid types.
    pub fn next_greater_elements_lc(nums: Vec<i32>) -> Vec<i32> {
        let mut sol = vec![-1; nums.len()];
        let mut stack = Vec::with_capacity(nums.len());

        for i in (0..(nums.len())).cycle().take(nums.len() * 2) {
            while let Some(j) = stack.pop_if(|top| nums[*top] < nums[i]) {
                sol[j] = nums[i];
            }
            stack.push(i);
        }
        sol
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0503() {
        assert_eq!(
            Solution::next_greater_elements(&[1, 2, 1]),
            [Some(2), None, Some(2)]
        );
        assert_eq!(
            Solution::next_greater_elements(&[1, 2, 3, 4, 3]),
            [Some(2), Some(3), Some(4), None, Some(4)]
        );
        assert_eq!(
            Solution::next_greater_elements(&[3, 8, 4, 1, 2]),
            [Some(8), None, Some(8), Some(2), Some(3)]
        );
        assert_eq!(
            Solution::next_greater_elements(&[1, 1, 1, 2]),
            [Some(2), Some(2), Some(2), None]
        );
        assert_eq!(
            Solution::next_greater_elements(&[1, 2, 2, 2]),
            [Some(2), None, None, None]
        );
        assert_eq!(
            Solution::next_greater_elements(&[2, 1, 1, 1]),
            [None, Some(2), Some(2), Some(2)]
        );
        assert_eq!(
            Solution::next_greater_elements(&[1, 2, 3, 2, 1]),
            [Some(2), Some(3), None, Some(3), Some(2)]
        );
    }
}
