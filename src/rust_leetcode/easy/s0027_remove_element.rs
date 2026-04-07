//! https://leetcode.com/problems/remove-element/
//! Easy - [array, two pointers]
//! Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
//! The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//! Consider the number of elements in nums which are not equal to val be k, to get accepted,
//! you need to do the following things:
//! 1. Change the array nums such that the first k elements of nums contain the elements which are not equal to val.
//!    The remaining elements of nums are not important as well as the size of nums.
//! 2. Return k.

pub struct Solution {}

impl Solution {
    /// Iterate over the vector and remove the wanted elements.
    /// Removing any i-th element while require reallocating
    /// the remaining (n - i) elements, which is wasteful.
    pub fn remove_element_naive(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut length = nums.len();
        let mut i = 0;
        while i < length {
            if nums[i] == val {
                // This will require many memory reallocations.
                nums.remove(i);
                length -= 1;
            } else {
                i += 1;
            }
        }
        length as i32
    }

    /// Swap the wanted elements to the back of the array.
    pub fn remove_element<T: PartialEq>(nums: &mut [T], val: T) -> usize {
        if nums.is_empty() {
            return 0;
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut swapped = 0;

        while left <= right {
            if nums[right] == val {
                swapped += 1;
                if right == 0 {
                    break;
                }
                right -= 1;
            } else if nums[left] == val {
                nums.swap(left, right);
                swapped += 1;
                left += 1;
                right -= 1;
            } else {
                left += 1;
            }
        }
        nums.len() - swapped
    }
}

#[cfg(test)]
pub mod tests {
    use super::Solution;

    fn test_helper(nums: &mut [i32], val: i32, nums_different: &mut [i32]) {
        let sol = Solution::remove_element(nums, val);
        assert_eq!(sol, nums_different.len());
        nums_different.sort();
        let solution = &mut nums[..nums_different.len()];
        solution.sort();
        assert_eq!(solution, nums_different);
    }

    #[test]
    fn test_0027() {
        test_helper(&mut [], 3, &mut []);
        test_helper(&mut [1, 1], 1, &mut []);
        test_helper(&mut [3, 2, 2, 3], 3, &mut [2, 2]);
        test_helper(&mut [0, 1, 2, 2, 3, 0, 4, 2], 2, &mut [0, 0, 1, 3, 4]);
    }
}
