//! https://leetcode.com/problems/letter-combinations-of-a-phone-number/
//! Medium - [hash table, string, backtracking]
//! Given a string containing digits from 2-9 inclusive,
//! return all possible letter combinations that the number could represent.
//! Return the answer in any order.
//! A mapping of digits to letters (just like on the telephone buttons) is given below.
//! Note that 1 does not map to any letters.

pub struct Solution {}

const LETTERS_PER_DIGIT: [&str; 10] = [
    "",     // 0
    "",     // 1
    "abc",  // 2
    "def",  // 3
    "ghi",  // 4
    "jkl",  // 5
    "mno",  // 6
    "pqrs", // 7
    "tuv",  // 8
    "wxyz", // 9
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        // Instantiate containers with the correct capacity to avoid O(n) reallocations.
        let mut words: Vec<String> = Vec::with_capacity(
            LETTERS_PER_DIGIT
                .iter()
                .map(|s| s.len())
                .max()
                .expect("LETTERS_PER_DIGIT is empty")
                .pow(digits.len() as u32),
        );
        let mut current_word: Vec<char> = Vec::with_capacity(digits.len());
        Solution::letter_combinations_helper(
            &mut words,
            &mut current_word,
            &digits.chars().collect::<Vec<char>>(),
            0,
            &LETTERS_PER_DIGIT,
        );
        words
    }

    pub fn letter_combinations_helper(
        words: &mut Vec<String>,
        current_word: &mut Vec<char>,
        digits: &[char],
        i_digit: usize,
        letters_per_digit: &[&'static str; 10],
    ) {
        // There are more digits to translate.
        if let Some(digit) = digits.get(i_digit) {
            // Get all the letter associated with the next digit.
            let letters = letters_per_digit
                .get(digit.to_digit(10).expect("char is not a digit") as usize)
                .expect("digit is not [0, 9]")
                .chars();
            // Recurse down for every letter that is available for the digit ...
            for letter in letters {
                // ... add the letter to the current word ...
                current_word.push(letter);
                Solution::letter_combinations_helper(
                    words,
                    current_word,
                    digits,
                    i_digit + 1,
                    letters_per_digit,
                );
                // ... and track back once all downstream recursions are complete.
                // Move on to the next letter in this position by removing this one i.e., BACKTRACKING.
                current_word.pop();
            }
        } else {
            // There are no more digits i.e., one letter was added for each digit, meaining the word is complete.
            words.push(current_word.iter().collect());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    #[test]
    fn test_0017() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            <Vec<String>>::new()
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
