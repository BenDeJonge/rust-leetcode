//! <https://leetcode.com/problems/longest-consecutive-sequence/>
//! Medium - [array, hash-table, union-find]
//!
//! Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
//! You must write an algorithm that runs inO(n)time.
//!
//! Example 1:
//! Input: nums = [100,4,200,1,3,2]
//! Output: 4
//! Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
//! Example 2:
//! Input: nums = [0,3,7,2,5,8,4,6,0,1]
//! Output: 9
//! Example 3:
//! Input: nums = [1,0,1,2]
//! Output: 3
//!
//! Constraints:
//! - 0 <= nums.length <= 10**5
//! - -10**9 <= nums[i] <= 10**9

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    /// Solve in O(n), with n the length of nums.
    ///
    /// The longest sequence stretches from i to j. This means that i is the
    /// lowest number in this sequence, and does therefore not have any lower
    /// neighbor (i - 1). So, consecutive upper neighbors (i + 1) are counted
    /// for numbers that do not have lower neighbors.
    fn longest_consecutive_inner(nums: &[i32]) -> usize {
        let unique = nums.iter().collect::<HashSet<_>>();
        unique
            .iter()
            .map(|&&num| {
                unique
                    .get(&(num - 1))
                    .map(|_| 1)
                    .unwrap_or_else(|| (num..).take_while(|n| unique.contains(n)).count())
            })
            .max()
            .unwrap_or_default()
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        Self::longest_consecutive_inner(&nums)
            .try_into()
            .expect("does not fit into i32")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0128() {
        assert_eq!(Solution::longest_consecutive_inner(&[]), 0);
        assert_eq!(Solution::longest_consecutive_inner(&[1]), 1);
        assert_eq!(Solution::longest_consecutive_inner(&[1, 1]), 1);
        assert_eq!(Solution::longest_consecutive_inner(&[1, 3, 5, 7]), 1);
        assert_eq!(Solution::longest_consecutive_inner(&[1, 2, 3, 4]), 4);
        assert_eq!(Solution::longest_consecutive_inner(&[1, 0, 1, 2]), 3);
        assert_eq!(
            Solution::longest_consecutive_inner(&[10, 4, 20, 1, 3, 2]),
            4
        );
        assert_eq!(
            Solution::longest_consecutive_inner(&[0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
