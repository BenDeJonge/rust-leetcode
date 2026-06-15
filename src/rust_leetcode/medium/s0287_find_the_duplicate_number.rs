//! <https://leetcode.com/problems/find-the-duplicate-number/>
//! Medium - [array, two-pointers, binary-search, bit-manipulation]
//!
//! Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
//! There is only one repeated number in nums, return this repeated number.
//! You must solve the problem without modifying the array nums and using only constant extra space.
//!
//! Example 1:
//! Input: nums = [1,3,4,2,2]
//! Output: 2
//! Example 2:
//! Input: nums = [3,1,3,4,2]
//! Output: 3
//! Example 3:
//! Input: nums = [3,3,3,3,3]
//! Output: 3
//!
//! Constraints:
//! - 1 <= n <= 10**5
//! - nums.length == n + 1
//! - 1 <= nums[i] <= n
//! - All the integers in nums appear only once except for precisely one integer which appears two or more times.
//!
//! Follow up:
//! - How can we prove that at least one duplicate number must exist in nums?
//! - Can you solve the problem in linear runtime complexity?

pub struct Solution {}

impl Solution {
    /// Time complexity: O(n)
    ///
    /// Tortoise and hare algorithm for cycle detection.
    ///
    /// Consider a list of numbers:
    /// ```text
    /// idx | 0 1 2 3 4
    /// ----|----------
    /// val | 2 3 4 1 1
    /// ```
    ///
    /// A graph can be drawn based on their indices as follows:
    ///
    /// ```text
    /// 0 ──> 2 ──> 4 ──> 1 ──> 3
    ///                   ^     │
    ///                   └─────┘
    /// ```
    ///
    /// This can only be the case if one the numbers is duplicated.
    ///
    /// Cycles in a graph can be detected using a tortoise-and-hare two-pointer algorithm,
    /// where the fast (hare, `h`) pointer takes 2 steps for every step of the slow (tortoise, `t`) pointer.
    /// Therefore, the distance (`d`) between them is always equal to:
    ///
    /// `h = 2t <=> d = h - t = 2t - t = t`
    ///
    /// If there is a cycle, the tortoise and hare will enter it at some point.
    /// Even later, they will both arrive at the same index.
    /// Since they are caught in a loop, `d` must then be a multiple of the period of the cycle (`p`).
    ///
    /// When the cycle is found, the tortoise is reset to the starting position, meaning `d = h = 2t`.
    /// The tortoise then starts to approach the cycle at its original speed of a single step per iteration.
    /// The hare is also slowed down to taking single steps.
    ///
    /// The distance to start of the loop (`x`) is now the same for both the tortoise and the hare.
    /// The tortoise would need to travel (`2t - x) % p` to reach the hare.
    /// Therefore, the remaining distance from the hare to the loop must be `x % p`.
    /// So, when the both meet again, we have found the repeated element.
    ///
    /// ```text
    ///     x
    /// t ─────> i ─────┐
    ///          ^      │ (2t - x) % p
    ///    x % p │      │
    ///          └── h ─┘          
    /// ```
    pub fn find_duplicate_inner(nums: &[usize]) -> usize {
        let mut slow = nums[0];
        let mut fast = nums[slow];

        // Traverse the graph of indices until a cycle is found.
        while slow != fast {
            slow = nums[slow];
            fast = nums[nums[fast]];
        }

        // The tortoise and hare meet again at the repeated element.
        slow = 0;
        while slow != fast {
            slow = nums[slow];
            fast = nums[fast];
        }

        slow
    }

    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let vector: Vec<usize> = nums
            .iter()
            .map(|&num| usize::try_from(num).expect("cannot convert"))
            .collect();
        Solution::find_duplicate_inner(&vector) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0287_odd_number() {
        assert_eq!(Solution::find_duplicate_inner(&[1, 1, 2, 3, 4]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[1, 2, 1, 3, 4]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[1, 2, 3, 1, 4]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[1, 2, 3, 4, 1]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 1, 1, 3, 4]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 1, 3, 1, 4]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 1, 3, 4, 1]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 3, 1, 1, 4]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 3, 1, 4, 1]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 3, 4, 1, 1]), 1);
    }

    #[test]
    fn test_0287_even_number() {
        assert_eq!(Solution::find_duplicate_inner(&[1, 1, 2, 3]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[1, 2, 1, 3]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[1, 2, 1, 3]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 1, 1, 3]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 1, 3, 1]), 1);
        assert_eq!(Solution::find_duplicate_inner(&[2, 3, 1, 1]), 1);
    }
}
