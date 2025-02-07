//! https://leetcode.com/problems/permutations/
//! Medium - [array, backtracking]
//! Given an array of 1 to 8 numbers between -10 and 10,
//! compute all permutations. There might be duplicate numbers.

pub struct Solution {}

impl Solution {
    /// An implementation of [Heap's Algorithm](
    /// https://en.wikipedia.org/wiki/Heap%27s_algorithm),
    /// which computes all $k!$ permutations of a $k$-length vector.
    pub fn heap_algorithm<T: Copy>(k: usize, vec: &mut [T], sol: &mut Vec<Vec<T>>) {
        if k == 1 {
            sol.push(vec.to_owned());
            return;
        }
        for i in 0..k {
            Self::heap_algorithm(k - 1, vec, sol);
            if k % 2 == 0 {
                vec.swap(i, k - 1);
            } else {
                vec.swap(0, k - 1);
            }
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sol = Vec::with_capacity((1..=nums.len()).product());
        Self::heap_algorithm(nums.len(), &mut nums, &mut sol);
        // Remove duplicates.
        sol.sort();
        sol.dedup();
        sol
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_permutations() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
        assert_eq!(
            Solution::permute(vec![1, 1, 2]),
            [[1, 1, 2], [1, 2, 1], [2, 1, 1],]
        );
    }
}
