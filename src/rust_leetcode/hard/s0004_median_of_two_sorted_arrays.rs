//! https://leetcode.com/problems/median-of-two-sorted-arrays/
//! Hard - [array, binary search, divide and conquer]
//! Given two sorted arrays nums1 and nums2 of size `m` and `n` respectively,
//! return the median of the two sorted arrays.
//! The overall run time complexity should be `O(log(m+n))`.

pub struct Solution {}
impl Solution {
    /// A binary-search solution that finds the median in `O(log(n + m))`.
    /// The reasoning is the same, but rather than moving both pointers linearly
    /// we jump them around as in a binary search.
    /// The goal is to find the two numbers in each array that, if the arrays
    /// would be grouped together, would be the four central numbers.
    pub fn find_median_sorted_arrays_logarithmic<N: Copy + Into<i32>>(
        nums1: &[N],
        nums2: &[N],
    ) -> f64 {
        // Ensure that nums1 is the shorter array.
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays_logarithmic(nums2, nums1);
        }
        let n_total = nums1.len() + nums2.len();
        let n_needed = n_total.div_ceil(2);
        let mut left = 0;
        let mut right = nums1.len();

        // For each array, set up:
        // - the index at which we split.
        let mut mid1: usize;
        let mut mid2: usize;
        // - the value left of the split.
        let mut l1: i32;
        let mut l2: i32;
        // - the value right of the split.
        let mut r1: i32;
        let mut r2: i32;

        while left <= right {
            // The two arrays combined must house half of all the elements.
            // For performance, the first (short) array is bisected and the
            // corresponding fraction of the second (long) array is computed.
            mid1 = (left + right) / 2;
            mid2 = n_needed - mid1;

            // We can potentially partition at the edge of either array,
            // meaning none of its elements are grabbed.
            // If that is the case, that value is set to the edge of the i32
            // spectrum to avoid using it in the eventual median calculation.
            if mid1 < nums1.len() {
                r1 = nums1[mid1].into();
            } else {
                r1 = i32::MAX;
            }
            if mid2 < nums2.len() {
                r2 = nums2[mid2].into();
            } else {
                r2 = i32::MAX;
            }
            if mid1 >= 1 {
                l1 = nums1[mid1 - 1].into()
            } else {
                l1 = i32::MIN;
            }
            if mid2 >= 1 {
                l2 = nums2[mid2 - 1].into();
            } else {
                l2 = i32::MIN;
            }

            // We need to find 4 numbers (2 out of each array) such that these
            // would be the central numbers of the merged array.
            // `.. .. l1 l2 r1 r2 .. ..`
            // Because both arrays are sorted, we know:
            // `l1 <= r1 && l2 <= r2`
            // To guarantee the above ordering, we have to enforce:
            // `l1 <= r2 && l2 <= r1`
            // However, we have no information regarding the orderings:
            // `l1 <-> l2` or `r1 <-> r2`
            if l1 <= r2 {
                // Both paritions are good, we found the median.
                if l2 <= r1 {
                    return Self::calc_median(l1, l2, r1, r2, n_total);
                }
                // The partition of nums1 is too small.
                else {
                    left = mid1 + 1;
                }
            }
            // The partition of nums1 is too big.
            else {
                right = mid1 - 1;
            }
        }
        unreachable!("expected arrays to be sorted")
    }

    fn calc_median(l1: i32, l2: i32, r1: i32, r2: i32, total: usize) -> f64 {
        // We know the following:
        // - `l1 <= r1 && l2 <= r2`
        // - `l1 <= r2 && l2 <= r1`
        // However, we have no information regarding the orderings:
        // `l1 <-> l2` or `r1 <-> r2`
        // The rightmost number of `l1 l2` is the median.
        // `.. l1 l2 r1 r2`
        if total % 2 != 0 {
            l1.max(l2) as f64
        }
        // Find the middle two numbers (one l and one r) and average them.
        else {
            (l1.max(l2) + r1.min(r2)) as f64 / 2.0
        }
    }

    /// A two-pointer solution that finds the median in `O(n + m)`.
    pub fn find_median_sorted_arrays_linear<N: Copy + Into<i32>>(nums1: &[N], nums2: &[N]) -> f64 {
        let mut left = 0;
        let mut right = 0;

        let mut median1 = 0i32;
        let mut median2 = 0i32;

        let mut num_l: i32;
        let mut num_r: i32;

        let i_median = (nums1.len() + nums2.len()) / 2 + 1;
        while left + right < i_median {
            median2 = median1;
            // Can advance both pointers, so advance the smaller one.
            if left < nums1.len() && right < nums2.len() {
                num_l = nums1[left].into();
                num_r = nums2[right].into();
                if num_l < num_r {
                    median1 = num_l;
                    left += 1;
                } else {
                    median1 = num_r;
                    right += 1;
                }
            }
            // Can only advance the left pointer.
            else if left < nums1.len() {
                median1 = nums1[left].into();
                left += 1;
            }
            // Can only advance the right pointer.
            else {
                median1 = nums2[right].into();
                right += 1;
            }
        }

        if (nums1.len() + nums2.len()) % 2 != 0 {
            median1 as f64
        } else {
            (median1 + median2) as f64 / 2.0
        }
    }

    /// Find the median in `O((n + m) + log(n + m))`.
    ///
    /// This complexity comes from the fact that sorting is `O(k log(k))`,
    /// where `k = n + m` as both vectors have been combined first.
    pub fn find_median_sorted_arrays_naive<N: Copy + Into<i32>>(nums1: &[N], nums2: &[N]) -> f64 {
        let mut nums: Vec<i32> = nums1.iter().chain(nums2).map(|x| (*x).into()).collect();
        nums.sort();
        if nums.len() % 2 != 0 {
            nums[nums.len() / 2] as f64
        } else {
            (nums[nums.len() / 2 - 1] + nums[nums.len() / 2]) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn test_helper<N: Copy + Into<i32>>(nums1: &[N], nums2: &[N], expected: f64) {
        assert_eq!(
            Solution::find_median_sorted_arrays_naive(nums1, nums2),
            expected
        );
        assert_eq!(
            Solution::find_median_sorted_arrays_linear(nums1, nums2),
            expected
        );
        assert_eq!(
            Solution::find_median_sorted_arrays_logarithmic(nums1, nums2),
            expected
        );
    }

    #[test]
    fn test_0004() {
        // [1 4 5 8 10 12 16]
        test_helper(&[1, 4, 10], &[5, 8, 12, 16], 8.0);
        // [1 1 2 3 3 4 5 5 6 7 7 8 9 9 11 13 15]
        test_helper(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[1, 3, 5, 7, 9, 11, 13, 15],
            6.0,
        );
        // [1 2 3 4]
        test_helper(&[1, 2], &[3, 4], 2.5);
        // [1 2 3]
        test_helper(&[1, 3], &[2], 2.0);
    }
}
