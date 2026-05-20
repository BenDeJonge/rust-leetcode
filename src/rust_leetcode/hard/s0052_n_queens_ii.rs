//! <https://leetcode.com/problems/n-queens-ii/>
//! Hard - [backtracking]
//! The n-queens puzzle is the problem of placing n queens on an n x n chessboar
//!  such that no two queens attack each other.
//! Given an integer n, return the number of distinct solutions to the n-queens puzzle.

pub struct Solution {}

impl Solution {
    pub fn total_n_queens(n: usize) -> usize {
        let mut result = 0;
        // Storing their locations as (row, col).
        let mut queens = <Vec<(usize, usize)>>::with_capacity(n);
        Self::dfs(n, &mut queens, (0, 0), &mut result);
        result
    }

    pub fn dfs(
        n: usize,
        queens: &mut Vec<(usize, usize)>,
        current_queen: (usize, usize),
        result: &mut usize,
    ) {
        // We have tried to place a queen in the bottom row ...
        if current_queen.0 == n {
            // ... and it was accepted, meaning we have found a solution.
            if queens.len() == n {
                *result += 1;
            }
            // Either way, we can backtrack.
            return;
        }
        if Self::is_valid(queens, current_queen) {
            // We can save the position of the new queen.
            queens.push(current_queen);
            // Recursively explore the positions of the next queens.
            Self::dfs(n, queens, (current_queen.0 + 1, 0), result);
            // We have explored all solutions with this new queen. Let's backtrack.
            queens.pop();
        }
        // We can add another queen in the current row ...
        if current_queen.1 < n - 1 {
            Self::dfs(n, queens, (current_queen.0, current_queen.1 + 1), result);
        }
        // ... or in the next row.
        else {
            Self::dfs(n, queens, (current_queen.0 + 1, 0), result);
        }
    }

    pub fn is_valid(queens: &[(usize, usize)], current_queen: (usize, usize)) -> bool {
        !queens.iter().any(|queen| {
            // Same row.
            queen.0 == current_queen.0
            // Same col.
			|| queen.1 == current_queen.1
            // Same diagonal.
			|| Self::delta(queen.0, current_queen.0) == Self::delta(queen.1, current_queen.1)
        })
    }

    fn delta(a: usize, b: usize) -> usize {
        a.max(b) - a.min(b)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const SOLUTIONS: [usize; 9] = [1, 0, 0, 2, 10, 4, 40, 92, 352];

    #[test]
    fn test_0052() {
        for (i, sol) in SOLUTIONS.into_iter().enumerate() {
            assert_eq!(Solution::total_n_queens(i + 1), sol);
        }
    }
}
