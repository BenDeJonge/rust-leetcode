/// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
/// Medium - [array, two pointers, binary search]
/// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order,
/// find two numbers such that they add up to a specific target number.
/// Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
/// Return the indices of the two numbers, index1 and index2,
/// added by one as an integer array [index1, index2] of length 2.
/// The tests are generated such that there is exactly one solution.
/// You may not use the same element twice.
/// Your solution must use only constant extra space.
use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i_left = 0;
        let mut i_right = numbers.len() - 1;
        while i_left < i_right {
            let left = numbers[i_left];
            let right = numbers[i_right];
            match target.cmp(&(left + right)) {
                // Found a solution.
                Ordering::Equal => return vec![i_left as i32 + 1, i_right as i32 + 1],
                // The target is too big, increase the lowest value of the sum to get closer.
                Ordering::Greater => i_left += 1,
                // The target is too small, decrease the highest value of the sum to get closer.
                Ordering::Less => i_right -= 1,
            }
        }
        // No solution.
        vec![]
    }
}

fn main() {}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0167() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
