/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
/// Easy - [array, two pointers]
/// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique
/// element appears only once. The relative order of the elements should be kept the same. Then return the number of
/// unique elements in nums.
/// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
/// 1. Change the array nums such that the first k elements of nums contain the unique elements in the order they were
/// present in nums initially. The remaining elements of nums are not important as well as the size of nums.
/// 2. Return k.
pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // Edge case to avoid nums[i+1] out of bounds errors.
        if nums.len() == 1 {
            return 1;
        }
        let mut i = 1;
        let mut length = nums.len();
        while i < length {
            // A duplicate.
            if nums[i - 1] == nums[i] {
                let v = nums[i - 1];
                // Add it to the end.
                nums.remove(i - 1);
                nums.push(v);
                // We do not have to check that number again.
                length -= 1;
            }
            // Check number i + 1 against i + 2. If we do not put this in the else, we will skip numbers, as moving a
            // number to the end, brings the next one in to compare against the previous i.
            else {
                i += 1
            }
        }
        // As we only advance i every time a number is unique, we can simply return this.
        i as i32
    }
}

fn main() {}

#[cfg(test)]

mod tests {
    use super::Solution;

    fn test_helper(vec: &mut Vec<i32>, n_unique: i32, vec_unique: Vec<i32>) {
        assert_eq!(Solution::remove_duplicates(vec), n_unique);
        assert_eq!(vec[..n_unique as usize], vec_unique);
    }

    #[test]
    fn test_0026() {
        test_helper(&mut vec![1, 1, 2], 2, vec![1, 2]);
        test_helper(
            &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
            5,
            vec![0, 1, 2, 3, 4],
        );
        test_helper(&mut vec![1], 1, vec![1]);
        test_helper(&mut vec![1, 1], 1, vec![1]);
    }
}
