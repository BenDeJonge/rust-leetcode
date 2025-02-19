//! https://leetcode.com/problems/jump-game-vii/
//! Medium - [string, dynamic-programming, sliding-window, prefix-sum]
//! You are given a 0-indexed binary string `s` and two integers `minJump`
//! and `maxJump`. In the beginning, you are standing at index 0,
//! which is equal to `"0"`.
//! Return true if you can reach index `s.length - 1` in `s`.
//! You can move from index `i` to index `j` if:
//! - `i + minJump <= j <= min(i + maxJump, s.length - 1)` and
//! - `s[j] == "0"`.
//!
//! Example 1:
//! Input: `s = "011010", minJump = 2, maxJump = 3`
//! Output: `true`
//! Explanation:
//! In the first step, move from index 0 to index 3.
//! In the second step, move from index 3 to index 5.
//!
//! Example 2:
//! Input: `s = "01101110", minJump = 2, maxJump = 3`
//! Output: `false`
//!
//! Constraints:
//! `2 <= s.length <= 10^5`
//! `s[i]` is either `"0"` or `"1"`.
//! `s[0] == "0"`
//! `1 <= minJump <= maxJump < s.length``

pub struct Solution {}

impl Solution {
    pub fn can_reach_typed(s: &str, min_jump: usize, max_jump: usize) -> bool {
        if s.ends_with('1') {
            return false;
        }
        let mut can_visit: Vec<bool> = s.chars().map(|ch| ch == '0').collect();
        let mut n_ways = 0;
        // Create a sliding window that now:
        // - includes a close by position from where a `min_jump` might reach.
        // - loses a far away position from where a `max_jump` might reach.
        // Only these new bounds need to be checked, as the inner positions
        // were already covered by the jump range.
        //
        // In the below diagram, a capital T/F represents a newly set boolean.
        // ```text
        //                    |       [t f f t f t]  |  Initial state
        // |---|   v          |  |---|   v           |  i = 1, n_ways = 0
        //       0 1 1 0 1 0  |       [t F f t f t]  |    no jumps
        //   |---|   v        |    |---|   v         |  i = 2, n_ways = 1
        //       0 1 1 0 1 0  |       [t f F t f t]  |    + min_jump
        //     |---|   v      |      |---|   v       |  i = 3, n_ways = 1
        //       0 1 1 0 1 0  |       [t f f T f t]  |    no jumps
        //       |---|   v    |        |---|   v     |  i = 4, n_ways = 0
        //       0 1 1 0 1 0  |       [t f f t F t]  |    - max_jump
        //         |---|   v  |          |---|   v   |  i = 5, n_ways = 1
        //       0 1 1 0 1 0  |       [t f f t f T]  |    + min_jump
        // ```
        // Move the window rightwards...
        for i in 1..s.len() {
            // ... did we gain a way to position `i` through a `min_jump`?
            if i >= min_jump {
                n_ways += can_visit[i - min_jump] as isize;
            }
            // ... is an acces point to position `i` now further than `max_jump`?
            if i > max_jump {
                n_ways -= can_visit[i - max_jump - 1] as isize;
            }
            // There is at least one way to visit this position.
            // Positions with a `1` can never be overwritten.
            can_visit[i] &= n_ways > 0;
        }
        // Check if we can visit the final position.
        can_visit[s.len() - 1]
    }

    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        Self::can_reach_typed(&s, min_jump as usize, max_jump as usize)
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test_2001() {
        assert!(!Solution::can_reach_typed("01", 1, 1,));
        assert!(!Solution::can_reach_typed("0000000000", 8, 8,));
        assert!(Solution::can_reach_typed("011010", 2, 3,));
        assert!(!Solution::can_reach_typed("01101110", 2, 3,));
    }
}
