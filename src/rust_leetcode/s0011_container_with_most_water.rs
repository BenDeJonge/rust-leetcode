//! https://leetcode.com/problems/container-with-most-water/
//! Medium - [array, two pointer, greedy]
//! You are given an integer array height of length n.
//! There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
//! Find two lines that together with the x-axis form a container, such that the container contains the most water.
//! Return the maximum amount of water a container can store.
//! Notice that you may not slant the container.

pub struct Solution {}

impl Solution {
    pub fn max_area_naive(height: Vec<i32>) -> i32 {
        let mut best_area = 0;
        for left in 0..height.len() - 1 {
            for right in left..height.len() {
                // The left side is so low that no matter the horizontal length, we will never beat the best area.
                if height[left] * ((height.len() - left) as i32) < best_area {
                    break;
                }
                // Compute and compare the new area.
                let horizontal = (right - left) as i32;
                let vertical = height[left].min(height[right]);
                best_area = best_area.max(horizontal * vertical)
            }
        }
        best_area
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        // Start the boundaries as the widest container.
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut best_area = 0;
        while left < right {
            // Compute the current area and overwrite the max if it is bigger.
            best_area = best_area.max(((right - left) as i32) * height[left].min(height[right]));
            // Greedy approach: try to improve the worst height.
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        best_area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0011() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1)
    }
}
