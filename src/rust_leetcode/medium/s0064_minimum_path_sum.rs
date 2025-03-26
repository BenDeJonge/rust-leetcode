//! https://leetcode.com/problems/minimum-path-sum/
//! Medium - [array, dynamic-programming, matrix]
//!
//! Given a `m x n` grid filled with non-negative numbers,
//! find a path from top left to bottom right,
//! which minimizes the sum of all numbers along its path.
//! Note: You can only move either down or right at any point in time.
//!
//! Example 1:
//! Input: `grid = [[1,3,1],[1,5,1],[4,2,1]]`
//! Output: `7`
//! Explanation: Because the path `1 -> 3 -> 1 -> 1 -> 1` minimizes the sum.
//!
//! Example 2:
//! Input: `grid = [[1,2,3],[4,5,6]]`
//! Output: `12`
//!
//! Constraints:
//! - `m == grid.length`
//! - `n == grid[i].length`
//! - `1 <= m, n <= 200`
//! - `0 <= grid[i][j] <= 200`

pub type Coord = (usize, usize);

pub struct Solution {}

impl Solution {
    /// A dynamic programming solution to find the cheapest path from `(0, 0)`
    /// to `(i, j)` by calculating the cheapest path to each position in the
    /// array from the principal axes.
    ///
    /// ```text
    /// |  1  2  3  4  5  6 |    |  1  3  6 10 15 21 |    |  1  3  6 10 15 21 |
    /// |  7  8  9 10 11 12 | -> |  8  .  .  .  .  . | -> |  8 11 15 20 26 33 |
    /// | 13 14 15 16 17 18 |    | 21  .  .  .  .  . |    | 21 25 30 36 43 51 |
    /// ```
    fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let shape = (grid.len(), grid[0].len());

        // Calculate the rolling sum of the first column ...
        for row in 1..shape.0 {
            grid[row][0] += grid[row - 1][0];
        }
        // ... and first row.
        for col in 1..shape.1 {
            grid[0][col] += grid[0][col - 1];
        }

        // Calculate the cheapest path to each position.
        for row in 1..shape.0 {
            for col in 1..shape.1 {
                grid[row][col] += grid[row - 1][col].min(grid[row][col - 1])
            }
        }
        grid[shape.0 - 1][shape.1 - 1]
    }

    /// A dynamic programming solution to find the cheapest path from `(0, 0)`
    /// to `(i, j)` in `O(2(i + j))` by monitoring the cost to reach each position.
    pub fn min_path_sum_recursive(grid: Vec<Vec<i32>>) -> i32 {
        let curr_coord = (0, 0);
        let curr_cost = 0;
        let mut min_cost = i32::MAX;
        let mut costs = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

        Self::min_path_sum_recursive_helper(
            &grid,
            curr_coord,
            curr_cost,
            &mut min_cost,
            &mut costs,
        );
        min_cost
    }

    fn min_path_sum_recursive_helper(
        grid: &[Vec<i32>],
        curr_coord: Coord,
        curr_cost: i32,
        min_cost: &mut i32,
        costs: &mut [Vec<i32>],
    ) {
        let shape = (grid.len(), grid[0].len());
        let new_sum = curr_cost + Self::get_by_coord(grid, curr_coord);

        let cost = Self::get_mut_by_coord(costs, curr_coord);
        if *cost <= curr_cost {
            return;
        }
        *cost = curr_cost;

        let dests = Self::get_destinations(curr_coord, shape);
        if dests.is_empty() {
            if new_sum < *min_cost {
                *min_cost = new_sum;
            }
            return;
        }
        for dest in dests {
            Self::min_path_sum_recursive_helper(grid, dest, new_sum, min_cost, costs);
        }
    }

    fn get_by_coord<T>(grid: &[Vec<T>], coord: Coord) -> &T {
        &grid[coord.0][coord.1]
    }

    fn get_mut_by_coord<T>(grid: &mut [Vec<T>], coord: Coord) -> &mut T {
        &mut grid[coord.0][coord.1]
    }

    fn get_destinations(coord: Coord, shape: Coord) -> Vec<Coord> {
        let mut vec = Vec::new();
        if coord.0 < shape.0 - 1 {
            vec.push((coord.0 + 1, coord.1));
        }
        if coord.1 < shape.1 - 1 {
            vec.push((coord.0, coord.1 + 1))
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0064() {
        // | 1  3  1 |      | 1  3  1 |
        // | 1  5  1 |  ->  | .  .  1 |
        // | 4  2  1 |      | .  .  1 |
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        // | 1  2  3 |      | 1  2  3 |
        // | 4  5  6 |  ->  | .  .  1 |
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
