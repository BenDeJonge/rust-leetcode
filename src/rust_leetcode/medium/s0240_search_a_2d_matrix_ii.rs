//! https://leetcode.com/problems/search-a-2d-matrix-ii/
//! Medium - [array, binary-search, divide-and-conquer, matrix]
//!
//! Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. This matrix has the following properties:
//! - Integers in each row are sorted in ascending from left to right.
//! - Integers in each column are sorted in ascending from top to bottom.
//!
//! Example 1:
//! Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
//! Output: true
//! Example 2:
//! Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
//! Output: false
//!
//! Constraints:
//! - m == matrix.length
//! - n == matrix[i].length
//! - 1 <= n, m <= 300
//! - -10**9 <= matrix[i][j] <= 10**9
//! - All the integers in each row are sorted in ascending order.
//! - All the integers in each column are sorted in ascending order.
//! - -10**9 <= target <= 10**9

use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    /// Solve in O(c + r)
    ///
    /// The top value of each column is the smallest element in that column.
    /// So, if this element is already too large, that column can be ignored.
    /// Similarly, the last element of each row is the largest available.
    /// If this is too small, that row can be skipped.
    pub fn search_matrix<T: Ord>(matrix: &[Vec<T>], target: T) -> bool {
        let mut r = 0;
        let mut col = Some(matrix[0].len() - 1);
        while r < matrix.len()
            && let Some(c) = col
        {
            match matrix[r][c].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => col = c.checked_sub(1),
                Ordering::Less => r += 1,
            }
        }
        false
    }

    /// This was not further optimized because the best possible runtime is:
    /// O(max(r,c) + log(c) + log(r))
    /// because:
    /// - we are linearly looping over at most the longest dimension
    ///   (this cannot be improved as skipping over a row or column might
    ///   give an unuseable greater value.)
    /// - we are executing a binary search along the row
    /// - we could execute a binary search along the column
    ///   (this is currently not done)
    pub fn search_matrix_naive(matrix: &[Vec<i32>], target: i32) -> bool {
        let n_rows = matrix.len() - 1;
        let n_cols = matrix[0].len() - 1;
        let largest = matrix[n_rows][n_cols];
        match largest.cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => return false,
            Ordering::Greater => {}
        }

        let mut r = 0;
        let mut c = 0;

        while r <= n_rows && c <= n_cols {
            let corner = matrix[r][c];
            match corner.cmp(&target) {
                Ordering::Equal => {
                    return true;
                }
                // This could be a binary search.
                Ordering::Less => {
                    r += 1.min(n_rows);
                    c += 1.min(n_cols);
                }
                Ordering::Greater => {
                    return matrix[r]
                        .binary_search(&target)
                        .ok()
                        // This could be a binary search.
                        .or_else(|| matrix.iter().position(|row| row[c] == target))
                        .is_some();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0240() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::search_matrix(&matrix, 5));
        assert!(Solution::search_matrix(&matrix, 8));
        assert!(Solution::search_matrix(&matrix, 12));
        assert!(!Solution::search_matrix(&matrix, 20));
        assert!(!Solution::search_matrix(&matrix, 31));

        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        assert!(Solution::search_matrix(&matrix, 19));
    }
}
