//! <https://leetcode.com/problems/house-robber/>
//! Medium - [array, dynamic-programming]
//!
//! You are a professional robber planning to rob houses along a street. Each house has a certain amount of money
//! stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems
//! connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
//! Given an integer array nums representing the amount of money of each house, return the maximum amount of money you
//! can rob tonight without alerting the police.
//!
//! Example 1:
//! Input: nums = [1,2,3,1]
//! Output: 4
//! Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//! Total amount you can rob = 1 + 3 = 4.
//! Example 2:
//! Input: nums = [2,7,9,3,1]
//! Output: 12
//! Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
//! Total amount you can rob = 2 + 9 + 1 = 12.
//!
//! Constraints:
//! - 1 <= nums.length <= 100
//! - 0 <= nums[i] <= 400

pub struct Solution {}

impl Solution {
    /// - Time: `O(n)`
    /// - Space: `O(1)`
    ///
    /// A dynamic programming approach where the short (`i + 2`) and far (`i + 3`)
    /// neighbors are tracked in an array. There is no need to explore beyond these,
    /// as any further neighbors will already be counted in these ones.
    /// I.e., for any `j` the neighbor `i + j` is counted by:
    /// - the short neighbor for all `j = 2k`
    /// - the far neighbor for all `j = 2k + 1`.
    pub fn rob(nums: &[i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut short = 0;
        let mut far = 0;

        for num in nums {
            let tmp = short;
            short = short.max(far + num);
            far = tmp;
        }
        short
    }

    /// - Time: `O(n)`
    /// - Space: `O(n)`
    ///
    /// Store all scores in an array. Only the last 2 elements are used so this can be optimized away.
    pub fn rob_arr(nums: &[i32]) -> i32 {
        match nums.len() {
            0 => return 0,
            1 => return nums[0],
            _ => {}
        }

        let mut scores = vec![0; nums.len()];
        for i in (0..nums.len()).rev() {
            let j = scores.get(i + 2).unwrap_or(&0);
            let k = scores.get(i + 3).unwrap_or(&0);
            let score = j.max(k);
            scores[i] += score + nums[i];
        }
        *scores[0..2].iter().max().unwrap()
    }

    fn naive_recursive(nums: &[i32]) -> i32 {
        let mut best = 0;
        // Explore all starting houses.
        for i in 0..nums.len() {
            Self::naive_recursive_helper(nums, i, 0, &mut best);
        }
        best
    }

    fn naive_recursive_helper(nums: &[i32], i: usize, score: i32, best: &mut i32) {
        let new_score = score + nums[i];
        let next = i + 2..nums.len();
        if next.is_empty() {
            *best = new_score.max(*best);
        }
        for j in next {
            Self::naive_recursive_helper(nums, j, new_score, best);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0198_1() {
        assert_eq!(Solution::rob(&[1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_0198_2() {
        assert_eq!(Solution::rob(&[2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn zeros() {
        assert_eq!(Solution::rob(&[0, 0, 0, 0]), 0);
    }

    #[test]
    fn first() {
        assert_eq!(Solution::rob(&[10, 0, 0, 0]), 10);
    }

    #[test]
    fn second() {
        assert_eq!(Solution::rob(&[0, 10, 0, 0]), 10);
    }

    #[test]
    fn last() {
        assert_eq!(Solution::rob(&[0, 0, 0, 10]), 10);
    }

    #[test]
    fn empty() {
        assert_eq!(Solution::rob(&[]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(Solution::rob(&[1]), 1);
    }

    #[test]
    fn two() {
        assert_eq!(Solution::rob(&[1, 2]), 2);
    }

    #[test]
    fn three() {
        assert_eq!(Solution::rob(&[1, 2, 3]), 4);
    }
}
