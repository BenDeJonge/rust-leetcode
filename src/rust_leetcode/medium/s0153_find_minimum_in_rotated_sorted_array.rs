//! <https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/>
//! Medium - [array, binary-search]
//!
//! Suppose an array of length n sorted in ascending order is rotated between 1 and n times.
//! For example, the array nums = [0,1,2,4,5,6,7] might become:
//! - [4,5,6,7,0,1,2] if it was rotated 4 times.
//! - [0,1,2,4,5,6,7] if it was rotated 7 times.
//!
//! Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array
//! [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
//!
//! Given the sorted rotated array nums of unique elements, return the minimum element of this array.
//! You must write an algorithm that runs inO(log n) time.
//!
//! Example 1:
//! Input: nums = [3,4,5,1,2]
//! Output: 1
//! Explanation: The original array was [1,2,3,4,5] rotated 3 times.
//! Example 2:
//! Input: nums = [4,5,6,7,0,1,2]
//! Output: 0
//! Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.
//! Example 3:
//! Input: nums = [11,13,15,17]
//! Output: 11
//! Explanation: The original array was [11,13,15,17] and it was rotated 4 times.
//!
//! Constraints:
//! - n == nums.length
//! - 1 <= n <= 5000
//! - -5000 <= nums[i] <= 5000
//! - All the integers of nums are unique.
//! - nums is sorted and rotated between 1 and n times.

use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        *Self::find_min_inner(&nums).unwrap_or(&i32::default())
    }

    /// O(log(n))
    ///
    /// Define a slice with two pointers. If the minimum is contained inside,
    /// the left must be larger than the right.
    fn find_min_inner<T: Ord>(values: &[T]) -> Option<&T> {
        // The array is not rotated or of length 0 or 1.
        if values.first() <= values.last() {
            return values.first();
        }

        let mut left = 0;
        let mut right = values.len() - 1;

        while left <= right {
            let middle = (left + right) / 2;
            match values[left].cmp(&values[middle]) {
                // [4, 0, 1, 2, 3]
                //  L     M     R
                Ordering::Greater => {
                    right = middle;
                }
                // [2, 3, 4, 0, 1]
                //  L     M     R
                Ordering::Less => {
                    left = middle;
                }
                // [4, 0, 1, 2, 3]
                //  LM R
                Ordering::Equal => {
                    return values.get(right);
                }
            }
        }

        unreachable!("sorted array of unique elements has 1 defined min")
    }

    /// O(log(n))
    ///
    /// Find the rightmost value larger than the last value.
    /// That index is the right edge of the slice of numbers that were moved
    /// from the end of the array to the front.
    fn find_min_ideomatic<T: Ord>(values: &[T]) -> Option<&T> {
        values.get(values.partition_point(|val| val > values.last().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::Solution;

    #[test]
    fn test_0153_edge_cases() {
        helper(&Vec::<usize>::new(), None);
        helper(&[0], Some(&0));
        helper(&[0, 1], Some(&0));
        helper(&[1, 0], Some(&0));
        helper(&[0, 1, 2], Some(&0));
        helper(&[2, 0, 1], Some(&0));
        helper(&[1, 2, 0], Some(&0));
    }

    #[test]
    fn test_0153_odd() {
        helper(&[0, 1, 2, 3, 4], Some(&0));
        helper(&[4, 0, 1, 2, 3], Some(&0));
        helper(&[3, 4, 0, 1, 2], Some(&0));
        helper(&[2, 3, 4, 0, 1], Some(&0));
        helper(&[1, 2, 3, 4, 0], Some(&0));
    }

    #[test]
    fn test_0153_even() {
        helper(&[0, 1, 2, 3, 4, 5], Some(&0));
        helper(&[5, 0, 1, 2, 3, 4], Some(&0));
        helper(&[4, 5, 0, 1, 2, 3], Some(&0));
        helper(&[3, 4, 5, 0, 1, 2], Some(&0));
        helper(&[2, 3, 4, 5, 0, 1], Some(&0));
        helper(&[1, 2, 3, 4, 5, 0], Some(&0));
    }

    fn helper<T: Ord + Debug>(values: &[T], expected: Option<&T>) {
        assert_eq!(Solution::find_min_inner(values), expected);
        assert_eq!(Solution::find_min_ideomatic(values), expected);
    }
}
