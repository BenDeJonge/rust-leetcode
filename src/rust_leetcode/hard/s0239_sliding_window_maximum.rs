//! <https://leetcode.com/problems/sliding-window-maximum/>
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
//! - 1 <= nums.length <= 10**5
//! - -10**4 <= nums[i] <= 10**4
//! - 1 <= k <= nums.length

use std::collections::{BTreeSet, VecDeque};

/// Fields are compared in order when deriving (Partial)Ord.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Number<T> {
    val: T,
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
    pub fn max_sliding_window_btreeset<T: Ord + Copy>(nums: &[T], k: usize) -> Vec<T> {
        let mut set: BTreeSet<Number<T>> = nums
            .iter()
            .take(k)
            .enumerate()
            .map(|(i, val)| Number { val: *val, i })
            .collect();

        let n = nums.len() + 1 - k;
        let mut max: Vec<T> = Vec::with_capacity(n);
        max.push(set.last().unwrap().val);

        for left in 0..n - 1 {
            set.remove(&Number {
                val: nums[left],
                i: left,
            });

            let right = left + k;
            set.insert(Number {
                val: nums[right],
                i: right,
            });

            max.push(set.last().unwrap().val);
        }

        max
    }

    /// Solves in O(n * k) time: check k numbers in (n + 1 - k) windows.
    pub fn max_sliding_window_naive<T: Ord + Copy>(nums: &[T], k: usize) -> Vec<T> {
        nums.windows(k)
            .map(|window| window.iter().max().unwrap())
            .copied()
            .collect()
    }

    /// Leetcode is still stuck on some version <1.93.0.
    pub fn max_sliding_window_leetcode<T: Ord + Copy>(nums: &[T], k: usize) -> Vec<T> {
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
        helper(&[1, 3, -1, -3, 5, 3, 6, 7], 3, &[3, 3, 5, 5, 6, 7]);
        helper(&[1], 1, &[1]);
    }

    fn helper(nums: &[i32], k: usize, expected: &[i32]) {
        assert_eq!(Solution::max_sliding_window_naive(nums, k), expected);
        assert_eq!(Solution::max_sliding_window(nums, k), expected);
        assert_eq!(Solution::max_sliding_window_btreeset(nums, k), expected);
    }
}
