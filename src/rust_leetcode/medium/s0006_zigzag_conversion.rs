//! https://leetcode.com/problems/zigzag-conversion/
//! Medium - [string]
//!
//! The string &"PAYPALISHIRING&" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! And then read line by line: &"PAHNAPLSIIGYIR&"
//! Write the code that will take a string and make this conversion given a number of rows:
//! string convert(string s, int numRows);
//!
//! Example 1:
//! Input: s = &"PAYPALISHIRING&", numRows = 3
//! Output: &"PAHNAPLSIIGYIR&"
//! Example 2:
//! Input: s = &"PAYPALISHIRING&", numRows = 4
//! Output: &"PINALSIGYAHRPI&"
//! Explanation:
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//! Example 3:
//! Input: s = &"A&", numRows = 1
//! Output: &"A&"
//!
//! Constraints:
//! - 1 <= s.length <= 1000
//! - s consists of English letters (lower-case and upper-case), ',' and '.'.
//! - 1 <= numRows <= 1000

use std::num::NonZero;

pub struct Solution {}

impl Solution {
    /// Solve in O(n) with n the length of s.
    ///
    /// Store each level in a String, which track the columns.
    /// Track the rows by moving up and down with a boolean flag.
    pub fn convert(s: &str, num_rows: NonZero<usize>) -> String {
        let n_rows = num_rows.get();
        let mut arr: Vec<String> = vec![String::new(); n_rows];
        let mut r = 0;
        let mut down = true;

        for ch in s.chars() {
            arr[r].push(ch);
            // Keep moving down until back at the first row.
            // Switch to up when at the bottom row.
            down = down & (r > 0) || r == n_rows - 1;
            match down {
                true => r = r.saturating_sub(1),
                false => r += 1,
            }
        }

        arr.concat()
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::Solution;

    #[test]
    fn test_0006() {
        // P   A   H   N
        // A P L S I I G
        // Y   I   R
        assert_eq!(
            Solution::convert("PAYPALISHIRING", NonZero::new(3).unwrap()),
            "PAHNAPLSIIGYIR"
        );
        // P     I    N
        // A   L S  I G
        // Y A   H R
        // P     I
        assert_eq!(
            Solution::convert("PAYPALISHIRING", NonZero::new(4).unwrap()),
            "PINALSIGYAHRPI"
        );
        assert_eq!(Solution::convert("A", NonZero::new(1).unwrap()), "A");
        assert_eq!(Solution::convert("A", NonZero::new(3).unwrap()), "A");
    }
}
