//! https://leetcode.com/problems/palindrome-partitioning/
//! Medium - [string, dynamic-programming, backtracking]
//!
//! Given a string `s`, partition `s` such that every substring of the partition
//! is a palindrome. Return all possible palindrome partitionings of `s`.
//!
//! Example 1:
//! Input: s = "aab"
//! Output: [["a","a","b"],["aa","b"]]
//! Example 2:
//! Input: s = "a"
//! Output: [["a"]]
//!
//! Constraints:
//! - `1 <= s.length <= 16`
//! - `s` contains only lowercase English letters.

pub struct Solution {}

impl Solution {
    // LeetCode does not use reasonable Rust function signatures.
    pub fn partition(s: String) -> Vec<Vec<String>> {
        Solution::partition_as_ref(&s)
    }

    fn partition_as_ref(s: &str) -> Vec<Vec<String>> {
        let mut current = vec![];
        let mut solutions = vec![];
        Solution::helper(s.as_bytes(), 0, &mut current, &mut solutions);
        solutions
    }

    fn helper(s: &[u8], start: usize, current: &mut Vec<String>, solutions: &mut Vec<Vec<String>>) {
        if start == s.len() {
            solutions.push(current.to_vec());
            return;
        }
        // Generate all possible subslices.
        // Explore until s.len because slicing is exclusive at the upper bound.
        for end in (start + 1)..=s.len() {
            let substring = &s[start..end];
            if Solution::is_palindrome(substring) {
                current.push(String::from_utf8(substring.to_vec()).unwrap());
                Solution::helper(s, end, current, solutions);
                // Backtracking.
                current.pop();
            }
        }
    }

    fn is_palindrome(bytes: &[u8]) -> bool {
        if bytes.len() <= 1 {
            return true;
        }
        (0..(bytes.len() / 2)).all(|i| bytes[i] == bytes[bytes.len() - i - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0131() {
        assert_eq!(
            Solution::partition_as_ref("aab"),
            [vec!["a", "a", "b"], vec!["aa", "b"]]
        );
        assert_eq!(Solution::partition_as_ref("a"), vec![vec!["a"]]);
        assert_eq!(
            Solution::partition_as_ref("aaab"),
            [
                vec!["a", "a", "a", "b"],
                vec!["a", "aa", "b"],
                vec!["aa", "a", "b"],
                vec!["aaa", "b"]
            ]
        );
        assert_eq!(
            Solution::partition_as_ref("abcaa"),
            [vec!["a", "b", "c", "a", "a"], vec!["a", "b", "c", "aa"]]
        );
        assert_eq!(
            Solution::partition_as_ref("abbab"),
            [
                vec!["a", "b", "b", "a", "b"],
                vec!["a", "b", "bab"],
                vec!["a", "bb", "a", "b"],
                vec!["abba", "b"]
            ]
        );
        assert_eq!(
            Solution::partition_as_ref("abaca"),
            [
                vec!["a", "b", "a", "c", "a"],
                vec!["a", "b", "aca"],
                vec!["aba", "c", "a"]
            ]
        );
        assert_eq!(
            Solution::partition_as_ref("aaa"),
            [
                vec!["a", "a", "a"],
                vec!["a", "aa"],
                vec!["aa", "a"],
                vec!["aaa"]
            ]
        );
    }

    #[test]
    fn test_is_palindrome() {
        for s in ["", "a", "aa", "aba", "abba", "ababa"] {
            assert!(Solution::is_palindrome(s.as_bytes()))
        }
    }
    #[test]
    fn test_is_not_palindrome() {
        for s in ["ab", "ba", "aaba", "aabba", "abbaa"] {
            assert!(!Solution::is_palindrome(s.as_bytes()))
        }
    }
}
