//! https://leetcode.com/problems/trapping-rain-watter/
//! Hard - [array, two pointer, dynamic programming, stack, monotonic stack]
//! Given n non-negative integers representing an elevation map where the width
//! of each bar is 1, compute how much water it can trap after raining.

pub struct Solution {}
impl Solution {
    /// This solution is naive because we are iterating over the whole array for
    /// every element. This is not needed because the maximum does not change
    /// until we encounter a value that is actually larger.
    pub fn trap_naive(height: Vec<i32>) -> i32 {
        height.iter().enumerate().fold(0, |acc, (i, h)| {
            let water = Self::largest_left(i, &height)
                .min(Self::largest_right(i, &height))
                .saturating_sub(*h);
            if water.is_positive() {
                acc + water
            } else {
                acc
            }
        })
    }

    fn largest_left(i: usize, height: &[i32]) -> i32 {
        *height.iter().take(i).max().unwrap_or(&0)
    }

    fn largest_right(i: usize, height: &[i32]) -> i32 {
        *height.iter().skip(i).max().unwrap_or(&0)
    }

    /// Use 2 pointers to track the positions to calculate the water level for.
    /// This allows to cheaply updating the boundary on both sides of each
    /// pointer. These, in turn, define the wall heights of the "box" that might
    /// be able to hold water. The lowest of both pointers will limit the water
    /// level more, so that one is advanced.
    pub fn trap(height: Vec<i32>) -> i32 {
        let sum = &mut 0i32;
        let left = &mut 0usize;
        let right = &mut (height.len() - 1);
        let max_left = &mut 0i32;
        let max_right = &mut 0i32;

        while left < right {
            // The water level is limited by the ...
            // ... left boundary.
            if height[*left] <= height[*right] {
                Self::trap_helper(&height, left, max_left, sum, |ptr| *ptr += 1);
            }
            // ... right boundary.
            else {
                Self::trap_helper(&height, right, max_right, sum, |ptr| *ptr -= 1)
            }
        }
        *sum
    }

    fn trap_helper(
        height: &[i32],
        ptr: &mut usize,
        max: &mut i32,
        sum: &mut i32,
        advance_ptr: fn(&mut usize),
    ) {
        let h_curr = height[*ptr];
        // The height is lower than the boundary, so water does not flow away.
        if h_curr < *max {
            *sum += *max - h_curr;
        }
        // Water flows away so update the boundary.
        else {
            *max = h_curr;
        }
        advance_ptr(ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_0023() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
