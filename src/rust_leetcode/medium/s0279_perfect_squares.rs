//! <https://leetcode.com/problems/perfect-squares/>
//! Medium - [math, dynamic-programming, breadth-first-search, knapsack-problem, complete-knapsack]
//!
//! Given an integer n, return the least number of perfect square numbers that sum to n.
//! A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
//!
//! Example 1:
//! Input: n = 12
//! Output: 3
//! Explanation: 12 = 4 + 4 + 4.
//! Example 2:
//! Input: n = 13
//! Output: 2
//! Explanation: 13 = 4 + 9.
//!
//! Constraints:
//! - 1 <= n <= 10**4

const fn perfect_squares<const N: usize>() -> [usize; N] {
    let mut squares = [0; N];
    let mut i = 0;
    while i < N {
        squares[i] = i.pow(2);
        i += 1;
    }
    squares
}

const PERFECT_SQUARES: [usize; 102] = perfect_squares::<102>();

pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> usize {
        let n_usize = n as usize;
        let i = PERFECT_SQUARES
            .iter()
            .position(|&square| square.gt(&n_usize))
            .unwrap();
        Self::coin_exchange(n_usize, &PERFECT_SQUARES[1..i])
    }

    fn coin_exchange(target: usize, coins: &[usize]) -> usize {
        let mut dp = Self::make_dp_matrix(target, coins);
        for i_coin in 1..=coins.len() {
            for change in 0..=target {
                Self::make_change(change, i_coin, coins[i_coin - 1], &mut dp);
            }
        }
        dp[coins.len()][target]
    }

    fn make_change(change: usize, i_coin: usize, coin: usize, dp: &mut [Vec<usize>]) {
        // coin | 0 1 2 3 4 5       coin | 0 1 2 3 4 5       coin | 0 1 2 3 4 5
        // -----|------------       -----|------------       -----|------------
        //  0   | 0 0 0 0 0 0 take   0   | 0 0 0 0 0 0 take   0   | 0 0 0 0 0 0
        //  1   | 0 . . . . . ---->  1   | 0 1 . . . . ---->  1   | 0 1 2 3 4 5
        //  4   | 0 . . . . .        4   | 0 . . . . .        4   | 0 . . . . .
        //
        // The coin is too big      The coin is right        Refer back to dp[2][1] + 1
        // coin | 0 1 2 3 4 5       coin | 0 1 2 3 4 5       coin | 0 1 2 3 4 5
        // -----|------------       -----|------------       -----|------------
        //  0   | 0 0 0 0 0 0 take   0   | 0 0 0 0 0 0 take   0   | 0 0 0 0 0 0
        //  1   | 0 1 2 3 4 5 ---->  1   | 0 1 2 3 4 5 ---->  1   | 0 1 2 3 4 5
        //  4   | 0 1 . . . .        4   | 0 1 2 3 1 .        4   | 0 1 2 3 1 2
        dp[i_coin][change] = dp[i_coin - 1][change];
        if change >= coin {
            let leave_coin = dp[i_coin][change];
            let take_coin = dp[i_coin][change - coin] + 1;
            dp[i_coin][change] = leave_coin.min(take_coin);
        }
    }

    fn make_dp_matrix(target: usize, coins: &[usize]) -> Vec<Vec<usize>> {
        // coin | 0 1 2 3 4 5
        // -----|------------
        //  0   | 0 0 0 0 0 0
        //  1   | 0 . . . . .
        //  4   | 0 . . . . .
        // We only need the current and previous row, so the table could be trimmed.
        // This only uses O(t) space, not O(t * n), with t and n the target and number of coins.
        let mut dp = vec![vec![usize::MAX; target + 1]; coins.len() + 1];
        dp[0][0] = 0;
        dp
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0() {
        assert_eq!(Solution::num_squares(0), 0)
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::num_squares(5), 2)
    }

    #[test]
    fn test_12() {
        // 4 + 4 + 4 = 12
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_13() {
        // 4 + 9 = 13
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn test_7168() {
        // 4 + 9 = 13
        assert_eq!(Solution::num_squares(7168), 4);
    }

    #[test]
    fn test_10_000() {
        // 4 + 9 = 13
        assert_eq!(Solution::num_squares(10_000), 1);
    }
}
