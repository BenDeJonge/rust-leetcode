/// https://leetcode.com/problems/generate-parentheses/
/// Medium - [string, dynamic programming, backtracking]
/// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

const P_OPEN: char = '(';
const P_CLOSED: char = ')';

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n_usize = n as usize;
        let mut done = <Vec<String>>::with_capacity(2usize.pow(n as u32));
        let mut current = <Vec<char>>::with_capacity(n_usize * 2);
        Solution::dfs(&mut done, &mut current, n_usize, 0, 0);
        done
    }

    /// Depth First Search
    /// Construct a binary tree along these lines (for n = 2)
    /// ```text
    ///                  (0, 0, '')
    ///                      |
    ///                  (1, 0, '(')
    ///                 /         \
    ///      (2, 0, '((')         (1, 1, '()')
    ///               /             \
    ///     (2, 1, '(()')         (2, 1, '()(')
    ///             /                  \
    ///    (2, 2, '(())')         (2, 2, '()()')
    ///            |                   |
    /// done.push('(())')         done.push('()()')
    /// ```
    pub fn dfs(
        done: &mut Vec<String>,
        current: &mut Vec<char>,
        n: usize,
        left: usize,
        right: usize,
    ) {
        // We have complete a full set of parentheses.
        if (left == n) && (right == n) {
            done.push(current.iter().collect());
        }
        // There is room for open parentheses.
        if left < n {
            current.push(P_OPEN);
            Solution::dfs(done, current, n, left + 1, right)
        }
        // There are parentheses to be closed.
        if right < left {
            current.push(P_CLOSED);
            Solution::dfs(done, current, n, left, right + 1)
        }
        // Backtracking.
        current.pop();
    }

    pub fn generate_parenthesis_naive(n: i32) -> Vec<String> {
        let n_usize = n as usize;
        let mut done = <Vec<String>>::with_capacity(2usize.pow(n as u32));
        let mut current = <Vec<char>>::with_capacity(n_usize * 2);
        Solution::generate_parenthesis_helper(&mut done, &mut current, n_usize, 0, 0);
        done
    }

    pub fn generate_parenthesis_helper(
        done: &mut Vec<String>,
        current: &mut Vec<char>,
        n: usize,
        n_open: usize,
        n_closed: usize,
    ) {
        // We have completed a full set of parentheses.
        if (n_open == n) && (n_closed == n) {
            done.push(current.iter().collect());
        }
        // All parenthesis are currently closed, so we have to add an open one.
        else if n_open == n_closed {
            current.push(P_OPEN);
            Solution::generate_parenthesis_helper(done, current, n, n_open + 1, n_closed);
        }
        // We can only close parentheses. The opposite scenario does not exist, as those parentheses would be invalid.
        else if n_open == n {
            current.push(P_CLOSED);
            Solution::generate_parenthesis_helper(done, current, n, n_open, n_closed + 1);
        }
        // We can both open or close a pair of parentheses.
        else {
            current.push(P_OPEN);
            Solution::generate_parenthesis_helper(done, current, n, n_open + 1, n_closed);
            current.push(P_CLOSED);
            Solution::generate_parenthesis_helper(done, current, n, n_open, n_closed + 1);
        }
        // Backtracking.
        current.pop();
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;
    #[test]
    fn test_0022() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }
}
