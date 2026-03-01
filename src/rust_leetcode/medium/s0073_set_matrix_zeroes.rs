//! https://leetcode.com/problems/set-matrix-zeroes/
//! Medium - [array, hash-table, matrix]
//!
//! Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
//! You must do it in place.
//!
//! Example 1:
//! Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
//! Output: [[1,0,1],[0,0,0],[1,0,1]]
//! Example 2:
//! Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
//! Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//!
//! Constraints:
//! - m == matrix.length
//! - n == matrix[0].length
//! - 1 <= m, n <= 200
//! - -231 <= matrix[i][j] <= 231 - 1
//!
//! Follow up:
//! - A straightforward solution using O(mn) space is probably a bad idea.
//! - A simple improvement uses O(m + n) space, but still not the best solution.
//! - Could you devise a constant space solution?

pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let m = matrix.len();
        let n = matrix[0].len();
        let target = 0;

        // Store the initial state before modifying the storage row/col.
        let flip_row = matrix[0].contains(&target);
        let flip_col = matrix.iter().any(|row| row[0] == target);

        // Store the rows/columns that need to be flipped in the first row/col.
        // Assigning there is fine as those rows/cols will be flipped later.
        for r in 0..m {
            for c in 0..n {
                if matrix[r][c] == target {
                    matrix[r][0] = target;
                    matrix[0][c] = target;
                }
            }
        }

        // Loop over the storage column and flip all indicated rows,
        // except the storage row, as that is still needed.
        for row in matrix.iter_mut().skip(1) {
            if row[0] == target {
                *row = vec![target; n];
            }
        }

        // Loop over the storage row and flip all indicated columns.
        for c in 1..n {
            if matrix[0][c] == target {
                for row in matrix.iter_mut() {
                    row[c] = target;
                }
            }
        }

        // Flip the storage row/col, if needed.
        if flip_row {
            matrix[0] = vec![target; n];
        }
        if flip_col {
            for row in matrix.iter_mut() {
                row[0] = target;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0073() {
        let matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let solution = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        helper(matrix, solution);

        let matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let solution = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        helper(matrix, solution);

        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];
        let solution = vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        helper(matrix, solution);

        let matrix = vec![vec![1, 0, 3]];
        let solution = vec![vec![0, 0, 0]];
        helper(matrix, solution);

        let matrix = vec![vec![1], vec![0], vec![3]];
        let solution = vec![vec![0], vec![0], vec![0]];
        helper(matrix, solution);

        let matrix = vec![
            vec![-4, -2147483648, 6, -7, 0],
            vec![-8, 6, -8, -6, 0],
            vec![2147483647, 2, -9, -6, -10],
        ];
        let solution = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![2147483647, 2, -9, -6, 0],
        ];
        helper(matrix, solution);
    }

    fn helper(mut matrix: Vec<Vec<i32>>, solution: Vec<Vec<i32>>) {
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, solution);
    }
}
