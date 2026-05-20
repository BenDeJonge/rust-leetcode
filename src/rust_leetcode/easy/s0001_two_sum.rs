//! <https://leetcode.com/problems/two-sum/>
//! Easy - [array, hash table]
//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to
//! target. You may assume that each input would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.

use std::collections::hash_map::HashMap;

pub struct Solution {}

impl Solution {
    /// Store each number in a hashmap with its index.
    /// We have found a solution when the map already contains the difference between a number and the target.
    fn two_sum(nums: &[i32], target: i32) -> Option<[i32; 2]> {
        let mut hash_map = HashMap::<i32, usize>::new();
        for (i, num) in nums.iter().enumerate() {
            let difference = target - *num;
            match hash_map.get(&difference) {
                Some(j) => return Some([i32::try_from(i).unwrap(), i32::try_from(*j).unwrap()]),
                None => {
                    hash_map.insert(*num, i);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn test_sorted_solution(nums: &[i32], target: i32, solution: &[i32]) {
        let mut sol = Solution::two_sum(nums, target).unwrap();
        sol.sort_unstable();
        assert_eq!(sol, solution);
    }

    #[test]
    fn test_0001() {
        test_sorted_solution(&[2, 7, 11, 15], 9, &[0, 1]);
        test_sorted_solution(&[3, 2, 4], 6, &[1, 2]);
        test_sorted_solution(&[3, 3], 6, &[0, 1]);
    }
}
