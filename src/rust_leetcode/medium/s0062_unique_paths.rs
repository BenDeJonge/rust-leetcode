//! https://leetcode.com/problems/unique-paths/
//! Medium - [math, dynamic-programming, combinatorics]
//! There is a robot on an `m x n` grid.
//! The robot is initially located at the top-left corner (`grid[0][0]`).
//! The robot tries to move to the bottom-right corner (`grid[m - 1][n - 1]`).
//! The robot can only move either down or right at any point in time.
//! Given the two integers `m` and `n`, return the number of possible unique
//! paths that the robot can take to reach the bottom-right corner.
//! The test cases are generated so that the answer will be <= `2 * 10**9`.
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
    1,                                                   // 0
    1,                                                   // 1
    2,                                                   // 2
    6,                                                   // 3
    24,                                                  // 4
    120,                                                 // 5
    720,                                                 // 6
    5_040,                                               // 7
    40_320,                                              // 8
    362_880,                                             // 9
    3_628_800,                                           // 10
    39_916_800,                                          // 11
    479_001_600,                                         // 12
    6_227_020_800,                                       // 13
    87_178_291_200,                                      // 14
    1_307_674_368_000,                                   // 15
    20_922_789_888_000,                                  // 16
    355_687_428_096_000,                                 // 17
    6_402_373_705_728_000,                               // 18
    121_645_100_408_832_000,                             // 19
    2_432_902_008_176_640_000,                           // 20
    51_090_942_171_709_440_000,                          // 21
    1_124_000_727_777_607_680_000,                       // 22
    25_852_016_738_884_976_640_000,                      // 23
    620_448_401_733_239_439_360_000,                     // 24
    15_511_210_043_330_985_984_000_000,                  // 25
    403_291_461_126_605_635_584_000_000,                 // 26
    10_888_869_450_418_352_160_768_000_000,              // 27
    304_888_344_611_713_860_501_504_000_000,             // 28
    8_841_761_993_739_701_954_543_616_000_000,           // 29
    265_252_859_812_191_058_636_308_480_000_000,         // 30
    8_222_838_654_177_922_817_725_562_880_000_000,       // 31
    263_130_836_933_693_530_167_218_012_160_000_000,     // 32
    8_683_317_618_811_886_495_518_194_401_280_000_000,   // 33
    295_232_799_039_604_140_847_618_609_643_520_000_000, // 34
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
