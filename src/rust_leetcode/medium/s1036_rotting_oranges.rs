//! https://leetcode.com/problems/rotting-oranges/
//! Medium - [array, breadth-first-search, matrix]
//!
//! You are given an m x n grid where each cell can have one of three values:
//! - 0 representing an empty cell,
//! - 1 representing a fresh orange, or
//! - 2 representing a rotten orange.
//!
//! Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
//! Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
//!
//! Example 1:
//! Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
//! Output: 4
//! Example 2:
//! Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
//! Output: -1
//! Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
//! Example 3:
//! Input: grid = [[0,2]]
//! Output: 0
//! Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
//!
//! Constraints:
//! - m == grid.length
//! - n == grid[i].length
//! - 1 <= m, n <= 10
//! - grid[i][j] is 0, 1, or 2.

/// Represent a fruit basket
/// ```text
/// [ 2 1 1 ]                 [ 0 1 2 ]
/// [ 1 1 0 ] with bit counts [ 3 4 5 ]
/// [ 0 1 1 ]                 [ 6 7 8 ]
/// ```
/// as
/// - rotten: `0b000_000_001`
/// - fresh: `0b110_011_111`.
#[derive(Debug)]
struct FruitBasket {
    rows: usize,
    cols: usize,
    rotten: u128,
    fresh: u128,
}

impl FruitBasket {
    /// Get a bitmask of where the rotten oranges can spread.
    fn spread(&self) -> u128 {
        self.rotten | self.spread_horizontal() | self.spread_vertical()
    }

    /// Avoid rows wrapping around.
    ///  [ 1 1 0 ]      [ 1 1 1 ]      [ 0 1 1 ]
    ///  [ 1 1 0 ]  <<  [ 0 1 1 ]  >>  [ 1 0 1 ]
    ///  [ 0 0 0 ]      [ 0 0 0 ]      [ 1 0 0 ]
    /// 000 011 011 << 000 110 111 >> 001 101 110
    fn spread_horizontal(&self) -> u128 {
        let mut rows = 0;
        for r in 0..self.rows {
            let mask = get_row_mask(r, self.cols);
            let row = self.rotten & mask;
            rows |= (mask & (row << 1)) | (mask & (row >> 1));
        }
        rows
    }

    /// Less care can be taken here:
    /// - upwards (to LSB) out-of-bounds cannot go below 0
    /// - downwards (to MSB) out-of-bounds goes in unused bits.
    fn spread_vertical(&self) -> u128 {
        self.rotten << self.cols | self.rotten >> self.cols
    }

    fn rot(&mut self) -> bool {
        // Can only spread to where there is an orange.
        let spread = self.spread() & self.fresh;
        let done = self.rotten == spread;
        self.rotten = spread;
        done
    }

    pub fn count(&mut self) -> Option<usize> {
        let mut i = 1usize;
        while !self.rot() {
            i += 1;
        }

        // In the last step we discover that the rot did not spread further.
        // Hence, it is removed from the count (- 1).
        if self.fresh & self.rotten == self.fresh {
            return Some(i - 1);
        }
        None
    }
}

impl From<Vec<Vec<i32>>> for FruitBasket {
    fn from(value: Vec<Vec<i32>>) -> Self {
        let rows = value.len();
        let cols = value.first().unwrap().len();

        let mut fresh = 0u128;
        let mut rotten = 0u128;
        for (r, row) in value.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                match col {
                    0 => continue,
                    1 => add_to_basket(&mut fresh, r, c, cols),
                    2 => add_to_basket(&mut rotten, r, c, cols),
                    _ => unreachable!(),
                }
            }
        }

        // Include the rotten oranges in the fresh ones,
        // as this will determine where the rot can spread,
        // which includes the first (already rotten) ones.
        fresh |= rotten;

        Self {
            rows,
            cols,
            rotten,
            fresh,
        }
    }
}

fn get_row_mask(i: usize, cols: usize) -> u128 {
    let row = u128::MAX >> (128 - cols);
    row << (i * cols)
}

fn add_to_basket(basket: &mut u128, row: usize, col: usize, cols: usize) {
    *basket += 1 << (col + (row * cols))
}

pub struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        FruitBasket::from(grid)
            .count()
            .map(|c| c as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_leetcode::medium::s1036_rotting_oranges::add_to_basket;

    use super::Solution;

    #[test]
    fn test_add_to_basket() {
        let cols = 2usize;
        let mut i = 0;

        for row in 0..3usize {
            for col in 0..cols {
                let mut basket = 0;
                add_to_basket(&mut basket, row, col, cols);
                assert_eq!(basket, 1 << i);
                i += 1;
            }
        }
    }

    #[test]
    fn test_1036() {
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
    }
}
