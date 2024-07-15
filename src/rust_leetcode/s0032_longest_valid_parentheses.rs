// https://leetcode.com/problems/longest-valid-parentheses/
/// Hard - [string, dynamic programming, stack]
/// Given a string containing just the characters '(' and ')'
/// return the length of the longest valid (well-formed) parentheses substring.

#[derive(PartialEq, Eq)]
pub enum Parenthesis {
    Open,
    Closed,
}

pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // In case of all open brackets.
        let mut stack = <Vec<i32>>::with_capacity(s.len());
        // Already push a closed bracket at the (fictitious) -1-th position.
        stack.push(-1);
        let mut best = 0;
        for (i, c) in s.chars().enumerate() {
            let i_i32 = i as i32;
            match c {
                '(' => stack.push(i_i32),
                ')' => {
                    // Whenever we encounter a closing bracket, we remove the latest index.
                    stack.pop();
                    // Opening braces only add to the stack, while closing braces keep the stack constant.
                    // So, if the stack is not empty, we must have some brace being closed.
                    if let Some(last) = stack.last() {
                        // If the previous brace was open, we can add to the best.
                        // If the previous brace was closed, the difference is just 1, which will not replace the best.
                        best = best.max(i_i32 - last);
                    }
                    // There is no match, so we add the closing brace.
                    // This will be removed again if there follows another closing brace.
                    else {
                        stack.push(i_i32);
                    }
                }
                _ => unreachable!("expect string of only '(' and ')'"),
            }
        }
        best
    }

    pub fn longest_valid_parentheses_naive(s: String) -> i32 {
        // Create a stack where we can push the latest parenthesis ...
        let mut stack = <Vec<(usize, Parenthesis)>>::new();
        // ... and a boolean array where true reflects a valid parentheses pair ...
        let mut match_arr = vec![false; s.len()];
        // ... and the maximal length.
        for (i, c) in s.chars().enumerate() {
            match c {
                // This opens a new pair, so add it to the stack.
                '(' => stack.push((i, Parenthesis::Open)),
                // This may or may not close a pair.
                ')' => {
                    // There are values on the stack.
                    if let Some((j, p)) = stack.last() {
                        // The i-th parenthesis closes the j-th one, so we mark both i and j as matching.
                        // We shorten the stack by popping the j-th one and not adding the i-th one.
                        if *p == Parenthesis::Open {
                            match_arr[i] = true;
                            match_arr[*j] = true;
                            stack.pop();
                        }
                        // This does not close a pair of parentheses, so we add the closed one.
                        else {
                            stack.push((i, Parenthesis::Closed))
                        }
                    }
                    // The stack was empty so just add this closed one.
                    else {
                        stack.push((i, Parenthesis::Closed))
                    }
                }
                _ => unreachable!("expect string of only '(' and ')'"),
            }
        }
        // If there are no unmatched parentheses remaining, the whole string was valid.
        if stack.is_empty() {
            return s.len() as i32;
        }
        // Now we have a boolean array where sections of true reflect valid parentheses.
        // We need to find the longest true section in this array.
        let mut current = 0;
        let mut best = 0;
        for &i in match_arr.iter() {
            if i {
                current += 1;
            } else {
                best = best.max(current);
                current = 0;
            }
        }
        best = best.max(current);
        best
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0032() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
