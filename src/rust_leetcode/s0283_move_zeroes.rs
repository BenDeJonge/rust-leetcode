/// https://leetcode.com/problems/move-zeroes/
/// Easy - [array, two pointer]
/// Given an integer array nums, move all 0's to the end of it
/// while maintaining the relative order of the non-zero elements.
/// Note that you must do this in-place without making a copy of the array.
pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut start = 0;
        let mut stop = nums.len() - 1;
        while start < stop {
            // When we encounter a 0, we put it at the end.
            // This means that we can put our stop pointer one more to the front, as the last element
            // is now definitely a 0 that we do not want to check again.
            if nums[start] == 0 {
                nums.remove(start);
                nums.push(0);
                stop -= 1;
            }
            // For any other number, we can advance the start pointer.
            // We do not want to do this when we encounter a 0, as the cutting operation of the 0
            // already acts as "advancing" the start pointer by moving the array forward.
            else {
                start += 1;
            }
        }
    }

    pub fn move_zeroes_ideomatic(nums: &mut Vec<i32>) {
        // Temporary container for the zeros.
        let mut n_zeros = 0;
        nums.retain(|&el| {
            // Keep all non-zero elements.
            if el != 0 {
                true
            }
            // Count the zeros.
            else {
                n_zeros += 1;
                false
            }
        });
        // Append the required number of zeros.
        // The advantage is that all unneeded elements are removed in one go,
        // as removing from an array is not cheap.
        // Additionally, no reallocations need to happen for the zeros, as these are not pushed incrementally.
        nums.append(&mut vec![0; n_zeros]);
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;

    #[test]
    fn test_0001() {
        let mut v1 = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v1);
        assert_eq!(v1, vec![1, 3, 12, 0, 0]);
        let mut v2 = vec![0];
        Solution::move_zeroes(&mut v2);
        assert_eq!(v2, vec![0]);
        let mut v3 = vec![0, 0, 1];
        Solution::move_zeroes(&mut v3);
        assert_eq!(v3, vec![1, 0, 0]);
    }
}
