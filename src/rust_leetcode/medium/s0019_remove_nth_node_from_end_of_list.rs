//! https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//! Medium - [linked-list, two-pointers]
//!
//! Given the head of a linked list,
//! remove the nth node from the end of the list and return its head.
//!
//! Example 1:
//! Input: head = [1,2,3,4,5], n = 2
//! Output: [1,2,3,5]
//! Example 2:
//! Input: head = [1], n = 1
//! Output: []
//! Example 3:
//! Input: head = [1,2], n = 1
//! Output: [1]
//!
//! Constraints:
//! - The number of nodes in the list is `sz`.
//! - `1 <= sz <= 30`
//! - `0 <= Node.val <= 100`
//! - `1 <= n <= sz`
//!
//! Follow up: Could you do this in one pass?

pub struct Solution {}

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

impl Solution {
    /// A fast and slow pointer solution.
    /// - Time complexity: `O(n)`.
    /// - Space complexity: `O(1)`.
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Get a mutable and immutable reference to head at the same time
        // by casting head to a raw pointer (head as *mut _) and back.
        // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/raw-pointers.html
        let mut slow: &mut Option<Box<ListNode>> = unsafe { &mut *(&mut head as *mut _) };
        let mut fast = &head;

        // Move the fast pointer n spots.
        for _ in 0..n {
            fast = &(fast.as_ref().unwrap().next);
        }

        // Exhausting the fast pointer before moving the slow pointer
        // means the first node needs to be skipped.
        if fast.is_none() {
            return head.unwrap().next;
        }

        // The slow pointer lags n spots behind and will be in
        // the correct spot whenever the fast one is exhausted.
        let dummy = &mut Box::new(ListNode::new(0));
        while fast.as_ref().unwrap_or(dummy).next.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }

        // Skip a node.
        slow.as_mut().unwrap().next = slow
            .as_mut()
            .unwrap()
            .next
            .take()
            .as_mut()
            .unwrap_or(dummy)
            .next
            .take();

        head
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_0019() {
        //           v
        // [1, 2, 3, 4, 5] -> [1, 2, 3, 5]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        let removed = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::remove_nth_from_end(list, 2), removed);

        //     v
        // [1, 2] -> [1]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let removed = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(Solution::remove_nth_from_end(list, 1), removed);

        //  v
        // [1, 2] -> [2]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let removed = Some(Box::new(ListNode { val: 2, next: None }));
        assert_eq!(Solution::remove_nth_from_end(list, 2), removed);

        //  v
        // [1] -> []
        let list = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(Solution::remove_nth_from_end(list, 1), None);
    }
}
