//! https://leetcode.com/problems/maximum-subarray/
//! Medium - [array, divide-and-conquer, dynamic-programming]
//!
//! Given an integer array nums, find the subarray with the largest sum, and return its sum.
//!
//! Example 1:
//! Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//! Output: 6
//! Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//! Example 2:
//! Input: nums = [1]
//! Output: 1
//! Explanation: The subarray [1] has the largest sum 1.
//! Example 3:
//! Input: nums = [5,4,-1,7,8]
//! Output: 23
//! Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
//!
//! Constraints:
//! -`1 <= nums.length <= 10^5`
//! -`-10^4 <= nums[i] <= 10^4`
//!
//! Follow up: If you have figured out the O(n) solution,
//! try coding another solution using the divide and conquer approach,
//! which is more subtle.
#![allow(const_item_mutation)]

type Memo = Vec<Vec<Option<i32>>>;

enum Operation {
    CropLeft,
    CropRight,
    ExtendLeft,
    ExtendRight,
}

pub struct Solution {}

impl Solution {
    pub fn max_sub_array_dc(nums: Vec<i32>) -> i32 {
        todo!()
    }

    /// The optimal subarray lies either in:
    /// - the left half of `nums`
    /// - the right half of `nums`
    /// - spanning parts of both halves of `nums`
    fn solve_dc(nums: &[i32], left: usize, right: usize) -> i32 {
        if left > right {
            return i32::MIN;
        }
        let mid = (left + right) / 2;
        let mut sum_left = 0;
        let mut sum_right = 0;
        let mut current_sum = 0;

        let mut i = mid - 1;
        while i < left - 1 {
            current_sum += nums[i];
            sum_left = sum_left.max(current_sum);
            i -= 1;
        }
        for i in (mid + 1)..(right + 1) {
            current_sum += nums[i];
            sum_right = sum_right.max(current_sum);
        }
        
        todo!()
    }

    /// A recursive implementation that considers each subarray and maximises.
    /// - Time complexity: `O_t = O(n²)`
    /// - Auxiliary space: `O_s = O(n)`
    fn solve_recursive(nums: &[i32], i: usize, pick_element: bool) -> i32 {
        // The chosen subarray must have at least one element.
        // If nothing has been chosen yet, return a dummy value.
        if i >= nums.len() {
            if pick_element {
                return 0;
            } else {
                return i32::MIN;
            }
        }
        // Sub array must be contiguous. If we pick now, we must pick all subsequent ones.
        if pick_element {
            return *[0, nums[i] + Self::solve_recursive(nums, i + 1, true)]
                .iter()
                .max()
                .unwrap();
        }
        *[
            // Don't choose the current element.
            Self::solve_recursive(nums, i + 1, false),
            // Choose the current element and recurse deeper.
            nums[i] + Self::solve_recursive(nums, i + 1, true),
        ]
        .iter()
        .max()
        .unwrap()
    }

    fn max_sub_array_dp_iterative(nums: Vec<i32>) -> i32 {
        let mut memo = vec![vec![None; nums.len()]; 2];
        memo[0][0] = Some(nums[0]);
        memo[1][0] = Some(nums[0]);
        for (i, num) in nums.iter().enumerate().skip(1) {
            // Pick the current element.
            memo[1][i] = Some(std::cmp::max(
                // ... on its own.
                nums[i],
                // ... or include it in the current subarray. This will be done
                // if the current subarray has a positive sum.
                num + memo[1][i - 1].unwrap_or_default(),
            ));
            // Do not pick the current element.
            memo[0][i] = std::cmp::max(
                // Get to the current element by also skipping the previous.
                memo[0][i - 1],
                // But verify that it is not better to pick it instead.
                memo[1][i],
            );
        }
        memo[0].last().unwrap().unwrap()
    }

    pub fn max_sub_array_dp(nums: Vec<i32>) -> i32 {
        let mut memo = vec![vec![None; nums.len()]; 2];
        Self::solve_dp(&nums, 0, false, &mut memo)
    }

    /// A dynamic programming implementation that tracks the previous solutions.
    /// Each solution has two parameters:
    /// - a boolean `pick_element` that describes if the next pick is required
    ///   to keep the subarray contiguous.
    /// - the current subarray, which can be of length `n`.
    ///   So, we can memoize this as an array of shape `(2, n)`.
    /// - Time complexity: `O_t = O(n)`
    /// - Auxiliary space: `O_s = O(n)`
    fn solve_dp(nums: &[i32], i: usize, pick_element: bool, memo: &mut Memo) -> i32 {
        if i >= nums.len() {
            if pick_element {
                return 0;
            } else {
                return i32::MIN;
            }
        }
        if let Some(previous) = memo[pick_element as usize][i] {
            return previous;
        }
        // Sub array must be contiguous. If we pick now, we must pick all subsequent ones.
        if pick_element {
            return *[0, nums[i] + Self::solve_dp(nums, i + 1, true, memo)]
                .iter()
                .max()
                .unwrap();
        }
        *[
            // Don't choose the current element.
            Self::solve_dp(nums, i + 1, false, memo),
            // Choose the current element and recurse deeper.
            nums[i] + Self::solve_dp(nums, i + 1, true, memo),
        ]
        .iter()
        .max()
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0053() {
        assert_eq!(
            Solution::max_sub_array_dp(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array_dp(vec![1]), 1);
        assert_eq!(Solution::max_sub_array_dp(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array_dp(vec![-1]), -1);
    }
}
