//! https://leetcode.com/problems/palindrome-linked-list/
//! Easy - [linked-list, two-pointers, stack, recursion]
//!
//! Given the head of a singly linked list,
//! return true if it is a palindrome or false otherwise.
//!
//! Example 1:
//! Input: `head = [1,2,2,1]`
//! Output: `true`
//! Example 2:
//! Input: `head = [1,2]`
//! Output: `false`
//!
//! Constraints:
//! - The number of nodes in the list is in the range [1, 10^5].
//! - `0 <= Node.val <= 9`
//!
//! Follow up: Could you do it in `O(n)` time and `O(1)` space?

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

impl Solution {
    /// Check if a linked list is a palindrome.
    ///
    /// **WARNING:** the input list is mutated (halved in length).
    ///
    /// - Time complexity: `O(n)`.
    /// - Space complexity: `O(1)`.
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // Edge case: [] and [1,] are palindromes.
        if head.as_ref().is_none() || head.as_ref().unwrap().next.is_none() {
            return true;
        }

        let mut len = 0usize;
        let mut current = &head;
        while let Some(node) = current {
            current = &node.next;
            len += 1;
        }

        // len = 4 -> until index (4 - 1) / 2 = 1 -> 2 elements
        // len = 5 -> until index (5 - 1) / 2 = 2 -> 3 elements
        // The first list can be one element longer.
        let mut midpoint = &mut head;
        for _ in 0..(len - 1) / 2 {
            midpoint = &mut midpoint.as_mut().unwrap().next;
        }

        let mut to_be_reversed = midpoint.as_mut().unwrap().next.take();
        let mut right = None;
        while let Some(mut node) = to_be_reversed {
            to_be_reversed = node.next;
            node.next = right;
            right = Some(node);
        }

        let mut left = head;
        while let (Some(l), Some(r)) = (left, right) {
            if l.val != r.val {
                return false;
            }
            left = l.next;
            right = r.next;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_leetcode::easy::s0234_palindrome_linked_list::ListNode;

    use super::Solution;

    #[test]
    fn test_0234() {
        // [1, 2, 3, 2, 1,]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })),
        }));
        assert!(Solution::is_palindrome(list));

        // [1, 2, 2, 1,]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert!(Solution::is_palindrome(list));

        // [1, 1,]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert!(Solution::is_palindrome(list));

        // [1,]
        let list = Some(Box::new(ListNode { val: 1, next: None }));
        assert!(Solution::is_palindrome(list));

        // []
        assert!(Solution::is_palindrome(None));

        // [1, 2, 3, 1,]
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert!(!Solution::is_palindrome(list));
    }
}
