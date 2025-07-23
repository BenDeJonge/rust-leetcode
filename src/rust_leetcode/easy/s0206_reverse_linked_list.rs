//! https://leetcode.com/problems/reverse-linked-list/
//! Easy - [linked-list, recursion]
//!
//! Given the head of a singly linked list return the reversed list.
//!
//! Example 1:
//! Input: `head = [1,2,3,4,5]`
//! Output: `[5,4,3,2,1]`
//! Example 2:
//! Input: `head = [1,2]`
//! Output: `[2,1]`
//! Example 3:
//! Input: `head = []`
//! Output: `[]`
//!
//! Constraints:
//! - The number of nodes in the list is the range `[0, 5000]`.
//! - `-5000 <= Node.val <= 5000`
//!
//! Follow up: A linked list can be reversed either iteratively or recursively.
//! Could you implement both?

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

pub struct Solution {}

impl Solution {
    /// Reverse a linked list in:
    /// - Time complexity: `O(n)`.
    /// - Auxiliary space: `O(1)`.
    ///
    /// With `n` the number of nodes in the linked list.
    ///
    /// | Diagram                     | Previous | Current |
    /// |-----------------------------|----------|---------|
    /// | None    1 -> 2 -> 3 -> None | None     | 1       |
    /// | None <- 1    2 -> 3 -> None | 1        | 2       |
    /// | None <- 1 <- 2    3 -> None | 2        | 3       |
    /// | None <- 1 <- 2 <- 3    None | 3        | None    |
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;
        while let Some(mut node) = current {
            // Because of move semantics, commenting any of these lines is a
            // compiler error. Since ownership of all pointers is transfered to
            // a new variable for all three, new values must be set before the
            // next iteration of the loop.
            current = node.next;
            node.next = previous;
            previous = Some(node);
        }
        previous
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_leetcode::easy::s0206_reverse_linked_list::ListNode;

    use super::Solution;

    #[test]
    fn test_0206() {
        assert_eq!(Solution::reverse_list(None), None);

        // 1 -> 2
        let linked = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        let reverse = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(Solution::reverse_list(linked), reverse);

        // 1 -> 2 -> 3 -> 4 -> 5
        let linked = Some(Box::new(ListNode {
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
        let reverse = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(Solution::reverse_list(linked), reverse);
    }
}
