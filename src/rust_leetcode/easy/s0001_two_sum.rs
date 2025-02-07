//! https://leetcode.com/problems/two-sum/
//! Easy - [array, hash table]
//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to
//! target. You may assume that each input would have exactly one solution, and you may not use the same element twice.
//! You can return the answer in any order.

use std::collections::hash_map::HashMap;

pub struct Solution {}

impl Solution {
    /// Store each number in a hashmap with its index.
    /// We have found a solution when the map already contains the difference between a number and the target.
    fn two_sum_option(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
        let mut hash_map = HashMap::<i32, usize>::new();
        for (i, num) in nums.iter().enumerate() {
            let difference = target - *num;
            match hash_map.get(&difference) {
                Some(j) => return Some(vec![i as i32, *j as i32]),
                None => {
                    hash_map.insert(*num, i);
                }
            }
        }
        None
    }

    /// Convert to wanted return type.
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum_option(nums, target).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    fn test_sorted_solution(nums: Vec<i32>, target: i32, solution: Vec<i32>) {
        let mut sol = Solution::two_sum(nums, target);
        sol.sort();
        assert_eq!(sol, solution);
    }

    #[test]
    fn test_0001() {
        test_sorted_solution(vec![2, 7, 11, 15], 9, vec![0, 1]);
        test_sorted_solution(vec![3, 2, 4], 6, vec![1, 2]);
        test_sorted_solution(vec![3, 3], 6, vec![0, 1]);
    }
}
