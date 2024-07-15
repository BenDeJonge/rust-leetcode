/// https://leetcode.com/problems/add-two-numbers/
/// Medium - [linked list, math, recursion]
/// You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
use crate::util::singly_linked_list::ListNode;
pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            // Both lists are exhausted simultaneously.
            (None, None) => None,
            // One list is exhausted.
            (Some(node), None) | (None, Some(node)) => Some(node),
            // Both lists still have numbers.
            (Some(node1), Some(node2)) => {
                let sum = node1.val + node2.val;
                // We do not need to carry over anything.
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(node1.next, node2.next),
                    }))
                }
                // We need to carry over the one.
                else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        // Move the carry.
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            // Recursively add the carry to the first node's value.
                            Solution::add_two_numbers(carry, node1.next),
                            // Add the second node's value.
                            node2.next,
                        ),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_0002() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(&vec![2, 4, 3]),
                ListNode::from_vec(&vec![5, 6, 4])
            ),
            ListNode::from_vec(&vec![7, 0, 8])
        );
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_vec(&vec![0]), ListNode::from_vec(&vec![0])),
            ListNode::from_vec(&vec![0])
        );
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_vec(&vec![9, 9, 9, 9, 9, 9, 9]),
                ListNode::from_vec(&vec![9, 9, 9, 9])
            ),
            ListNode::from_vec(&vec![8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
