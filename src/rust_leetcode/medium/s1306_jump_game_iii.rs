//! https://leetcode.com/problems/jump-game-iii/
//! Medium - [array, depth-first-search, breadth-first-search]
//! Given an array of non-negative integers arr,
//! you are initially positioned at the `start` index of the array.
//! When you are at index `i`, you can jump to `i + arr[i]` or `i - arr[i]`,
//! Check if you can reach any index with value 0.
//! Notice that you can not jump outside of the array at any time.
//!
//! Example 1:
//! Input: arr = [4,2,3,0,3,1,2], start = 5
//! Output: true
//! Explanation:
//! All possible ways to reach at index 3 with value 0 are:
//! index 5 -> index 4 -> index 1 -> index 3
//! index 5 -> index 6 -> index 4 -> index 1 -> index 3
//!
//! Example 2:
//! Input: arr = [4,2,3,0,3,1,2], start = 0
//! Output: true
//! Explanation:
//! One possible way to reach at index 3 with value 0 is:
//! index 0 -> index 4 -> index 1 -> index 3
//!
//! Example 3:
//! Input: arr = [3,0,2,1,2], start = 2
//! Output: false
//! Explanation: There is no way to reach at index 1 with value 0.
//!
//! Constraints:
//! `arr.length <= 5 * 10^4`
//! `arr[i] < arr.length`
//! `start < arr.length`

pub struct Solution {}

const TARGET: i32 = 0;
const HAS_BEEN_VISITED: i32 = -1;

impl Solution {
    /// Poorly typed function signature is required by leetcode.
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        Self::can_reach_ref(&mut arr, start as usize)
    }

    pub fn can_reach_ref(arr: &mut [i32], start: usize) -> bool {
        Self::dfs(arr, start)
    }

    /// Change arr to track previously visited positions.
    /// Every node is visited at most once, so time ~ `O(n)`.
    /// No extra space is required as only arr is used, so space ~ `O(n)`.
    pub fn dfs(arr: &mut [i32], start: usize) -> bool {
        if arr[start] == HAS_BEEN_VISITED {
            return false;
        } else if arr[start] == TARGET {
            return true;
        }
        let step = arr[start] as usize;
        arr[start] = HAS_BEEN_VISITED;

        // Jump to the right.
        (start + step < arr.len() && Self::dfs(arr, start + step))
            // Jump to the left if in range.
            || (start >= step && Self::dfs(arr, start - step))
    }

    // Requires more space to track visited with expensive initial allocation.
    pub fn can_reach_ref_naive(arr: &[usize], start: usize) -> bool {
        let mut visited = vec![false; arr.len()];
        Self::dfs_naive(arr, start, &mut visited)
    }

    pub fn dfs_naive(arr: &[usize], start: usize, visited: &mut [bool]) -> bool {
        if visited[start] {
            return false;
        }
        visited[start] = true;
        (arr[start] == 0)
                // Jump to the right.
                || (start + arr[start] < arr.len() && Self::dfs_naive(arr, start + arr[start], visited))
                // Jump to the left if in range.
                || (start >= arr[start] && Self::dfs_naive(arr, start - arr[start], visited))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1428() {
        assert!(Solution::can_reach_ref(&mut [4, 2, 3, 0, 3, 1, 2], 5));
        assert!(Solution::can_reach_ref(&mut [4, 2, 3, 0, 3, 1, 2], 0));
        assert!(!Solution::can_reach_ref(&mut [3, 0, 2, 1, 2], 2));
    }
}
