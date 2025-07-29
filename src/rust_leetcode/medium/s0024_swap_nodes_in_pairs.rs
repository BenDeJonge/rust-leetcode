//! https://leetcode.com/problems/swap-nodes-in-pairs/
//! Medium - [linked-list, recursion]
//!
//! Given alinked list, swap every two adjacent nodes and return its head.
//! You must solve the problem withoutmodifying the values in the list's nodes
//! (i.e., only nodes themselves may be changed.)
//!
//! Example 1:
//! Input: head = [1,2,3,4]
//! Output: [2,1,4,3]
//! Explanation:
//! Example 2:
//! Input: head = []
//! Output: []
//! Example 3:
//! Input: head = [1]
//! Output: [1]
//! Example 4:
//! Input: head = [1,2,3]
//! Output: [2,1,3]
//!
//! Constraints:
//! - The number of nodes in thelistis in the range `[0, 100]`.
//! - `0 <= Node.val <= 100`

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

macro_rules! next_as_mut {
    ($a: expr) => {
        $a.as_mut().unwrap().next
    };
}

impl Solution {
    /// Iteratively swap the pairs in:
    /// - Time complexity: `O(n)`
    /// - Space complexity: `O(1)`
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        // Example: [1, 2, 3, 4] = {1, {2, {3, {4, None}}}}
        while current.is_some() && current.as_ref().unwrap().next.is_some() {
            // Create intermediate nodes such that:
            // - current = {1, None}
            // - next = {2, None}
            // - tail = {3, {4, None}}
            let mut next = next_as_mut!(current).take();
            let tail = next_as_mut!(next).take();

            // Fill in their values:
            // - current = {1, {3, {4, None}}}
            next_as_mut!(current) = tail;
            // - next = {2, {1, {3, {4, None}}}}
            next_as_mut!(next) = current.take();
            // - current = {2, {1, {3, {4, None}}}}
            current.replace(next.unwrap());

            // Advance current twice for the next iteration:
            // - current = {3, {4, None}}
            current = &mut next_as_mut!(next_as_mut!(current));
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_leetcode::medium::s0024_swap_nodes_in_pairs::ListNode;

    use super::Solution;

    #[test]
    fn test_0024() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));
        let reversed = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::swap_pairs(list), reversed);

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let reversed = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        assert_eq!(Solution::swap_pairs(list), reversed);

        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let reversed = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(Solution::swap_pairs(list), reversed);

        let list = Some(Box::new(ListNode { val: 1, next: None }));
        let reversed = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(Solution::swap_pairs(list), reversed);

        assert_eq!(Solution::swap_pairs(None), None);
    }
}
