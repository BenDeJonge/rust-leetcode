//! https://leetcode.com/problems/3sum/
//! Medium - [array, two pointers, sorting]
//! Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
//! such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//! Notice that the solution set must not contain duplicate triplets.

use std::cmp::Ordering;
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut result = <Vec<Vec<i32>>>::new();
        // Sort the input array to allow for a two pointer approach.
        let mut nums_sorted = nums.clone();
        nums_sorted.sort();
        // Loop over length - 2 as beyond that, we cannot form a triplet anymore.
        for (i, &num) in nums_sorted.iter().enumerate() {
            // Skip duplicate numbers (beyond the first loop, as there is not yet a previous number there).
            if i > 0 && num == nums_sorted[i - 1] {
                continue;
            }
            // The two pointers, which should never touch or cross.
            let mut i_left = i + 1;
            let mut i_right = nums.len() - 1;
            while i_left < i_right {
                let left = nums_sorted[i_left];
                let right = nums_sorted[i_right];
                match (num + left + right).cmp(&0) {
                    // The other two numbers match the target.
                    Ordering::Equal => {
                        result.push(vec![num, left, right]);
                        i_left += 1;
                        // Avoid duplicates by incrementing the left pointer as long as the numbers are identical.
                        // Both pointers should never cross.
                        // If needed, the right pointer is updated automatically in the next loop iteration, as the sum
                        // will then be too big anyway.
                        while nums_sorted[i_left] == nums_sorted[i_left - 1] && i_left < i_right {
                            i_left += 1;
                        }
                    }
                    // The sum is too low i.e., we are using too small numbers. Increase the smallest number.
                    Ordering::Less => i_left += 1,
                    // The sum is too big i.e., we are using too large numbers. Decrease the largest number.
                    Ordering::Greater => i_right -= 1,
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    #[test]
    fn test_0015() {
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), <Vec<Vec<i32>>>::new());
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
