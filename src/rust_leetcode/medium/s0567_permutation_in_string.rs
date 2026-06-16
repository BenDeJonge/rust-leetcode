//! <https://leetcode.com/problems/permutation-in-string/>
//! Medium - [hash-table, two-pointers, string, sliding-window]
//!
//! Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
//! In other words, return true if one of s1's permutations is the substring of s2.
//!
//! Example 1:
//! Input: s1 = &"ab&", s2 = &"eidbaooo&"
//! Output: true
//! Explanation: s2 contains one permutation of s1 (&"ba&").
//! Example 2:
//! Input: s1 = &"ab&", s2 = &"eidboaoo&"
//! Output: false
//!
//! Constraints:
//! - 1 <= s1.length, s2.length <= 10**4
//! - s1 and s2 consist of lowercase English letters.

#[derive(PartialEq, Debug)]
struct Alphabet([usize; 26]);

impl Alphabet {
    pub fn new() -> Self {
        Self([0; 26])
    }

    pub fn replace(&mut self, old: u8, new: u8) {
        let i_old = Self::byte_to_idx(old);
        self.0[i_old] = self.0[i_old].saturating_sub(1);
        let i_new = Self::byte_to_idx(new);
        self.0[i_new] = self.0[i_new].saturating_add(1);
    }

    fn byte_to_idx(byte: u8) -> usize {
        // SAFETY: this will only work for lowercase English letters.
        (byte - b'a').into()
    }
}

impl From<&str> for Alphabet {
    fn from(value: &str) -> Self {
        let mut alphabet: [usize; 26] = [0; 26];
        for byte in value.as_bytes() {
            let i = Alphabet::byte_to_idx(*byte);
            alphabet[i] = alphabet[i].saturating_add(1);
        }
        Self(alphabet)
    }
}

pub struct Solution {}

impl Solution {
    /// Solve in O(n), with n the length of s2.
    ///
    /// A sliding window approach where the newly added letters from the right
    /// replace deprecated letters at the left.
    pub fn check_inclusion(s1: &str, s2: &str) -> bool {
        if s2.len() < s1.len() {
            return false;
        }

        let target = Alphabet::from(s1);
        let mut current = Alphabet::from(&s2[..s1.len()]);
        if current == target {
            return true;
        }

        let mut old = *s2.as_bytes().first().expect("s2 not empty");
        for window in s2.as_bytes().windows(s1.len()).skip(1) {
            let new = *window.last().expect("window not empty");
            current.replace(old, new);
            if current == target {
                return true;
            }
            old = *window.first().expect("window not empty");
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0567_true() {
        assert!(Solution::check_inclusion("abc", "abc"));
        assert!(Solution::check_inclusion("abc", "bac"));
        assert!(Solution::check_inclusion("abc", "abcdef"));
        assert!(Solution::check_inclusion("abc", "defabc"));
        assert!(Solution::check_inclusion("abc", "defbac"));
    }

    #[test]
    fn test_0567_false() {
        assert!(!Solution::check_inclusion("abc", "adc"));
        assert!(!Solution::check_inclusion("abc", "ab"));
        assert!(!Solution::check_inclusion("bc", "bbbbbb"));
        assert!(!Solution::check_inclusion("bc", "bbbaccc"));
    }
}
