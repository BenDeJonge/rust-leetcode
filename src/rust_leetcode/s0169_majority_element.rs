/// https://leetcode.com/problems/majority-element/
/// Easy - [array, hash table, divide and conquer, sorting, counting]
/// Given an array nums of size n, return the majority element.
/// The majority element is the element that appears more than ⌊n / 2⌋ times.
/// You may assume that the majority element always exists in the array.
pub struct Solution {}

impl Solution {
    /// Using the Boyer–Moore majority vote algorithm
    /// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
    /// Initialize the majority element and a counter.
    /// Loop over the elements.
    /// - If the element is the majority, increment the counter.
    /// - If not, decrement the counter.
    /// - If the counter reaches 0, set the current element as the majority.
    ///
    /// When a majority is guaranteed, no second confirming loop is needed.
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority = nums.first().expect("nums is empty");
        let mut count = 0;
        for el in nums.iter() {
            if el == majority {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    count = 1;
                    majority = el;
                }
            }
        }
        *majority
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0001() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
