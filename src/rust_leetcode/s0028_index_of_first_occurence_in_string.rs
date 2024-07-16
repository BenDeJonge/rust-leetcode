/// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/
/// Easy - [two pointers, string, string matching]
/// Given two strings needle and haystack,
/// return the index of the first occurrence of needle in haystack,
/// or -1 if needle is not part of haystack.
use crate::util::strings::char_windows;
pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |i| i as i32)
    }

    pub fn str_str_naive(haystack: String, needle: String) -> i32 {
        for (i, w) in char_windows(&haystack, needle.len()).enumerate() {
            if w == needle {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0028() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
        assert_eq!(
            Solution::str_str("aababbbabab".to_string(), "babab".to_string()),
            6
        );
    }
}
