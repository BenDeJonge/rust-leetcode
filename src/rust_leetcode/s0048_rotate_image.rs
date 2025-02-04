//! https://leetcode.com/problems/rotate-image/
//! Medium - [array, math, matrix]
//! Given a square n x n matrix, rotate it by 90 degrees clockwise in-place.

pub struct Solution {}

impl Solution {
    fn rotate(matrix: &mut [Vec<i32>]) {
        Self::transpose(matrix);
        Self::flip_lr(matrix);
    }

    /// A more generalized three step algorithm by [Catanzaro et al., 2014](
    /// https://research.nvidia.com/sites/default/files/pubs/2014-02_A-Decomposition-for/ppopp2014.pdf
    /// ).
    /// 1. Rotate the columns
    /// 2. Permute the rows.
    /// 3. Permute the columns.
    ///
    /// For a square 2D matrix, nested for loops will suffice.
    fn transpose<T: Copy>(matrix: &mut [Vec<T>]) {
        if matrix.is_empty() {
            return;
        }
        let mut a;
        let mut b;
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                // Do not go beyond the diagonal.
                if c == r {
                    break;
                }
                a = matrix[r][c];
                b = matrix[c][r];
                matrix[r][c] = b;
                matrix[c][r] = a;
            }
        }
    }

    fn flip_lr<T: Copy>(matrix: &mut [Vec<T>]) {
        if matrix.is_empty() {
            return;
        }
        let mut a;
        let mut b;
        let mut idx;
        for row in matrix {
            for c in 0..(row.len()) / 2 {
                idx = row.len() - 1 - c;
                a = row[c];
                b = row[idx];
                row[c] = b;
                row[idx] = a;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fmt::Debug;

    use super::Solution;

    type MatrixTestCase<T> = (Vec<Vec<T>>, Vec<Vec<T>>);

    fn test_helper<T: Copy + PartialEq + Debug>(
        test_cases: Vec<MatrixTestCase<T>>,
        func: fn(&mut Vec<Vec<T>>),
    ) {
        for mut test_case in test_cases {
            func(&mut test_case.0);
            assert_eq!(test_case.0, test_case.1)
        }
    }

    #[test]
    fn test_transpose() {
        test_helper(
            vec![
                // 3 x 3 matrix.
                (
                    vec![
                        vec![1, 2, 3], //
                        vec![4, 5, 6], //
                        vec![7, 8, 9], //
                    ],
                    vec![
                        vec![1, 4, 7], //
                        vec![2, 5, 8], //
                        vec![3, 6, 9], //
                    ],
                ),
                // 4 x 4 matrix.
                (
                    vec![
                        vec![1, 2, 3, 4],     //
                        vec![5, 6, 7, 8],     //
                        vec![9, 10, 11, 12],  //
                        vec![13, 14, 15, 16], //
                    ],
                    vec![
                        vec![1, 5, 9, 13],  //
                        vec![2, 6, 10, 14], //
                        vec![3, 7, 11, 15], //
                        vec![4, 8, 12, 16], //
                    ],
                ),
            ],
            |matrix| Solution::transpose(matrix),
        )
    }

    #[test]
    fn test_flip_lr() {
        test_helper(
            vec![
                // 3 x 3 matrix.
                (
                    vec![
                        vec![1, 2, 3], //
                        vec![4, 5, 6], //
                        vec![7, 8, 9], //
                    ],
                    vec![
                        vec![3, 2, 1], //
                        vec![6, 5, 4], //
                        vec![9, 8, 7], //
                    ],
                ),
                // 4 x 4 matrix.
                (
                    vec![
                        vec![1, 2, 3, 4],     //
                        vec![5, 6, 7, 8],     //
                        vec![9, 10, 11, 12],  //
                        vec![13, 14, 15, 16], //
                    ],
                    vec![
                        vec![4, 3, 2, 1],     //
                        vec![8, 7, 6, 5],     //
                        vec![12, 11, 10, 9],  //
                        vec![16, 15, 14, 13], //
                    ],
                ),
            ],
            |matrix| Solution::flip_lr(matrix),
        );
    }

    #[test]
    fn test_rotate() {
        test_helper(
            vec![
                // 3 x 3 matrix.
                (
                    vec![
                        vec![1, 2, 3], //
                        vec![4, 5, 6], //
                        vec![7, 8, 9], //
                    ],
                    vec![
                        vec![7, 4, 1], //
                        vec![8, 5, 2], //
                        vec![9, 6, 3], //
                    ],
                ),
                // 4 x 4 matrix.
                (
                    vec![
                        vec![1, 2, 3, 4],     //
                        vec![5, 6, 7, 8],     //
                        vec![9, 10, 11, 12],  //
                        vec![13, 14, 15, 16], //
                    ],
                    vec![
                        vec![13, 9, 5, 1],  //
                        vec![14, 10, 6, 2], //
                        vec![15, 11, 7, 3], //
                        vec![16, 12, 8, 4], //
                    ],
                ),
            ],
            |matrix| Solution::rotate(matrix),
        );
    }
}
