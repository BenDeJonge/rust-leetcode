//! https://leetcode.com/problems/climbing-stairs/
//! Easy - [math, dynamic programming, memoization]
//! You are climbing a staircase. It takes n steps to reach the top.
//! Each time you can either climb 1 or 2 steps.
//! In how many distinct ways can you climb to the top?

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    /// This feels like a math question rather than a programming one.
    /// Let's write out all combinations for n in [0, 6].
    /// n = 0: 1 (there is 1 way to climb 0 stairs: do not climb any stairs)
    ///     []
    /// n = 1: 1
    ///     [1]
    /// n = 2: 2
    ///     [2]
    ///     [1,1]
    /// n = 3: 3
    ///     [2, 1], [1, 2]
    ///     [1, 1, 1]
    /// n = 4: 5
    ///     [2, 2]
    ///     [2, 1, 1], [1, 2, 1], [1, 1, 2]
    ///     [1, 1, 1, 1]
    /// n = 5: 8
    ///     [2, 2, 1], [2, 1, 2], [1, 2, 2]
    ///     [2, 1, 1, 1], [1, 2, 1, 1], [1, 1, 2, 1], [1, 1, 1, 2]
    ///     [1, 1, 1, 1, 1]
    /// n = 6: 13
    ///     [2, 2, 2]
    ///     [2, 2, 1, 1], [2, 1, 2, 1], [2, 1, 1, 2], [1, 2, 2, 1], [1, 2, 1, 2], [1, 1, 2, 2]
    ///     [2, 1, 1, 1, 1], [1, 2, 1, 1, 1], [1, 1, 2, 1, 1], [1, 1, 1, 2, 1], [1, 1, 1, 1, 2]
    ///     [1, 1, 1, 1, 1, 1]
    /// In table form, it becomes clear that f(n) is the Fibonacci sequence!
    /// The mysteries of life...
    /// |   n  | 0 | 1 | 2 | 3 | 4 | 5 |  6 |
    /// |------|---|---|---|---|---|---|----|
    /// | f(n) | 1 | 1 | 2 | 3 | 5 | 8 | 13 |
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = <HashMap<i32, i32>>::new();
        Solution::fib(n, &mut cache)
    }

    pub fn fib(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(&old) = cache.get(&n) {
            return old;
        }
        let answer = if n < 2 {
            1
        } else {
            Solution::fib(n - 1, cache) + Solution::fib(n - 2, cache)
        };
        cache.insert(n, answer);
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_0070() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
