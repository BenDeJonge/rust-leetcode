//! https://leetcode.com/problems/search-a-2d-matrix/
//! Medium - [array, binary-search, matrix]
//!
//! You are given an `m x n` integer matrix matrix with these properties:
//! - Each row is sorted in non-decreasing order.
//! - The first integer of each row is greater than the last integer of the previous row.
//!
//! Given an integer target, return true if target is in matrix or false otherwise.
//! You must write a solution in `O(log(m * n))` time complexity.
//!
//! Example 1:
//! - Input: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3`
//! - Output: `true`
//!
//! Example 2:
//! - Input: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13`
//! - Output: `false`
//!
//! Constraints:
//! - ``m == matrix.length`
//! - ``n == matrix[i].length`
//! - ``1 <= m, n <= 100`
//! - ``-10^4 <= matrix[i][j], target <= 10^4`

use std::cmp::Ordering;

pub type Coord = (usize, usize);

pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Self::search_matrix_generic(&matrix, target)
    }

    /// A standard binary search where we pretend the 2D matrix is a 1D vector
    /// by transforming the coordinates. The transformation mimics a 1D vector
    /// of length `m * n`, hence, the time complexity is `O_t(log(m * n))`.
    pub fn search_matrix_generic<T>(matrix: &[Vec<T>], target: T) -> bool
    where
        T: Ord,
    {
        let shape: Coord = (matrix.len(), matrix[0].len());
        let mut low = 0;
        let mut high = (shape.0 * shape.1) - 1;
        while low <= high {
            let mid = low + (high - low) / 2;
            let linear = Self::index_1d_to_2d(mid, &shape).expect("low and high are within bounds");
            let value = &matrix[linear.0][linear.1];
            match value.cmp(&target) {
                Ordering::Less => low = mid + 1,
                Ordering::Equal => return true,
                Ordering::Greater => match mid.checked_sub(1) {
                    Some(h) => high = h,
                    None => return false,
                },
            }
        }
        false
    }

    /// Handles the transformation from a 1D to a 2D index.
    /// E.g., for a `(4, 5)` matrix:
    /// ```text
    /// [  0  1  2  3  4 ]    [ (0,0) (0,1) (0,2) (0,3) (0,4) ]
    /// [  5  6  7  8  9 ] -> [ (1,0) (1,1) (1,2) (1,3) (1,4) ]
    /// [ 10 11 12 13 14 ]    [ (2,0) (2,1) (2,2) (2,3) (2,4) ]
    /// [ 15 16 17 18 19 ]    [ (3,0) (3,1) (3,2) (3,3) (3,4) ]
    /// ```
    fn index_1d_to_2d(idx: usize, shape: &Coord) -> Option<Coord> {
        let tmp = (idx / shape.1, idx % shape.1);
        if tmp.0 < shape.0 && tmp.1 < shape.1 {
            Some(tmp)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0074() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![7, 8, 9, 10, 11],
            vec![13, 14, 15, 16, 17],
            vec![19, 20, 21, 22, 23],
        ];
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                assert!(Solution::search_matrix_generic(&matrix, matrix[row][col]))
            }
        }
        for value in [0, 6, 12, 18, 24] {
            assert!(!Solution::search_matrix_generic(&matrix, value));
        }
    }
}
