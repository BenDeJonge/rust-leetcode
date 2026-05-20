//! <https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/>
//! Easy - [two pointers, string, string matching]
//! Given two strings needle and haystack,
//! return the index of the first occurrence of needle in haystack,
//! or -1 if needle is not part of haystack.

use crate::util::strings::char_windows;

pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: &str, needle: &str) -> Option<usize> {
        haystack.find(needle)
    }

    pub fn str_str_naive(haystack: &str, needle: &str) -> Option<usize> {
        for (i, w) in char_windows(haystack, needle.len()).enumerate() {
            if w == needle {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0028() {
        assert_eq!(Solution::str_str("sadbutsad", "sad"), Some(0));
        assert_eq!(Solution::str_str("leetcode", "leeto"), None);
        assert_eq!(Solution::str_str("aababbbabab", "babab"), Some(6));
    }
}
