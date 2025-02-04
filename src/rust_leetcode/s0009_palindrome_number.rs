/// https://leetcode.com/problems/palindrome-number/
/// Easy - [math]
/// Given an integer x, return true if x is a palindrome, and false otherwise.
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers can never be palindromes.
        if x < 0 {
            return false;
        }
        // Construct a new number out of the reverse digits.
        let mut new_num = 0;
        let mut num = x;
        while num > 0 {
            new_num = new_num * 10 + num % 10;
            num /= 10;
        }
        x == new_num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0009() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
