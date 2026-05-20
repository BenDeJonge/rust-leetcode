//! <https://leetcode.com/problems/next-greater-element-i/>
//! Easy - [array, hash-table, stack, monotonic-stack]
//!
//! The next greater element of some element x in an array is the first greater
//! element that is to the right of x in the same array.
//! You are given two distinct 0-indexed integer arrays nums1 and nums2,
//! where nums1 is a subset of nums2.
//! For each 0 <= i < nums1.length, find the index j such that
//! nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2.
//! If there is no next greater element, then the answer for this query is -1.
//! Return an array ans of length nums1.length such that ans[i] is
//! the next greater element as described above.
//!
//! Example 1:
//! Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
//! Output: [-1,3,-1]
//! Explanation: The next greater element for each value of nums1 is as follows:
//! - 4 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
//! - 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
//! - 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
//!
//! Example 2:
//! Input: nums1 = [2,4], nums2 = [1,2,3,4]
//! Output: [3,-1]
//! Explanation: The next greater element for each value of nums1 is as follows:
//! - 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
//! - 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.
//!
//! Constraints:
//! - 1 <= nums1.length <= nums2.length <= 1000
//! - 0 <= nums1[i], nums2[i] <= 10**4
//! - All integers in nums1 and nums2 are unique.
//! - All the integers of nums1 also appear in nums2.
//!
//! Follow up: Could you find an O(nums1.length + nums2.length) solution?

use std::{collections::HashMap, hash::Hash};

pub struct Solution {}

impl Solution {
    /// Solve in O(n(nums1) + n(nums2))
    ///
    /// Make a monotonically decreasing stack.
    /// For each number, keep popping the stack until a larger number is found.
    /// For every popped number, we know the current is a larger one.
    /// We can track this in a Hashmap.
    pub fn next_greater_element<T: Ord + Hash + Copy>(nums1: &[T], nums2: &[T]) -> Vec<Option<T>> {
        let mut stack = Vec::with_capacity(nums2.len());
        let mut map: HashMap<T, T> = HashMap::with_capacity(nums2.len());
        for number in nums2 {
            while let Some(smaller) = stack.pop_if(|top| *top < *number) {
                map.insert(smaller, *number);
            }
            stack.push(*number);
        }

        nums1
            .iter()
            .map(|number| map.get(number).copied())
            .collect()
    }

    /// Solve in O(n(nums1) + n(nums2))
    pub fn next_greater_element_naive<T: Ord + Copy>(nums1: &[T], nums2: &[T]) -> Vec<Option<T>> {
        nums1
            .iter()
            .map(|left| {
                if let Some(i) = nums2.iter().position(|el| el == left) {
                    nums2.iter().skip(i).find(|&right| right > left).copied()
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0496() {
        assert_eq!(
            Solution::next_greater_element(&[4, 1, 2], &[1, 3, 4, 2]),
            &[None, Some(3), None]
        );
        assert_eq!(
            Solution::next_greater_element(&[2, 4], &[1, 2, 3, 4]),
            &[Some(3), None]
        );
    }
}
