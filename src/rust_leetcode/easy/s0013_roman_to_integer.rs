//! https://leetcode.com/problems/roman-to-integer/
//! Easy - [hash table, math, string]
//! Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//! Given a roman numeral, convert it to an integer.

use std::cmp::{Ord, Ordering};

pub struct Solution {}

pub enum RomanDigits {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl RomanDigits {
    pub fn from_char(c: char) -> Result<RomanDigits, String> {
        match c {
            'I' => Ok(RomanDigits::I),
            'V' => Ok(RomanDigits::V),
            'X' => Ok(RomanDigits::X),
            'L' => Ok(RomanDigits::L),
            'C' => Ok(RomanDigits::C),
            'D' => Ok(RomanDigits::D),
            'M' => Ok(RomanDigits::M),
            _ => Err("could not parse".to_string()),
        }
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        println!();
        let mut previous = 0;
        s.chars()
            .rev()
            .map(|c| {
                let current = RomanDigits::from_char(c).expect("could not parse") as i32;
                // Store the value of the previous digit before overwriting it with the current one.
                let tmp = previous;
                previous = current;
                match current.cmp(&tmp) {
                    // Normally, Roman digits are declining in value from left to right. Each one increases the value.
                    Ordering::Equal | Ordering::Greater => current,
                    // However, smaller digits can modify the value of the following bigger one. They decrease the next
                    // number (or the total sum) with their own value.
                    Ordering::Less => -current,
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_0013() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
