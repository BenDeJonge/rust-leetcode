//! <https://leetcode.com/problems/longest-common-subsequence/>
//! Medium - [string, dynamic-programming, longest-common-subsequence]
//!
//! Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
//! A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
//! - For example, &"ace&" is a subsequence of &"abcde&".
//!
//! A common subsequence of two strings is a subsequence that is common to both strings.
//!
//! Example 1:
//! Input: text1 = &"abcde&", text2 = &"ace&"
//! Output: 3  
//! Explanation: The longest common subsequence is &"ace&" and its length is 3.
//! Example 2:
//! Input: text1 = &"abc&", text2 = &"abc&"
//! Output: 3
//! Explanation: The longest common subsequence is &"abc&" and its length is 3.
//! Example 3:
//! Input: text1 = &"abc&", text2 = &"def&"
//! Output: 0
//! Explanation: There is no such common subsequence, so the result is 0.
//!
//! Constraints:
//! - 1 <= text1.length, text2.length <= 1000
//! - text1 and text2 consist of only lowercase English characters.

pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Self::lcs(text1.as_bytes(), text2.as_bytes())
            .try_into()
            .unwrap()
    }

    /// Solve in:
    /// - `O_t(N * M)`
    /// - `O_m(2 * min(N, M))`
    ///
    /// with `N`, `M` the length of `a` and `b`, respectively.
    pub fn lcs<T: PartialEq>(a: &[T], b: &[T]) -> usize {
        // Minimize memory footprint.
        if a.len() < b.len() {
            return Self::lcs(b, a);
        }

        // Construct the following DP score matrix:
        //   | . a b c d e      | . a b c d e
        // --|------------    --|------------
        // . | 0 0 0 0 0 0    . | 0 0 0 0 0 0
        // a | 0 . . . . . -> a | 0 1 1 1 1 1
        // c | 0 . . . . .    c | 0 1 1 2 2 2
        // e | 0 . . . . .    e | 0 1 1 2 2 3
        // For memory optimization purposes, we only track the previous and current row.
        let mut previous = vec![0; b.len() + 1];
        for el_a in a.iter() {
            let mut current = previous.clone();
            for (c, el_b) in b.iter().enumerate() {
                // Add a new element to the current subsequence (NW).
                current[c + 1] = if el_a == el_b {
                    previous[c] + 1
                }
                // Choose the current LCS (N or W).
                else {
                    previous[c + 1].max(current[c])
                }
            }
            std::mem::swap(&mut previous, &mut current);
        }
        previous[b.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn helper(a: &str, b: &str, expected: &str) {
        assert_eq!(Solution::lcs(a.as_bytes(), b.as_bytes()), expected.len());
    }

    #[test]
    fn test_1250() {
        helper("ace", "abcde", "ace");
        helper("ace", "fabcde", "ace");
        helper("mhzqniwbn", "zmhazijwbq", "mhziwb");
        helper("bsbininm", "jmjkbkjkv", "m");
    }

    #[test]
    fn test_long() {
        // Test case from https://en.wikipedia.org/wiki/Longest_common_subsequence
        helper(
            "this is some text that will be changed",
            "this is the changed text",
            "this is the changed",
        );
    }

    #[test]
    fn test_same_strings() {
        helper("abc", "abc", "abc");
    }

    #[test]
    fn test_no_match() {
        helper("abc", "def", "");
    }

    #[test]
    fn test_empty_strings() {
        helper("", "abc", "");
        helper("abc", "", "");
        helper("", "", "");
    }
}
