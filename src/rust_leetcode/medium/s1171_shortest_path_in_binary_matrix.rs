//! https://leetcode.com/problems/shortest-path-in-binary-matrix/
//! Medium - [array, breadth-first-search, matrix]
//!
//! Given an `n x n` binary matrix grid, return the length of the shortest clear
//! path in the matrix. If there is no clear path, return `-1`.
//! A clear path in a binary matrix is a path from the top-left cell
//! (i.e., `(0, 0)`) to the bottom-right cell (i.e., `(n - 1, n - 1)`) such that:
//! - All the visited cells of the path are 0.
//! - All the adjacent cells of the path are 8-directionally connected
//!   (i.e., they are different and they share an edge or a corner).
//!
//! The length of a clear path is the number of visited cells of this path.
//!
//! Example 1:
//! Input: grid = [[0,1],[1,0]]
//! Output: 2
//! Example 2:
//! Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
//! Output: 4
//! Example 3:
//! Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
//! Output: -1
//!
//! Constraints:
//! - `n == grid.length`
//! - `n == grid[i].length`
//! - `1 <= n <= 100`
//! - `grid[i][j]` is 0 or 1

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    /// Find the shortest path in matrix while travelling in 8 directions
    /// (4 cardinals and 4 diagonals), only visiting nodes of value 0 not 1.
    ///
    /// Time complexity: `O(n)` with `n` the number of nodes.
    /// Auxiliary space complexity: `O(n)` i.e., a grid with distances.
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if let Some(res) = Self::bfs_track_level(grid, 1) {
            res as i32
        } else {
            -1
        }
    }

    /// Track the traveled distance in an additional array that also serves as
    /// a monitor of whether or not a position has been visited.
    fn bfs_track_distance<T: PartialEq>(grid: Vec<Vec<T>>, do_not_visit: T) -> Option<usize> {
        if grid[0][0] == do_not_visit {
            return None;
        }

        let bounds = [grid.len(), grid[0].len()];
        let dest = [grid.len() - 1, grid[0].len() - 1];

        let mut distances = vec![vec![None; grid[0].len()]; grid.len()];
        distances[0][0] = Some(1);
        let mut queue = VecDeque::from([[0usize, 0usize]]);

        'outer: while let Some(coord) = queue.pop_front() {
            let distance = distances[coord[0]][coord[1]].unwrap();

            for neighbor in Self::get_neighbors(coord, bounds) {
                if grid[neighbor[0]][neighbor[1]] == do_not_visit
                    || distances[neighbor[0]][neighbor[1]].is_some_and(|dist| dist <= distance + 1)
                {
                    continue;
                }
                // No need to check if any later path is shorter because in bfs
                // shortest paths are explored first.
                distances[neighbor[0]][neighbor[1]] = Some(distance + 1);
                if neighbor == dest {
                    break 'outer;
                }
                queue.push_back(neighbor);
            }
        }
        distances[dest[0]][dest[1]]
    }

    /// Track the number of steps as an integer by only exploring the current
    /// number of states in the queue. Additional pushes are part of the next
    /// level of BFS. An additional array to monitor visits is still required.
    fn bfs_track_level<T: PartialEq>(grid: Vec<Vec<T>>, do_not_visit: T) -> Option<usize> {
        if grid[0][0] == do_not_visit {
            return None;
        }

        let bounds = [grid.len(), grid[0].len()];
        let dest = [grid.len() - 1, grid[0].len() - 1];

        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue = VecDeque::from([[0usize, 0usize]]);
        let mut steps = 1;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let coord = queue.pop_front().unwrap();
                if visited[coord[0]][coord[1]] {
                    continue;
                }
                visited[coord[0]][coord[1]] = true;

                if coord == dest {
                    return Some(steps);
                }

                for neighbor in Self::get_neighbors(coord, bounds) {
                    if grid[neighbor[0]][neighbor[1]] != do_not_visit
                        && !visited[neighbor[0]][neighbor[1]]
                    {
                        queue.push_back(neighbor);
                    }
                }
            }
            steps += 1;
        }

        None
    }

    fn get_neighbors(coord: [usize; 2], bounds: [usize; 2]) -> Vec<[usize; 2]> {
        let north = coord[0].checked_sub(1).map(|r| [r, coord[1]]);
        let east = if coord[1] < bounds[1] - 1 {
            Some([coord[0], coord[1] + 1])
        } else {
            None
        };
        let south = if coord[0] < bounds[0] - 1 {
            Some([coord[0] + 1, coord[1]])
        } else {
            None
        };
        let west = coord[1].checked_sub(1).map(|c| [coord[0], c]);
        [
            north,
            east,
            south,
            west,
            Self::combine(north, east),
            Self::combine(south, east),
            Self::combine(south, west),
            Self::combine(north, west),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn combine(coord_1: Option<[usize; 2]>, coord_2: Option<[usize; 2]>) -> Option<[usize; 2]> {
        if let (Some(c1), Some(c2)) = (coord_1, coord_2) {
            Some([c1[0], c2[1]])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn get_neighbors() {
        // x . .
        // . . .
        // . . .
        assert_eq!(
            Solution::get_neighbors([0, 0], [3, 3]),
            vec![
                [0, 1], // E
                [1, 0], // S
                [1, 1], // SE
            ]
        );
        // . x .
        // . . .
        // . . .
        assert_eq!(
            Solution::get_neighbors([0, 1], [3, 3]),
            vec![
                [0, 2], // E
                [1, 1], // S
                [0, 0], // W
                [1, 2], // SE
                [1, 0], // SW
            ]
        );
        // . . .
        // . x .
        // . . .
        assert_eq!(
            Solution::get_neighbors([1, 1], [3, 3]),
            vec![
                [0, 1], // N
                [1, 2], // E
                [2, 1], // S
                [1, 0], // W
                [0, 2], // NE
                [2, 2], // SE
                [2, 0], // SW
                [0, 0], // NW
            ]
        );
        // . . .
        // . . .
        // . . x
        assert_eq!(
            Solution::get_neighbors([2, 2], [3, 3]),
            vec![
                [1, 2], // N
                [2, 1], // W
                [1, 1], // NW
            ]
        );
    }

    #[test]
    fn test_1171() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            4
        );
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![
                vec![1, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0]
            ]),
            -1
        );
    }
}
