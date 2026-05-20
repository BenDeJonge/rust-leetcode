//! <https://leetcode.com/problems/pascals-triangle/>
//! Easy - [array, dynamic programming]
//! Given an integer numRows, return the first numRows of Pascal's triangle.
//! In Pascal's triangle, each number is the sum of the two numbers directly above it.

pub struct Solution {}

impl Solution {
    pub fn generate(num_rows: usize) -> Vec<Vec<usize>> {
        // Initialize a triangle.
        let mut triangle = Vec::with_capacity(num_rows);
        if num_rows == 0 {
            return triangle;
        }
        // Add the first row.
        triangle.push(vec![1]);
        for i in 1..num_rows {
            // Get a reference to the previous row.
            let previous = triangle.last().unwrap();
            // Instantiate the next row as all 1s.
            let mut new = vec![1; i + 1];
            // Each j-th number, except the first and last, is the sum of its two neighbors in the previous row.
            for j in 1..i {
                new[j] = previous[j - 1] + previous[j];
            }
            triangle.push(new);
        }
        triangle
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    #[test]
    fn test_0118() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
