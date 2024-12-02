/// https://leetcode.com/problems/jump-game-ii/
/// Medium - [array, dynamic programming, greedy]
/// Given an array of integers, determine the mimimum number of jumps to reach
/// the final index starting from the first.
/// It is guaranteed that the destination is reachable.
/// Every array element represents the current maximal jump size.

pub struct Solution {}

impl Solution {
    fn jump_nested_loop(nums: &[usize]) -> usize {
        let mut n_jumps = 0;
        let mut i = 0;
        while i < nums.len() - 1 {
            let num = nums[i];
            if i + num >= nums.len() - 1 {
                n_jumps += 1;
                i = nums.len();
                continue;
            }
            let slice = &nums[(i + 1)..=(i + num)];
            i += 1 + slice
                .iter()
                .enumerate()
                .max_by_key(|(idx, &val)| val + idx)
                .unwrap()
                .0;
            n_jumps += 1;
        }
        n_jumps
    }

    pub fn jump(nums: &[isize]) -> isize {
        let mut current = 0;
        let mut next_jumps = 0;
        let mut n_jumps = 0;
        for new_jumps in &nums[..(nums.len() - 1)] {
            // We compare how many jumps our previous best still has with how
            // how many this new destination gives. We have to subtract 1 as a
            // jump get used to get here.
            // This allows us to keep track of the  current remaining jumps,
            // select the best destination along the way and start using that
            // best one even before knowing it is the best in a single loop.
            next_jumps = isize::max(next_jumps - 1, *new_jumps);
            // At this point, the next_jumps counter starts to take over.
            // We have to start looking for a new best destination to replenish
            // the next_jumps.
            if current == 0 {
                current = next_jumps;
                next_jumps = 0;
                n_jumps += 1;
            }
            // We use a jump to traverse this number.
            current -= 1;
        }
        n_jumps
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test_0045() {
        // Index: 0 -> 1 -> 4 = 2
        assert_eq!(Solution::jump(&[2, 3, 1, 1, 4]), 2);
        // Index: 0 -> 1 -> 4 = 2
        assert_eq!(Solution::jump(&[2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 0]), 2);
    }
}
