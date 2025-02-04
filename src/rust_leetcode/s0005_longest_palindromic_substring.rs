//! https://leetcode.com/problems/longest-palindromic-substring/
//! Medium - [two pointers, string, dynamic programming]
//! Given a string s, return the longest palindromic substring in s.

pub struct Palindrome {
    i_start: usize,
    length: usize,
}

impl Palindrome {
    pub fn new(length: usize, i_start: usize) -> Self {
        Self { length, i_start }
    }

    pub fn update(&mut self, i_start: usize, length: usize) {
        if self.length < length {
            self.i_start = i_start;
            self.length = length;
        }
    }

    pub fn to_string(&self, chars: Vec<char>) -> String {
        chars[self.i_start..self.i_start + self.length]
            .iter()
            .collect()
    }
}

impl Default for Palindrome {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

pub struct Solution {}

impl Solution {
    fn longest_palindrome(s: String) -> String {
        // Edge cases on length.
        if s.is_empty() {
            return "".to_string();
        } else if s.len() == 1 {
            return s.chars().next().unwrap().to_string();
        }
        // Constructing a vec of chars for easier iteration.
        let chars = s.chars().collect::<Vec<char>>();
        // We can safely set length to 1 as we know the string is not empty.
        let mut best: Palindrome = Palindrome::new(1, 0);
        // Construct a table representing if the string slice between bounds [left, right[ is palindromic.
        // This requires len x len entries. This is similar to a covariance matrix!
        let mut cache = vec![vec![false; s.len()]; s.len()];
        // All substrings of length 1 are palindromic.
        //          i
        //    1  0  .  0  0
        //    0  1  .  0  0
        // j  .  .  .  .  .
        //    0  0  .  1  0
        //    0  0  .  0  1
        for (i, el) in cache.iter_mut().enumerate() {
            el[i] = true;
        }
        // Special case of length 2 substrings. Below, we will check if adding two outer characters keeps the slice a
        // palindrome. However, for even lengthts, we then first need to know which pairs of two are palindromic.
        for i in 0..(s.len() - 1) {
            if chars[i] == chars[i + 1] {
                cache[i][i + 1] = true;
                best.update(i, 2);
            }
        }
        // Checking all other lengths, starting at 3. Here, we expand both indices outwards.
        // Make length inclusive, as we also need to test for the whole string being palindromic.
        for length in 3..=s.len() {
            // Looping over starting positions.
            for i in 0..s.len() - length + 1 {
                // The ending index, so we need minus 1 because we are using substring length.
                let j = i + length - 1;
                // It was a palindrome before we added the two outer characters.
                if cache[i + 1][j - 1]
				// The two outer characters are the same, so it is still a palindrome.
				&& chars[i] == chars[j]
                {
                    cache[i][j] = true;
                    best.update(i, length);
                }
            }
        }
        best.to_string(chars)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0005() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("ccc".to_string()),
            "ccc".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("a".to_string()),
            "a".to_string()
        );
        assert_eq!(Solution::longest_palindrome("".to_string()), "".to_string());
    }
}
