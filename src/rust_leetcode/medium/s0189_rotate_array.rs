//! <https://leetcode.com/problems/rotate-array/>
//! Medium - [array, math, two-pointers]
//!
//! Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
//!
//! Example 1:
//! Input: nums = [1,2,3,4,5,6,7], k = 3
//! Output: [5,6,7,1,2,3,4]
//! Explanation:
//! rotate 1 steps to the right: [7,1,2,3,4,5,6]
//! rotate 2 steps to the right: [6,7,1,2,3,4,5]
//! rotate 3 steps to the right: [5,6,7,1,2,3,4]
//! Example 2:
//! Input: nums = [-1,-100,3,99], k = 2
//! Output: [3,99,-1,-100]
//! Explanation:
//! rotate 1 steps to the right: [99,-1,-100,3]
//! rotate 2 steps to the right: [3,99,-1,-100]
//!
//! Constraints:
//! - 1 <= nums.length <= 10**5
//! - -2*31 <= nums[i] <= 2*31 - 1
//! - 0 <= k <= 10**5
//!
//! Follow up:
//! - Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
//! - Could you do it in-place with O(1) extra space?

use std::fmt::Debug;
use std::num::NonZero;

pub struct Solution {}

impl Solution {
    /// O(kn)
    pub fn rotate_naive<T>(nums: &mut [T], k: usize) {
        for _ in 0..k % nums.len() {
            for i in 0..(nums.len() - 1) {
                nums.swap(i, nums.len() - 1);
            }
        }
    }

    /// O(n)
    pub fn rotate_reverse<T>(nums: &mut [T], k: usize) {
        let i = k % nums.len();
        nums.reverse();
        nums[..i].reverse();
        nums[i..].reverse();
    }

    /// O(n)
    pub fn rotate_ideomatic<T>(nums: &mut [T], k: usize) {
        nums.rotate_right(k % nums.len());
    }

    pub fn rotate(nums: &mut [i32], k: i32) {
        Self::rotate_typed(nums, k.try_into().unwrap());
    }

    /// O(n)
    fn rotate_typed<T: Debug + Clone>(nums: &mut [T], k: usize) {
        if nums.is_empty() {
            return;
        }
        let len = nums.len();
        let step = k % len;
        if step == 0 {
            return;
        }

        let cycles = Self::gcd(NonZero::<usize>::try_from(len).unwrap(), step);
        for i in 0..cycles {
            let mut element = nums[i].clone();
            // Good use case for itertools peeking_take_while
            // https://docs.rs/itertools/0.10.0/itertools/trait.Itertools.html#method.peeking_take_while
            for n in (i..).step_by(step).map(|c| c % len).skip(1) {
                std::mem::swap(&mut nums[n], &mut element);
                if n == i {
                    break;
                }
            }
        }
    }

    /// Euclidian method to find greates common denominator.
    fn gcd(a: NonZero<usize>, b: usize) -> usize {
        if b == 0 {
            a.get()
        } else if b < a.get() {
            Self::gcd(NonZero::try_from(b).unwrap(), a.get())
        } else {
            Self::gcd(a, b % a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0189_even() {
        helper(&mut [0, 1, 2, 3, 4, 5], 0, &[0, 1, 2, 3, 4, 5]);
        helper(&mut [0, 1, 2, 3, 4, 5], 1, &[5, 0, 1, 2, 3, 4]);
        helper(&mut [0, 1, 2, 3, 4, 5], 2, &[4, 5, 0, 1, 2, 3]);
        helper(&mut [0, 1, 2, 3, 4, 5], 3, &[3, 4, 5, 0, 1, 2]);
        helper(&mut [0, 1, 2, 3, 4, 5], 4, &[2, 3, 4, 5, 0, 1]);
        helper(&mut [0, 1, 2, 3, 4, 5], 5, &[1, 2, 3, 4, 5, 0]);
        helper(&mut [0, 1, 2, 3, 4, 5], 6, &[0, 1, 2, 3, 4, 5]);
        helper(&mut [0, 1, 2, 3, 4, 5], 7, &[5, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_0189_odd() {
        helper(&mut [0, 1, 2, 3, 4], 0, &[0, 1, 2, 3, 4]);
        helper(&mut [0, 1, 2, 3, 4], 1, &[4, 0, 1, 2, 3]);
        helper(&mut [0, 1, 2, 3, 4], 2, &[3, 4, 0, 1, 2]);
        helper(&mut [0, 1, 2, 3, 4], 3, &[2, 3, 4, 0, 1]);
        helper(&mut [0, 1, 2, 3, 4], 4, &[1, 2, 3, 4, 0]);
        helper(&mut [0, 1, 2, 3, 4], 5, &[0, 1, 2, 3, 4]);
        helper(&mut [0, 1, 2, 3, 4], 6, &[4, 0, 1, 2, 3]);
    }

    fn helper(nums: &mut [usize], k: usize, expected: &[usize]) {
        Solution::rotate_typed(nums, k);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(Solution::gcd(1.try_into().unwrap(), 6), 1);
        assert_eq!(Solution::gcd(2.try_into().unwrap(), 6), 2);
        assert_eq!(Solution::gcd(3.try_into().unwrap(), 6), 3);
        assert_eq!(Solution::gcd(4.try_into().unwrap(), 6), 2);
        assert_eq!(Solution::gcd(5.try_into().unwrap(), 6), 1);
        assert_eq!(Solution::gcd(6.try_into().unwrap(), 6), 6);
        assert_eq!(Solution::gcd(7.try_into().unwrap(), 6), 1);
    }
}
