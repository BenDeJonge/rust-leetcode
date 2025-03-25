//! https://leetcode.com/problems/coin-change/
//! Medium - [array, dynamic-programming, breadth-first-search]
//! You are given an integer array coins representing coins of different
//! denominations and an integer amount representing a total amount of money.
//! Return the fewest number of coins that you need to make up that amount.
//! If that amount of money cannot be made up by any combination, return -1.
//! You may assume that you have an infinite number of each kind of coin.
//!
//! Example 1:
//! Input: coins = [1,2,5], amount = 11
//! Output: 3
//! Explanation: 11 = 5 + 5 + 1
//! Example 2:
//! Input: coins = [2], amount = 3
//! Output: -1
//! Example 3:
//! Input: coins = [1], amount = 0
//! Output: 0
//!
//! Constraints:
//! `1 <= coins.length <= 12`
//! `1 <= coins[i] <= 231 - 1`
//! `0 <= amount <= 104`

use std::cmp;

type CoinMemo = Vec<Vec<Option<usize>>>;

pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // Base cases.
        if amount == 0 {
            return 0;
        }
        // Sort the coins as usizes from large to small, removing zeros.
        let amount_usize = amount as usize;
        let mut coins_usize = coins
            .iter()
            .filter(|c| **c > 0)
            .map(|c| *c as usize)
            .collect::<Vec<usize>>();
        // coins_usize.sort_by(|a, b| b.cmp(a));
        coins_usize.sort();
        if let Some(res) = Self::coin_change_dp_onedimensional(&coins_usize, amount_usize) {
            res as i32
        } else {
            -1
        }
    }

    /// Time complexity `O(n^a)` with `n = coins.len()` and `a = amount`.
    pub fn coin_change_recursive(coins: &[usize], amount: usize, n_taken: usize) -> Option<usize> {
        if amount == 0 {
            return Some(n_taken);
        }
        for coin in coins.iter() {
            match coin.cmp(&amount) {
                cmp::Ordering::Equal => return Some(n_taken + 1),
                cmp::Ordering::Less => {
                    return Self::coin_change_recursive(coins, amount - coin, n_taken + 1)
                }
                cmp::Ordering::Greater => continue,
            }
        }
        None
    }

    /// Track all possible combinations of coins and remaining amounts in
    /// memory complexity `O(coins * amount)`.
    /// Each coordinate `(i, j)` stores the (current) minimum number of coins
    /// needed to make the amount `j` with all coins up to `i`.
    ///
    /// E.g., the full matrix to make 15 with coins [1, 2, 5] would look like.
    /// ```text
    ///      0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// ---------------------------------------------------
    /// 1 |  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    /// 2 |  0  1  1  2  2  3  3  4  4  5  5  6  6  7  7  8
    /// 5 |  0  1  1  2  2  1  2  2  3  3  2  3  3  4  4  3
    /// ```
    pub fn make_dp_matrix(coins: &[usize], amount: usize) -> CoinMemo {
        // Fence post problem: store sum 5 as [0, 1, 2, 3, 4, 5] in 6 elements.
        vec![vec![None; amount + 1]; coins.len()]
    }

    /// The recursive solution only tracks the used coin and remaining amount,
    /// so these can be tabulated in a memo array.
    pub fn coin_change_dp(
        i: usize,
        amount: usize,
        coins: &[usize],
        memo: &mut CoinMemo,
    ) -> Option<usize> {
        // There are no coins left to try.
        if i == coins.len() {
            return None;
        }
        // There is a previous solutions.
        if let Some(previous) = memo[i][amount] {
            return Some(previous);
        }

        let take = {
            match coins[i].cmp(&amount) {
                cmp::Ordering::Greater => None,
                cmp::Ordering::Equal => Some(1),
                cmp::Ordering::Less => {
                    Self::coin_change_dp(i, amount - coins[i], coins, memo).map(|take| take + 1)
                }
            }
        };
        let dont_take = Self::coin_change_dp(i + 1, amount, coins, memo);

        memo[i][amount] = match (take, dont_take) {
            (Some(t), Some(dt)) => Some(t.min(dt)),
            (Some(t), None) => Some(t),
            (None, Some(dt)) => Some(dt),
            (None, None) => None,
        };
        // The number of coins totaling the amount after adding the last one.
        memo[i][amount]
    }

    /// A one-dimensional dynamic programming solution that leads to a solution
    /// in time complexity of `C(coins * amount)`.
    pub fn coin_change_dp_onedimensional(coins: &[usize], amount: usize) -> Option<usize> {
        // Store the mimimum number of coins to reach any amount (0..=amount)
        let mut memo = vec![usize::MAX; amount + 1];
        memo[0] = 0;
        for target in 1..=amount {
            // Assumes that coins is sorted .
            for &coin in coins {
                if coin > target {
                    break;
                }
                // If a coin can reach target, target - coin is reachable too.
                if memo[target - coin] != usize::MAX {
                    // If adding the current coin leads to a smaller amount,
                    // update the current best case.
                    memo[target] = memo[target].min(1 + memo[target - coin])
                }
            }
        }
        if memo[amount] != usize::MAX {
            Some(memo[amount])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0322() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
        assert_eq!(
            Solution::coin_change(
                vec![411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422],
                9864
            ),
            24
        );
    }
}
