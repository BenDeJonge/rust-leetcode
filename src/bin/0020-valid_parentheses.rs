/// https://leetcode.com/problems/valid-parentheses/description/
/// Easy - [string, stack]
/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
/// An input string is valid if:
/// 1. Open brackets must be closed by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
/// 3. Every close bracket has a corresponding open bracket of the same type.
pub struct Solution {}

#[derive(PartialEq)]
pub enum BracketType {
    Round = 0,
    Square = 1,
    Curly = 2,
}

pub struct Bracket {
    btype: BracketType,
    open: bool,
}

impl Bracket {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '(' => Ok(Self {
                btype: BracketType::Round,
                open: true,
            }),
            ')' => Ok(Self {
                btype: BracketType::Round,
                open: false,
            }),

            '[' => Ok(Self {
                btype: BracketType::Square,
                open: true,
            }),
            ']' => Ok(Self {
                btype: BracketType::Square,
                open: false,
            }),

            '{' => Ok(Self {
                btype: BracketType::Curly,
                open: true,
            }),
            '}' => Ok(Self {
                btype: BracketType::Curly,
                open: false,
            }),
            _ => Err("could not parse".to_string()),
        }
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // let mut open_count = [0; 3];
        let mut next_needed_close = <Vec<BracketType>>::new();
        for (i, c) in s.chars().enumerate() {
            // We need to close more brackets than there are remaining.
            if next_needed_close.len() > s.len() - i {
                return false;
            }
            let b = Bracket::from_char(c).expect("could not parse");
            // Opening more brackets.
            if b.open {
                next_needed_close.push(b.btype);
            }
            // Closing a bracket.
            else if let Some(needed_close) = next_needed_close.pop() {
                // We are closing with a bracket of the wrong type.
                if needed_close != b.btype {
                    return false;
                }
            // We are closing a bracket before opening one.
            } else {
                return false;
            }
        }
        next_needed_close.is_empty()
    }
}
fn main() {}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0020() {
        assert!(Solution::is_valid("()".to_string()));
        assert!(Solution::is_valid("()[]{}".to_string()));
        assert!(!Solution::is_valid("(]".to_string()));
        assert!(!Solution::is_valid("([)]".to_string()));
    }
}
