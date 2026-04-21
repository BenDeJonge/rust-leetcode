//! https://leetcode.com/problems/next-greater-element-iii/
//! Medium - [math, two-pointers, string]
//!
//! Given a positive integer n, find the smallest integer which has exactly the same digits existing in the integer n and is greater in value than n. If no such positive integer exists, return -1.
//! Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does not fit in 32-bit integer, return -1.
//!
//! Example 1:
//! Input: n = 12
//! Output: 21
//! Example 2:
//! Input: n = 21
//! Output: -1
//!
//! Constraints:
//! - 1 <= n <= 231 - 1

use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    /// Comply to stupid LeetCodes types.
    pub fn next_greater_element(n: i32) -> i32 {
        Self::next_greater_element_opt(n).unwrap_or(-1)
    }

    /// Solve in O(m(n)) with m(n) the number of digits of n.
    ///
    /// Find the next permutation within i32 bounds.
    /// 230241 -> 230412
    pub fn next_greater_element_opt(n: i32) -> Option<i32> {
        let mut digits = Self::to_digits_le(n);
        let swap = Self::get_non_increasing(&digits);
        Self::get_next_permutation(&mut digits, swap)
    }

    /// Split a number into digits, starting with the least significant.
    fn to_digits_le(n: i32) -> Vec<i32> {
        let mut value = n;
        let mut digits = vec![];
        while value > 0 {
            let digit = value % 10;
            digits.push(digit);
            value /= 10;
        }
        digits
    }

    /// Find the point at which a slice is no longer monotonically increasing.
    ///
    /// ```text
    /// BE: 230241 -> LE: 142032
    /// 1 4 2 0 3 2 | 1 4 2 0 3 2 | 1 4 2 0 3 2
    /// ^ ^         | ^   ^       |   ^ ^
    /// ```
    fn get_non_increasing<T: Ord>(digits: &[T]) -> Option<(usize, usize)> {
        let mut swap = None;
        let mut right = 1;
        'outer: while right < digits.len() {
            let mut left = 0;
            while left < right {
                if digits[right] < digits[left] {
                    swap = Some((left, right));
                    break 'outer;
                }
                left += 1;
            }
            right += 1;
        }
        swap
    }

    /// Convert digits and a swapping point to the next permuation.
    ///
    /// ```text
    /// 1 4 2 0 3 2 | 1 2 4 0 3 2  | 2 1 4 0 3 2
    ///   ^ ^       |   ^ ^ (swap) | ....^ (sort)
    /// 230241 -> 230412
    /// ```
    fn get_next_permutation(digits: &mut [i32], swap: Option<(usize, usize)>) -> Option<i32> {
        if let Some((left, right)) = swap {
            digits.swap(left, right);
            digits[..right].sort_by_key(|d| Reverse(*d));
            Some(
                digits
                    .iter()
                    .enumerate()
                    // Handle i32 bounds by delegating overflows upwards using ?.
                    .try_fold(0i32, |acc, (pow, digit)| {
                        if let Some(d) = digit.checked_mul(10i32.pow(pow as u32)) {
                            acc.checked_add(d)
                        } else {
                            None
                        }
                    })?,
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0556() {
        assert_eq!(Solution::next_greater_element_opt(12), Some(21));
        assert_eq!(Solution::next_greater_element_opt(21), None);
        // From these test cases, it becomes obvious that the goal is
        // to find the next permutation of a number.
        assert_eq!(Solution::next_greater_element_opt(123), Some(132));
        assert_eq!(Solution::next_greater_element_opt(132), Some(213));
        assert_eq!(Solution::next_greater_element_opt(213), Some(231));
        assert_eq!(Solution::next_greater_element_opt(231), Some(312));
        assert_eq!(Solution::next_greater_element_opt(312), Some(321));
        assert_eq!(Solution::next_greater_element_opt(321), None);
        // Some edge cases
        assert_eq!(Solution::next_greater_element_opt(1), None);
        assert_eq!(Solution::next_greater_element_opt(i32::MAX), None);
        // Ensure that the pointer are moving in the correct direction.
        assert_eq!(Solution::next_greater_element_opt(230_241), Some(230_412));
    }
}
