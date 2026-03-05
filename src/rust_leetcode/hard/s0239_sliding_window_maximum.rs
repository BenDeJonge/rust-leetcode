//! https://leetcode.com/problems/sliding-window-maximum/
//! Hard - [array, queue, sliding-window, heap-priority-queue, monotonic-queue]
//!
//! You are given an array of integers nums,
//! there is a sliding window of size k
//! which is moving from the very left of the array to the very right.
//! You can only see the k numbers in the window.
//! Each time the sliding window moves right by one position.
//! Return the max sliding window.
//!
//! Example 1:
//! Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
//! Output: [3,3,5,5,6,7]
//! Explanation:
//! Window position                Max
//! ---------------               -----
//! [1  3  -1] -3  5  3  6  7       3
//!  1 [3  -1  -3] 5  3  6  7       3
//!  1  3 [-1  -3  5] 3  6  7       5
//!  1  3  -1 [-3  5  3] 6  7       5
//!  1  3  -1  -3 [5  3  6] 7       6
//!  1  3  -1  -3  5 [3  6  7]      7
//! Example 2:
//! Input: nums = [1], k = 1
//! Output: [1]
//!
//! Constraints:
//! - 1 <= nums.length <= 105
//! - -104 <= nums[i] <= 104
//! - 1 <= k <= nums.length

use std::collections::{BTreeSet, VecDeque};

/// Fields are compared in order when deriving (Partial)Ord.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Number {
    val: i32,
    i: usize,
}

pub struct Solution {}

impl Solution {
    /// Solves in O(n) times: put maximum at the front of a Dequeue.
    pub fn max_sliding_window<T: Ord + Copy>(nums: &[T], k: usize) -> Vec<T> {
        let mut idxs: VecDeque<usize> = VecDeque::new();
        let mut max = Vec::with_capacity(nums.len() + 1 - k);

        for right in 0..nums.len() {
            // Remove index too far left of window.
            idxs.pop_front_if(|left| *left + k <= right);

            // When a new number is coming in from the right,
            // all values less than this number can be removed.
            // Since this is done in every loop, the front index
            // points to the largest value in the window.
            while idxs.pop_back_if(|r| nums[*r] < nums[right]).is_some() {}
            idxs.push_back(right);

            // Start adding maxima when the first window is full.
            if right >= k - 1 {
                max.push(nums[*idxs.front().unwrap()]);
            }
        }
        max
    }

    /// Solves in O(n log k) time: get the min and max from a Binary Tree Set.
    pub fn max_sliding_window_btreeset(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k_usize = k as usize;
        let mut set = BTreeSet::from_iter(
            nums.iter()
                .take(k_usize)
                .enumerate()
                .map(|(i, val)| Number { val: *val, i }),
        );

        let n = nums.len() + 1 - k_usize;
        let mut max: Vec<i32> = Vec::with_capacity(n);
        max.push(set.last().unwrap().val);

        for left in 0..n - 1 {
            set.remove(&Number {
                val: nums[left],
                i: left,
            });

            let right = left + k_usize;
            set.insert(Number {
                val: nums[right],
                i: right,
            });

            max.push(set.last().unwrap().val);
        }

        max
    }

    /// Solves in O(n * k) time: check k numbers in (n + 1 - k) windows.
    pub fn max_sliding_window_naive<T: Ord + Clone>(nums: &[T], k: usize) -> Vec<T> {
        nums.windows(k)
            .map(|window| window.iter().max().unwrap())
            .cloned()
            .collect()
    }

    /// Leetcode is still stuck on some version <1.93.0.
    pub fn max_sliding_window_leetcode(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut idxs: VecDeque<usize> = VecDeque::new();
        let mut max = Vec::with_capacity(nums.len() + 1 - k);

        for right in 0..nums.len() {
            if let Some(left) = idxs.front()
                && *left + k <= right
            {
                idxs.pop_front();
            }
            while let Some(r) = idxs.back()
                && nums[*r] < nums[right]
            {
                idxs.pop_back();
            }
            idxs.push_back(right);
            if right >= k - 1 {
                max.push(nums[*idxs.front().unwrap()]);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0239() {
        helper(vec![1, 3, -1, -3, 5, 3, 6, 7], 3, &[3, 3, 5, 5, 6, 7]);
        helper(vec![1], 1, &[1]);
    }

    fn helper(nums: Vec<i32>, k: i32, expected: &[i32]) {
        assert_eq!(
            Solution::max_sliding_window_naive(&nums, k as usize),
            expected
        );
        assert_eq!(Solution::max_sliding_window(&nums, k as usize), expected);
        assert_eq!(Solution::max_sliding_window_btreeset(nums, k), expected);
    }
}
