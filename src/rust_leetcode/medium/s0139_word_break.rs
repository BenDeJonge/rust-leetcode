//! https://leetcode.com/problems/word-break/
//! Medium - [array, hash-table, string, dynamic-programming, trie, memoization]
//!
//! Given a string s and a dictionary of strings wordDict, return true if s can
//! be segmented into a space-separated sequence of one or more dictionary words.
//! Note that the same word in the dictionary may be reused multiple times in the segmentation.
//!
//! Example 1:
//! Input: s = &"leetcode&", wordDict = [&"leet&",&"code&"]
//! Output: true
//! Explanation: Return true because &"leetcode&" can be segmented as &"leet code&".
//! Example 2:
//! Input: s = &"applepenapple&", wordDict = [&"apple&",&"pen&"]
//! Output: true
//! Explanation: Return true because &"applepenapple&" can be segmented as &"apple pen apple&".
//! Note that you are allowed to reuse a dictionary word.
//! Example 3:
//! Input: s = &"catsandog&", wordDict = [&"cats&",&"dog&",&"sand&",&"and&",&"cat&"]
//! Output: false
//!
//! Constraints:
//! - 1 <= s.length <= 300
//! - 1 <= wordDict.length <= 1000
//! - 1 <= wordDict[i].length <= 20
//! - s and wordDict[i] consist of only lowercase English letters.
//! - All the strings of wordDict are unique.

pub struct Solution {}

impl Solution {
    /// Solves in O(n * m * l), with:
    /// - n: the length of the string s
    /// - m: the number of strings in word_dict
    /// - l: the difference in length between the mean word_dict and s.
    pub fn word_break(s: &str, word_dict: &[&str]) -> bool {
        // No need for an Option<bool>, where None means "not checked".
        // For non-checked starts, we need to explore anyway.
        // Any positive check will eventually get overwritten by a negative
        // if needed, including the answer at index 0.
        let mut previous: Vec<bool> = vec![true; s.len()];
        Self::word_break_helper(s, 0, word_dict, &mut previous);
        previous[0]
    }

    fn word_break_helper(s: &str, start: usize, word_dict: &[&str], previous: &mut [bool]) -> bool {
        // The full string was succesfully parsed.
        if start == s.len() {
            return true;
        }
        // Known to be impossible.
        if start > s.len() || !previous[start] {
            return false;
        }

        previous[start] = word_dict.iter().any(|word| {
            if s[start..].starts_with(word) {
                Self::word_break_helper(s, start + word.len(), word_dict, previous)
            } else {
                false
            }
        });
        previous[start]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0139() {
        assert!(Solution::word_break("leetcode", &["leet", "code"]));
        assert!(Solution::word_break("applepenapple", &["apple", "pen"]));
        assert!(!Solution::word_break(
            "castandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
        assert!(Solution::word_break("bb", &["a", "bbb", "bbbb", "b"]));
        assert!(!Solution::word_break("a", &["b"]));
        assert!(Solution::word_break("cars", &["car", "ca", "rs"]));
    }
}
