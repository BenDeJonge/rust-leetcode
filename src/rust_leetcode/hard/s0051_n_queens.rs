//! https://leetcode.com/problems/n-queens/
//! Hard - [array, backtracking]
//! The n-queens puzzle is the problem of placing n queens on an n x n chessboar
//! such that no two queens attack each other.
//! Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
//! Each solution contains a distinct board configuration of the n-queens' placement,
//! where 'Q' and '.' both indicate a queen and an empty space, respectively.

pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n_usize = n as usize;
        let mut result = <Vec<Vec<String>>>::with_capacity(n_usize);
        // Storing their locations as (row, col).
        let mut queens = <Vec<(usize, usize)>>::with_capacity(n_usize);
        Self::dfs(n_usize, &mut queens, (0, 0), &mut result);
        result
    }

    pub fn dfs(
        n: usize,
        queens: &mut Vec<(usize, usize)>,
        current_queen: (usize, usize),
        result: &mut Vec<Vec<String>>,
    ) {
        // We have tried to place a queen in the bottom row ...
        if current_queen.0 == n {
            // ... and it was accepted, meaning we have found a solution.
            if queens.len() == n {
                result.push(Self::board_state_to_vec(queens))
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
            Self::dfs(n, queens, (current_queen.0 + 1, 0), result)
        }
    }

    pub fn is_valid(queens: &[(usize, usize)], current_queen: (usize, usize)) -> bool {
        !queens.iter().any(|queen| {
            // Same row.
            queen.0 == current_queen.0
            // Same col.
			|| queen.1 == current_queen.1
            // Same diagonal.
			|| (queen.0 as i32 - current_queen.0 as i32).abs() == (queen.1 as i32 - current_queen.1 as i32).abs()
        })
    }

    pub fn board_state_to_vec(queens: &[(usize, usize)]) -> Vec<String> {
        queens
            .iter()
            .map(|queen| {
                let mut s = String::with_capacity(queens.len());
                s.push_str(&".".repeat(queen.1));
                s.push('Q');
                s.push_str(&".".repeat(queens.len() - queen.1 - 1));
                s
            })
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0052() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ]
        )
    }
}
