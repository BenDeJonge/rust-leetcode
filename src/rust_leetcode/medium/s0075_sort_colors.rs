//! https://leetcode.com/problems/sort-colors/
//! Medium - [array, two-pointers, sorting]
//!
//! Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
//! We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.
//! You must solve this problem without using the library's sort function.
//!
//! Example 1:
//! Input: nums = [2,0,2,1,1,0]
//! Output: [0,0,1,1,2,2]
//! Example 2:
//! Input: nums = [2,0,1]
//! Output: [0,1,2]
//!
//! Constraints:
//! - n == nums.length
//! - 1 <= n <= 300
//! - nums[i] is either 0, 1, or 2.
//!
//! Follow up:Could you come up with a one-pass algorithm using only constant extra space?

pub struct Solution {}

// | Case                | cmp | pointers | swaps |
// |---------------------|-----|----------|-------|
// |  ij              k  |     |          |       |
// | [0 2 1 2 0 1 1 2 0] | j<1 | i+1  j+1 | i<->j |
// |                     |     |          |       |
// |    ij            k  |     |          |       |
// | [0 2 1 2 0 1 1 2 0] | j>1 | k-1      | j<->k |
// |                     |     |          |       |
// |    ij          k    |     |          |       |
// | [0 0 1 2 0 1 1 2 2] | j<1 | i+1  j+1 | i<->j |
// |                     |     |          |       |
// |      ij      k      |     |          |       |
// | [0 0 1 2 0 1 1 2 2] | j=1 | j+1      |       |
// |                     |     |          |       |
// |      i j     k      |     |          |       |
// | [0 0 1 2 0 1 1 2 2] | j>1 | k-1      | j<->k |
// |                     |     |          |       |
// |      i j   k        |     |          |       |
// | [0 0 1 1 0 1 2 2 2] | j=1 | j+1      |       |
// |                     |     |          |       |
// |      i   j k        |     |          |       |
// | [0 0 1 1 0 1 2 2 2] | j<1 | i+1  j+1 | i<->j |
// |                     |     |          |       |
// |        i   jk       |     |          |       |
// | [0 0 0 1 1 1 2 2 2] | j<1 | i+1  j+1 | i<->j |

impl Solution {
    /// This is an implementation of Dijkstra's Dutch National Flag algorithm.
    /// The key idea is to maintain 3 pointers, one for each target bucket,
    /// which are advanced towards the middle depending on the value.
    pub fn sort_colors(nums: &mut [i32]) {
        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len() - 1;

        let middle = 1;

        // Monitor the pointer for the middle bucket:
        while j <= k {
            // At an element from the lower bucket.
            // i now points at a wanted value, so advance it.
            if nums[j] < middle {
                nums.swap(i, j);
                i += 1;
                j += 1;
            }
            // At an element in the higher bucket.
            // k now points at a wanted value, so advance it.
            else if nums[j] > middle {
                nums.swap(j, k);
                if k == 0 {
                    break;
                }
                k -= 1;
            }
            // At an element of the middle bucket.
            else {
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0075() {
        let nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        helper(nums, expected);

        let nums = vec![2];
        let expected = vec![2];
        helper(nums, expected);

        let nums = vec![2, 2];
        let expected = vec![2, 2];
        helper(nums, expected);
    }

    fn helper(mut nums: Vec<i32>, expected: Vec<i32>) {
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
}
