//! https://leetcode.com/problems/jump-game/
//! Medium - [array, dynamic programming, greedy]
//! Given an array of integers, determine if you can reach the final index
//! starting from the first.
//! Every array element represents the current maximal jump size.

use std::cmp;

pub struct Solution {}

impl Solution {
    fn can_jump_recursive(nums: &[usize]) -> bool {
        for i in (0..nums.len()).rev() {
            // Backtracking.
            if !Self::can_jump_recursive_helper(nums, i, nums.len() - 1) {
                continue;
            } else {
                return true;
            }
        }
        false
    }

    fn can_jump_recursive_helper(nums: &[usize], i: usize, target_i: usize) -> bool {
        // Base case: verify if the target can be reached.
        if i == 0 {
            return nums[0] >= target_i;
        }
        // The number can reach the destination.
        if nums[i] + i >= target_i {
            Self::can_jump_recursive_helper(nums, i - 1, i)
        }
        // The number cannot reach the destination, but maybe the previous can.
        else {
            Self::can_jump_recursive_helper(nums, i - 1, target_i)
        }
    }

    /// A simple greedy implementation, that tracks te furthest point reached.
    /// Whenever we try to look at numbers beyond this point, the algorithm exits.
    /// The furthest point is dynamically updated based on the current.
    pub fn can_jump(nums: &[i32]) -> bool {
        let mut furthest: i32 = 0;
        for (i, number) in nums.iter().enumerate() {
            // We have moved beyond the furthest point we can reach.
            // All these numbers don't matter anymore.
            if i as i32 > furthest {
                return false;
            }
            furthest = cmp::max(furthest, i as i32 + *number);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0055() {
        // Index: 0 -> 1 -> 4
        assert!(Solution::can_jump(&[2, 3, 1, 1, 4]));
        assert!(Solution::can_jump(&[2, 5, 0, 0]));
        assert!(Solution::can_jump(&[1, 2, 0, 3, 0, 0, 1]));
        // Index: 0 -> 1 -> 2 -> 3. Stuck at 0, no matter which steps are taken.
        assert!(!Solution::can_jump(&[3, 2, 1, 0, 4]));
    }
}
