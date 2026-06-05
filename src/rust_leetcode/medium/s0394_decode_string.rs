//! <https://leetcode.com/problems/decode-string/>
//! Medium - [string, stack, recursion]
//!
//! Given an encoded string, return its decoded string.
//! The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets
//! is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
//! You may assume that the input string is always valid; there are no extra white spaces, square brackets
//! are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits
//! and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].
//! The test cases are generated so that the length of the output will never exceed 10**5.
//!
//! Example 1:
//! Input: s = &"3[a]2[bc]&"
//! Output: &"aaabcbc&"
//! Example 2:
//! Input: s = &"3[a2[c]]&"
//! Output: &"accaccacc&"
//! Example 3:
//! Input: s = &"2[abc]3[cd]ef&"
//! Output: &"abcabccdcdcdef&"
//!
//! Constraints:
//! - 1 <= s.length <= 30
//! - s consists of lowercase English letters, digits, and square brackets '[]'.
//! - s is guaranteed to be a valid input.
//! - All the integers in s are in the range [1, 300].

use std::iter;

struct Encoding {
    n: usize,
    string: String,
}

impl Encoding {
    fn new(n: usize) -> Self {
        Self {
            n,
            string: String::new(),
        }
    }

    fn encode(self) -> String {
        String::from_iter(iter::repeat_n(self.string, self.n))
    }

    fn push_str(&mut self, s: &str) {
        self.string.push_str(s);
    }

    fn push(&mut self, ch: char) {
        self.string.push(ch);
    }
}

pub struct Solution {}

impl Solution {
    /// Solves in O(N)
    ///
    /// An encoding is started and terminated by '[' and ']', respectively.
    /// When an encoding is terminated, check if its output can be directly
    /// added to the output or is nested in another encoding.
    /// This can be achieved with a stack, mimicking recursion.
    pub fn decode_string(s: &str) -> String {
        let mut res = String::new();
        let mut stack = Vec::<Encoding>::new();
        let mut digits = 0;
        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    digits = digits * 10
                        + usize::try_from(ch.to_digit(10).expect("is 0..=9"))
                            .expect("is positive i32");
                }
                '[' => {
                    stack.push(Encoding::new(digits));
                    digits = 0;
                }
                ']' => {
                    let last = stack.pop().expect("every ] preceded by [");
                    if let Some(previous) = stack.last_mut() {
                        previous.push_str(&last.encode());
                    } else {
                        res.push_str(&last.encode());
                    }
                }
                _ => {
                    if let Some(enc) = stack.last_mut() {
                        enc.push(ch);
                    } else {
                        res.push(ch);
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0394() {
        assert_eq!(&Solution::decode_string("3[a]2[bc]&"), &"aaabcbc&");
        assert_eq!(&Solution::decode_string("3[a2[c]]&"), &"accaccacc&");
        assert_eq!(
            &Solution::decode_string("2[abc]3[cd]ef&"),
            &"abcabccdcdcdef&"
        );
        assert_eq!(
            &Solution::decode_string("aa2[b2[c2[d]]]ef"),
            &"aabcddcddbcddcddef"
        );
    }
}
