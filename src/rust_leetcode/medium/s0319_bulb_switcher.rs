//! https://leetcode.com/problems/bulb-switcher/
//! Medium - [math, brainteaser]
//!
//! There are `n` bulbs that are initially off.
//! You first turn on all the bulbs, then you turn off every second bulb.
//! On the third round, you toggle every third bulb
//! (turning on if it's off or turning off if it's on).
//! For the `i`-th round, you toggle every `i` bulb.
//! For the `n`-th round, you only toggle the last bulb.
//! Return the number of bulbs that are on after `n` rounds.
//!
//! Example 1:
//! Input: `n = 3`
//! Output: `1`
//! Explanation: At first, the three bulbs are [off, off, off].
//! After the first round, the three bulbs are [on, on, on].
//! After the second round, the three bulbs are [on, off, on].
//! After the third round, the three bulbs are [on, off, off].
//! So you should return 1 because there is only one bulb is on.
//! Example 2:
//! Input: `n = 0`
//! Output: `0`
//! Example 3:
//! Input: `n = 1`
//! Output: `1`
//!
//! Constraints:
//! - 0 <= n <= 109

pub struct Solution {}

impl Solution {
    /// Since a bulb is initially off, it must be switched an odd number of
    /// times to remain on at the end. A bulb at index `i` will therefore be on
    /// if `i` has an odd number of positive divisors.
    ///
    /// Let's look at a small scale example:
    /// `i` | Divisors
    /// ----|-----------
    /// 1   | 1
    /// 2   | 1, 2
    /// 3   | 1, 3
    /// 4   | 1, 2, 4
    /// 5   | 1, 5
    /// 6   | 1, 2, 3, 6
    /// 7   | 1, 7
    /// 8   | 1, 2, 4, 8
    /// 9   | 1, 3, 9
    ///
    /// The only values of `i` that qualify are 1, 4 and 9,
    /// which are the square numbers.
    pub fn bulb_switch(n: i32) -> i32 {
        n.isqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0319() {
        assert_eq!(Solution::bulb_switch(0), 0);
        assert_eq!(Solution::bulb_switch(1), 1);
        assert_eq!(Solution::bulb_switch(3), 1);
        assert_eq!(Solution::bulb_switch(99), 9);
        assert_eq!(Solution::bulb_switch(100), 10);
    }
}
