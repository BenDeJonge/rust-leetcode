//! <https://leetcode.com/problems/top-k-frequent-elements/>
//! Medium - [array, hash-table, divide-and-conquer, sorting, heap-priority-queue, bucket-sort, counting, quickselect]
//!
//! Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
//!
//! Example 1:
//! Input: nums = [1,1,1,2,2,3], k = 2
//! Output: [1,2]
//! Example 2:
//! Input: nums = [1], k = 1
//! Output: [1]
//! Example 3:
//! Input: nums = [1,2,1,2,1,2,3,1,3,2], k = 2
//! Output: [1,2]
//!
//! Constraints:
//! - 1 <= nums.length <= 10**5
//! - -10**4 <= nums[i] <= 10**4
//! - k is in the range [1, the number of unique elements in the array].
//! - It is guaranteed that the answer is unique.
//!
//! Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub struct Solution {}

#[derive(PartialEq, Eq, Debug)]
struct CountedVal<T: Ord> {
    pub val: T,
    pub count: usize,
}

impl<T: Ord> Ord for CountedVal<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl<T: Ord> PartialOrd for CountedVal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    /// Time complexity: O(n + mlog(k))
    /// - construct a hashmap by looping over all n elements
    /// - construct a binary heap by popping the m unique elements
    /// - pop from a binary heap of k elements
    ///
    /// The worst case scenario is:
    /// - n = m: every element in the list is unique, leading to a large map
    /// - k = n: a large heap needs to be constructed.
    ///
    /// The complexity is then O(n + nlog(n)) = O(nlog(n)).
    pub fn top_k_frequent<T: Ord + Copy + Hash>(nums: &[T], k: usize) -> Vec<T> {
        let mut counts = nums.iter().fold(HashMap::new(), |mut acc, num| {
            acc.entry(*num).and_modify(|count| *count += 1).or_insert(1);
            acc
        });
        let mut min_heap = BinaryHeap::with_capacity(k);
        for (num, count) in counts.drain() {
            if min_heap.len() < k {
                min_heap.push(Reverse(CountedVal { val: num, count }));
            } else if let Some(smallest) = min_heap.peek()
                && smallest.0.count < count
            {
                min_heap.pop();
                min_heap.push(Reverse(CountedVal { val: num, count }));
            }
        }
        min_heap
            .drain()
            .map(|counted_num| counted_num.0.val)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0347() {
        helper(&[1, 1, 1, 2, 2, 3], 2, &mut [1, 2]);
        helper(&[1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2, &mut [1, 2]);
        helper(&[1], 1, &mut [1]);
    }

    fn helper(nums: &[i32], k: usize, expected: &mut [i32]) {
        let mut sol = Solution::top_k_frequent(nums, k);
        sol.sort();
        expected.sort();
        assert_eq!(sol, expected);
    }
}
