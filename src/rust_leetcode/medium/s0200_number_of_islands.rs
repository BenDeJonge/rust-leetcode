//! <https://leetcode.com/problems/number-of-islands/>
//! Medium - [array, depth-first-search, breadth-first-search, union-find, matrix]
//!
//! Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water),
//! return the number of islands.
//! An island is surrounded by water and is formed by connecting adjacent lands
//! horizontally or vertically.
//! You may assume all four edges of the grid are all surrounded by water.
//!
//! Example 1:
//! Input: grid = [
//!   ['1','1','1','1','0'],
//!   ['1','1','0','1','0'],
//!   ['1','1','0','0','0'],
//!   ['0','0','0','0','0']
//! ]
//! Output: 1
//! Example 2:
//! Input: grid = [
//!   ['1','1','0','0','0'],
//!   ['1','1','0','0','0'],
//!   ['0','0','1','0','0'],
//!   ['0','0','0','1','1']
//! ]
//! Output: 3
//!
//! Constraints:
//! - m == grid.length
//! - n == grid[i].length
//! - 1 <= m, n <= 300
//! - grid[i][j] is '0' or '1'.

/// The '0' and '1' values in the input array.
pub(crate) enum Location {
    Water,
    Island,
}

pub(crate) struct InvalidInput(char);

impl TryFrom<char> for Location {
    type Error = InvalidInput;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Water),
            '1' => Ok(Self::Island),
            _ => Err(InvalidInput(value)),
        }
    }
}

impl TryFrom<&char> for Location {
    type Error = InvalidInput;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

/// Tracking the already visited locations, numbering the islands.
#[derive(Clone, PartialEq, Debug)]
enum Visited {
    Water,
    Island(usize),
    Uncharted,
}

pub struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        Self::num_islands_inner(grid).try_into().unwrap()
    }

    /// Solve in O(r * c), with r, c the number of rows and columns of the grid.
    ///
    /// Iterate over all coordinates, exploring the islands with DFS.
    /// Whenever an island has been completely mapped, increment the counter.
    /// This is basically a floodfill algorithm.
    fn num_islands_inner<T>(grid: Vec<Vec<T>>) -> usize
    where
        Location: for<'a> std::convert::TryFrom<&'a T>,
    {
        let shape = [grid.len(), grid[0].len()];
        let mut visited = vec![vec![Visited::Uncharted; shape[1]]; shape[0]];
        let mut islands = 0;

        for row in 0..shape[0] {
            for col in 0..shape[1] {
                if visited[row][col] != Visited::Uncharted {
                    continue;
                }
                Self::dfs(&grid, &mut visited, [row, col], islands);
                if let Visited::Island(_) = visited[row][col] {
                    islands += 1;
                }
            }
        }
        islands
    }

    fn dfs<T>(grid: &[Vec<T>], visited: &mut [Vec<Visited>], coord: [usize; 2], islands: usize)
    where
        Location: for<'a> std::convert::TryFrom<&'a T>,
    {
        let shape = [grid.len(), grid[0].len()];
        let [row, col] = coord;
        if visited[row][col] != Visited::Uncharted {
            return;
        }
        match Self::get_coord(grid, coord).unwrap().try_into() {
            Ok(Location::Water) => visited[row][col] = Visited::Water,
            Ok(Location::Island) => {
                visited[row][col] = Visited::Island(islands);
                for neighbor in Self::neighbors(coord, shape).iter().flatten() {
                    Self::dfs(grid, visited, *neighbor, islands);
                }
            }
            Err(_) => unreachable!("only water and islands"),
        }
    }

    fn neighbors(coord: [usize; 2], shape: [usize; 2]) -> [Option<[usize; 2]>; 4] {
        let top = coord[0].checked_sub(1).map(|r| [r, coord[1]]);
        let right = coord[1].checked_sub(1).map(|c| [coord[0], c]);
        let bottom = coord[0]
            .cmp(&(shape[0] - 1))
            .is_lt()
            .then_some([coord[0] + 1, coord[1]]);
        let left = coord[1]
            .cmp(&(shape[1] - 1))
            .is_lt()
            .then_some([coord[0], coord[1] + 1]);
        [top, right, bottom, left]
    }

    fn get_coord<T>(arr: &[Vec<T>], coord: [usize; 2]) -> Option<&T> {
        arr.get(coord[0]).and_then(|row| row.get(coord[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0200_single_island() {
        assert_eq!(
            Solution::num_islands_inner(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn test_0200_multiple_islands() {
        assert_eq!(
            Solution::num_islands_inner(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }

    #[test]
    fn test_0200_no_islands() {
        assert_eq!(
            Solution::num_islands_inner(vec![
                vec!['0', '0', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            0
        );
    }

    #[test]
    fn test_0200_all_islands() {
        assert_eq!(
            Solution::num_islands_inner(vec![
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1']
            ]),
            1
        );
    }

    #[test]
    fn test_0200_nested_islands() {
        assert_eq!(
            Solution::num_islands_inner(vec![
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '0', '1'],
                vec!['1', '0', '1', '0', '1'],
                vec!['1', '0', '0', '0', '1'],
                vec!['1', '1', '1', '1', '1']
            ]),
            2
        );
    }
}
