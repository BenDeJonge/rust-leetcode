//! https://leetcode.com/problems/unique-paths/
//! Medium - [math, dynamic-programming, combinatorics]
//! There is a robot on an `m x n` grid.
//! The robot is initially located at the top-left corner (`grid[0][0]`).
//! The robot tries to move to the bottom-right corner (`grid[m - 1][n - 1]`).
//! The robot can only move either down or right at any point in time.
//! Given the two integers `m` and `n`, return the number of possible unique
//! paths that the robot can take to reach the bottom-right corner.
//! The test cases are generated so that the answer will be <= `2 * 10^9`.
//!
//! Example 1:
//! Input: m = 3, n = 7
//! Output: 28
//! Example 2:
//! Input: m = 3, n = 2
//! Output: 3
//! Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
//! 1. Right -> Down -> Down
//! 2. Down -> Down -> Right
//! 3. Down -> Right -> Down
//!
//! Constraints:
//! - `1 <= m, n <= 100`

/// All factorials that fit in a u128.
/// https://onlinemschool.com/math/formula/factorial_table/
const FACTORIAL_U128_LUT: [u128; 35] = [
    1,                                       // 0
    1,                                       // 1
    2,                                       // 2
    6,                                       // 3
    24,                                      // 4
    120,                                     // 5
    720,                                     // 6
    5040,                                    // 7
    40320,                                   // 8
    362880,                                  // 9
    3628800,                                 // 10
    39916800,                                // 11
    479001600,                               // 12
    6227020800,                              // 13
    87178291200,                             // 14
    1307674368000,                           // 15
    20922789888000,                          // 16
    355687428096000,                         // 17
    6402373705728000,                        // 18
    121645100408832000,                      // 19
    2432902008176640000,                     // 20
    51090942171709440000,                    // 21
    1124000727777607680000,                  // 22
    25852016738884976640000,                 // 23
    620448401733239439360000,                // 24
    15511210043330985984000000,              // 25
    403291461126605635584000000,             // 26
    10888869450418352160768000000,           // 27
    304888344611713860501504000000,          // 28
    8841761993739701954543616000000,         // 29
    265252859812191058636308480000000,       // 30
    8222838654177922817725562880000000,      // 31
    263130836933693530167218012160000000,    // 32
    8683317618811886495518194401280000000,   // 33
    295232799039604140847618609643520000000, // 34
];

pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // Optimization: (n - 1)! must be calculated below.
        // For optimal performance, make sure to minimize n.
        if n > m {
            return Self::unique_paths(n, m);
        }
        // Combination k take i:
        //                k!
        // C(k, i) = -----------
        //           i! (k - i)!
        //
        // Choose i elements out of a set of k where order does not matter.
        // Here, we have to take k = (m - 1) + (n - 1) = m + n - 2 steps.
        // We can (arbitrarily) choose to select i = m - 1 horizontal steps.
        // k - i = m + n - 2 - (m - 1) = n - 1.
        // Logically, this is equal to the number of vertical steps.
        // We can still simplify the combinatorics to minimize need for
        // extremely large factorials:
        //
        //              (m + n - 2)!
        // C(k, i) = -----------------
        //           (m - 1)! (n - 1)!
        //
        //    (m + n - 2) (m + n - 3) ... (m + n - n) (m - 1)!
        // => ------------------------------------------------
        //                    (m - 1)! (n - 1)!
        //
        //    (m + n - 2) (m + n - 3) ... m
        // => -----------------------------
        //               (n - 1)
        let h_steps = (m - 1) as usize;
        let v_steps = (n - 1) as usize;
        (Self::pseudo_factorial(m as u128, (h_steps + v_steps) as u128) / Self::factorial(v_steps))
            .try_into()
            .unwrap()
    }

    fn factorial(i: usize) -> u128 {
        FACTORIAL_U128_LUT[i]
    }

    fn pseudo_factorial(start: u128, stop: u128) -> u128 {
        (start..=stop).product()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0062() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(36, 7), 4496388);
        assert_eq!(Solution::unique_paths(1, 100), 1);
    }
}
