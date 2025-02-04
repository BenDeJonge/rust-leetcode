//! https://leetcode.com/problems/single-number
//! Easy - [array, bit manipulation]
//! Given a non-empty array of integers nums, every element appears twice except for one.
//! Find that single one.
//! You must implement a solution with a linear runtime complexity and use only constant extra space.

pub struct Solution {}

impl Solution {
    /// The bitwise XOR of two numbers is zero.
    /// Since most numbers appears twice, they will (eventually) undo their own bitshift.
    /// However, one number is unique, meaning its XOR trace will propagate through the chain.
    /// E.g. [4, 1, 2, 1, 2]
    /// [0b100, 0b001, 0b010, 0b001, 0b010]
    /// 0b100 ^ 0b001 = 0b101
    /// 0b101 ^ 0b010 = 0b111
    /// 0b111 ^ 0b001 = 0b110
    /// 0b110 ^ 0b010 = 0b100 = 4
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // We use .into_iter() pass the type T into the .reduce(), rather than the reference &T from .iter()
        nums.into_iter()
            .reduce(|first, second| first ^ second)
            .expect("nums is empty")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0121() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
