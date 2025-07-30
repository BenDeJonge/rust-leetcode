//! https://leetcode.com/problems/next-permutation/
//! Medium - [array, two-pointers]
//!
//! A permutation of an array of integers is an arrangement of its members into
//! a sequence or linear order.
//!
//! E.g., for arr = [1,2,3], the following are all the permutations of arr:
//! [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
//!
//! The next permutation of an array of integers is the next lexicographically
//! greater permutation of its integer. More formally, if all the permutations
//! of the array are sorted in one container according to their lexicographical
//! order, then the next permutation of that array is the permutation that
//! follows it in the sorted container. If such arrangement is not possible,
//! the array must be rearranged as the lowest possible order (i.e., sorted in
//! ascending order).
//!
//! For example, the next permutation of arr = [1,2,3] is [1,3,2].
//! Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
//! While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does
//! not have a lexicographical larger rearrangement.
//!
//! Given an array of integers nums, find the next permutation of nums.
//! The replacement must be in place and use only constant extra memory.
//!
//! Example 1:
//! Input: nums = [1,2,3]
//! Output: [1,3,2]
//! Example 2:
//! Input: nums = [3,2,1]
//! Output: [1,2,3]
//! Example 3:
//! Input: nums = [1,1,5]
//! Output: [1,5,1]
//!
//! Constraints:
//! - `1 <= nums.length <= 100`
//! - `0 <= nums[i] <= 100`

pub struct Solution {}

impl Solution {
    /// Get the next permutation in place with a two pointer solutions.
    /// Elements in the vector need to be swapped if the left is smaller than
    /// the right element. Three cases need to be handled:
    /// 1. Swapping the last two numbers:
    ///    ```text
    ///            v v            v..
    ///    4 3 2 5 1 3 => 4 3 2 5 3 1
    ///    ```
    ///
    /// 2. Swapping two numbers and sorting all subsequent elements:
    ///    ```text
    ///        v v            v......
    ///    4 3 2 5 1 3 => 4 3 5 2 1 3 => 4 3 2 5 1 2 3
    ///    ```
    ///    Turns out that this case is equivalent to the first.
    ///    Note that the pointers travel "recursively" i.e.,
    ///    ```text
    ///          v   v          v v          v     v
    ///    4 3 2 5 1 3 => 4 3 2 5 1 3 => 4 3 2 5 1 3
    ///    ```
    ///    
    /// 3. Sorting the entire vector if it is in reverse lexicographic order.
    ///    ```text
    ///    v v            ...........
    ///    5 4 3 3 2 1 => 5 4 3 3 2 1 => 1 2 3 3 4 5
    ///    ```
    ///
    /// - Time complexity: `O(n)`
    /// - Space complexity: `O(1)`
    pub fn next_permutation(nums: &mut [i32]) {
        if nums.len() < 2 {
            return;
        }

        let mut right = nums.len() - 1;
        let mut left = Some(right - 1);

        'outer: while let Some(l) = left {
            while l < right {
                if nums[l] < nums[right] {
                    break 'outer;
                }
                right -= 1;
            }
            left = l.checked_sub(1);
            right = nums.len() - 1;
        }

        if let Some(l) = left {
            nums.swap(l, right);
            nums[(l + 1)..].sort();
        } else {
            nums.sort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0031() {
        let mut nums = [4, 2, 0, 2, 3, 2, 0];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [4, 2, 0, 3, 0, 2, 2]);

        let mut nums = [1, 2, 3];
        for expected in [
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
            [1, 2, 3],
        ] {
            Solution::next_permutation(&mut nums);
            assert_eq!(nums, expected);
        }

        let mut nums = [1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, [1, 5, 1]);
    }
}
