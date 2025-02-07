//! https://leetcode.com/problems/spiral-matrix/
//! Medium - [array, matrix, simulation]
//! Given an n x m matrix, return all elements in clockwise spiral order.

enum Direction {
    North,
    East,
    South,
    West,
}

type Coord = [usize; 2];

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Is horizontal, aka goes E <-> W.
    fn is_longitudinal(&self) -> bool {
        match self {
            Direction::East | Direction::West => true,
            Direction::North | Direction::South => false,
        }
    }

    /// Is vertical, aka goes N <-> S.
    fn is_latitudinal(&self) -> bool {
        !self.is_longitudinal()
    }

    fn take_step(&self, coord: Coord) -> Option<Coord> {
        match self {
            Direction::North => {
                if coord[0] == 0 {
                    None
                } else {
                    Some([coord[0] - 1, coord[1]])
                }
            }
            Direction::East => Some([coord[0], coord[1] + 1]),
            Direction::South => Some([coord[0] + 1, coord[1]]),
            Direction::West => {
                if coord[1] == 0 {
                    None
                } else {
                    Some([coord[0], coord[1] - 1])
                }
            }
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn spiral_order<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<T> {
        // Ugly but spiral_order should not own the data in matrix.
        Self::spiral_order_slice(&matrix.iter().map(|v| v.as_slice()).collect::<Vec<&[T]>>())
    }

    pub fn spiral_order_slice<T: Copy>(matrix: &[&[T]]) -> Vec<T> {
        let n_rows = matrix.len();
        let n_cols = matrix[0].len();
        let n_elements = n_cols * n_rows;
        let mut ret = <Vec<_>>::with_capacity(n_elements);
        let mut coord = [0, 0];

        let mut offset = 1;
        let mut direction = Direction::East;
        let mut i_direction = 0;
        let mut steps_in_direction;

        while ret.len() < n_elements - 1 {
            // Get the number of steps to take in the wanted direction, which
            // decreases every second iteration because of the spiral.
            // We special case the first 3 steps as these traverse the outer
            // edges of the matrix without being obstructed by previous passes
            // of the spiral path.
            if i_direction > 2 && i_direction % 2 != 0 {
                offset += 1;
            }
            if direction.is_longitudinal() {
                steps_in_direction = n_cols - offset;
            } else {
                steps_in_direction = n_rows - offset;
            }

            for _ in 0..(steps_in_direction) {
                ret.push(matrix[coord[0]][coord[1]]);
                coord = direction.take_step(coord).unwrap();
            }
            direction = direction.rotate_right();
            i_direction += 1;
        }
        // In the loop, we grab the current position and then take a step.
        // So, the last element has to be grabbed separately.
        ret.push(matrix[coord[0]][coord[1]]);
        ret
    }
}

#[cfg(test)]
mod test {

    use std::fmt::Debug;

    use super::Solution;

    fn test_helper<T: Copy + PartialEq + Debug>(matrix: &[&[T]], spiral: &[T]) {
        assert_eq!(Solution::spiral_order_slice(matrix), spiral)
    }

    #[test]
    fn test_spiral_matrix() {
        test_helper(
            &[
                &[1, 2, 3], //
                &[4, 5, 6], //
                &[7, 8, 9], //
            ],
            &[1, 2, 3, 6, 9, 8, 7, 4, 5],
        );
        test_helper(
            &[
                &[1, 2, 3, 4],    //
                &[5, 6, 7, 8],    //
                &[9, 10, 11, 12], //
            ],
            &[1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
        );
    }
}
