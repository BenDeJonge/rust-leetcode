//! https://leetcode.com/problems/jump-game-iv/
//! Hard - [array, hash-table, breadth-first-search]
//! Given an array of integers `arr`, you are initially positioned at the first
//! index of the array. In one step you can jump from index `i` to index:
//! - `i + 1` where`i + 1 < arr.length`.
//! - `i - 1` where `i - 1 >= 0`.
//! - `j` where: `arr[i] == arr[j] and i != j`.
//!
//! Return the minimum number of steps to reach the last index of the array.
//! Notice that you can not jump outside of the array at any time.
//!
//! Example 1:
//! Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
//! Output: 3
//! Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9.
//! Note that index 9 is the last index of the array.
//! Example 2:
//! Input: arr = [7]
//! Output: 0
//! Explanation: Start index is the last index. You do not need to jump.
//! Example 3:
//! Input: arr = [7,6,9,6,9,6,9,7]
//! Output: 1
//! Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
//! &nbsp;
//! Constraints:
//! - `1 <= arr.length <= 5 * 10^4`
//! - `-10^8 <= arr[i] <= 10^8`

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        // Track all vertices of each number.
        let mut occurrences =
            arr.iter()
                .enumerate()
                .fold(HashMap::<i32, Vec<usize>>::new(), |mut acc, (i, &int)| {
                    acc.entry(int).or_default().push(i);
                    acc
                });

        // Track which vertices have been visited.
        let mut visited = vec![false; arr.len()];
        visited[0] = true;

        // The current and next edges to explore.
        let mut current = vec![0];
        let mut next = vec![];

        // Loop over the array ...
        for step in 0.. {
            // BFS: evaluate all neighboring nodes.
            for i in current.drain(..) {
                // We have reached the end position.
                if i == arr.len() - 1 {
                    return step;
                }
                // By getting the neighboring vertices on demand, instead of
                // constructing the full graph in advance, the memory footprint
                // is much smaller.
                // Edges are reachable if they have the same value ...
                for j in occurrences
                    .remove(&arr[i])
                    .unwrap_or(Vec::new())
                    .into_iter()
                    // ... or are neigboring.
                    .chain(vec![i.saturating_sub(1), i + 1].into_iter())
                {
                    if visited[j] {
                        continue;
                    }
                    next.push(j);
                    visited[j] = true;
                }
            }
            std::mem::swap(&mut current, &mut next);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1447() {
        assert_eq!(
            Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
            3
        );
        assert_eq!(Solution::min_jumps(vec![7]), 0);
        assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    }
}
