//! https://leetcode.com/problems/kth-largest-element-in-an-array/
//! Medium - [array, divide-and-conquer, sorting, heap-priority-queue, quickselect]
//!
//! Given an integer array nums and an integer k, return the kth largest element in the array.
//! Note that it is the kth largest element in the sorted order, not the kth distinct element.
//! Can you solve it without sorting?
//!
//! Example 1:
//! Input: nums = [3,2,1,5,6,4], k = 2
//! Output: 5
//! Example 2:
//! Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
//! Output: 4
//!
//! Constraints:
//! - 1 <= k <= nums.length <= 105
//! - -104 <= nums[i] <= 104

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        Self::find_kth_largest_ref(&nums, k as usize)
    }

    /// Solve in O(n log(k)).
    ///
    /// Create a min heap of the k largest elements.
    pub fn find_kth_largest_ref<T: Ord + Copy>(nums: &[T], k: usize) -> T {
        // Put the first k elements on the heap, assuming these are the largest.
        let mut heap = BinaryHeap::from_iter(nums.iter().take(k).map(Reverse));

        // Replace the heap's smallest item with the remaining larger ones.
        for item in nums.iter().skip(k) {
            if item > heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(item));
            }
        }
        *heap.pop().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0215() {
        assert_eq!(Solution::find_kth_largest_ref(&[3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest_ref(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
