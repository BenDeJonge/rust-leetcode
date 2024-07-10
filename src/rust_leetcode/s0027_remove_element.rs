/// https://leetcode.com/problems/remove-element/
/// Easy - [array, two pointers]
/// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
/// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
/// Consider the number of elements in nums which are not equal to val be k, to get accepted,
/// you need to do the following things:
/// 1. Change the array nums such that the first k elements of nums contain the elements which are not equal to val.
/// The remaining elements of nums are not important as well as the size of nums.
/// 2. Return k.
pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // let original_length = nums.len();
        let mut length = nums.len();
        let mut i = 0;
        while i < length {
            if nums[i] == val {
                nums.remove(i);
                length -= 1;
            } else {
                i += 1;
            }
        }
        length as i32
    }
}

fn main() {}

#[cfg(test)]

pub mod tests {
    use std::vec;

    use super::Solution;

    fn test_helper(nums: &mut Vec<i32>, val: i32, nums_different: &mut Vec<i32>) {
        let sol = Solution::remove_element(nums, val);
        assert_eq!(sol as usize, nums_different.len());
        assert_eq!(nums, nums_different);
    }

    #[test]
    fn test_0027() {
        test_helper(&mut vec![], 3, &mut vec![]);
        test_helper(&mut vec![1, 1], 1, &mut vec![]);
        test_helper(&mut vec![3, 2, 2, 3], 3, &mut vec![2, 2]);
        test_helper(
            &mut vec![0, 1, 2, 2, 3, 0, 4, 2],
            2,
            &mut vec![0, 1, 3, 0, 4],
        );
    }
}
