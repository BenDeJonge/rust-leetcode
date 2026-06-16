//! <https://leetcode.com/problems/find-all-anagrams-in-a-string/>
//! Medium - [hash-table, string, sliding-window]
//!
//! Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
//!
//! Example 1:
//! Input: s = &"cbaebabacd&", p = &"abc&"
//! Output: [0,6]
//! Explanation:
//! The substring with start index = 0 is &"cba&", which is an anagram of &"abc&".
//! The substring with start index = 6 is &"bac&", which is an anagram of &"abc&".
//! Example 2:
//! Input: s = &"abab&", p = &"ab&"
//! Output: [0,1,2]
//! Explanation:
//! The substring with start index = 0 is &"ab&", which is an anagram of &"ab&".
//! The substring with start index = 1 is &"ba&", which is an anagram of &"ab&".
//! The substring with start index = 2 is &"ab&", which is an anagram of &"ab&".
//!
//! Constraints:
//! - 1 <= s.length, p.length <= 3 * 10**4
//! - s and p consist of lowercase English letters.

use std::{
    iter,
    ops::{Deref, DerefMut},
};

#[derive(PartialEq, Debug)]
struct Alphabet([usize; 26]);

impl Alphabet {
    pub fn new() -> Self {
        Self([0; 26])
    }

    pub fn replace(&mut self, old: u8, new: u8) {
        let old_val = self
            .get_mut(Self::byte_to_idx(old))
            .expect("old out of bounds");
        *old_val = old_val.saturating_sub(1);

        let new_val = self
            .get_mut(Self::byte_to_idx(new))
            .expect("new out of bounds");
        *new_val = new_val.saturating_add(1);
    }

    fn byte_to_idx(byte: u8) -> usize {
        (byte - b'a').into()
    }
}

impl From<&str> for Alphabet {
    fn from(value: &str) -> Self {
        let mut alphabet: [usize; 26] = [0; 26];
        for byte in value.as_bytes() {
            let i = Self::byte_to_idx(*byte);
            alphabet[i] = alphabet[i].saturating_add(1);
        }
        Self(alphabet)
    }
}

impl From<&Alphabet> for String {
    fn from(val: &Alphabet) -> Self {
        let mut bytes = vec![];
        for (byte, count) in val.iter().enumerate() {
            bytes.extend(iter::repeat_n(byte as u8 + b'a', *count));
        }
        String::from_utf8_lossy(&bytes).to_string()
    }
}

impl Deref for Alphabet {
    type Target = [usize; 26];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Alphabet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        Self::find_anagrams_helper(&p, &s)
            .map(|idx| idx as i32)
            .collect()
    }

    fn find_anagrams_helper<'a>(
        needle: &'a str,
        haystack: &'a str,
    ) -> Box<dyn Iterator<Item = usize> + 'a> {
        // In order to return a different iterator in this case,
        // a trait object must be returned instead of an Impl.
        if needle.len() > haystack.len() {
            return Box::new(iter::empty::<usize>());
        }

        let target = Alphabet::from(needle);
        // This slice requires handling the above edge case.
        let mut current = Alphabet::from(&haystack[..needle.len()]);
        let mut old = *haystack.as_bytes().first().expect("haystack is empty");

        Box::new(
            haystack
                .as_bytes()
                .windows(needle.len())
                .enumerate()
                .filter_map(move |(i, window)| {
                    if i > 0 {
                        let new = *window.last().expect("needle is empty");
                        current.replace(old, new);
                        old = *window.first().expect("needle is empty");
                    }
                    if current == target { Some(i) } else { None }
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0438() {
        assert_eq!(
            Solution::find_anagrams_helper("a", "aaaa").collect::<Vec<usize>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::find_anagrams_helper("abc", "cbaebabacd").collect::<Vec<usize>>(),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams_helper("ab", "abab").collect::<Vec<usize>>(),
            vec![0, 1, 2]
        );
        assert_eq!(
            Solution::find_anagrams_helper("ef", "abcd").collect::<Vec<usize>>(),
            Vec::<usize>::new()
        );
        assert_eq!(
            Solution::find_anagrams_helper("aaaa", "aaa").collect::<Vec<usize>>(),
            Vec::<usize>::new()
        );
    }
}
