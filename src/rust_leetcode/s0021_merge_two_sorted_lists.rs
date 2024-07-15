/// https://leetcode.com/problems/merge-two-sorted-lists/
/// Easy - [linked list, recursion]
/// You are given the heads of two sorted linked lists list1 and list2.
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
/// Return the head of the merged linked list.
use crate::util::singly_linked_list::ListNode;
pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            // Both lists are empty.
            (None, None) => None,
            // One list is empty, so we can put the other as the next value.
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            // Neither list is empty, so we need to compare values.
            // We can do this recursively, keeping the call stack until we reach any of the above base cases.
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    Some(Box::new(ListNode {
                        next: Solution::merge_two_lists(l1.next, Some(l2)),
                        val: l1.val,
                    }))
                } else {
                    Some(Box::new(ListNode {
                        next: Solution::merge_two_lists(Some(l1), l2.next),
                        val: l2.val,
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
    fn test_0021() {
        assert_eq!(
            Solution::merge_two_lists(
                ListNode::from_vec(&vec![1, 2, 4]),
                ListNode::from_vec(&vec![1, 3, 4])
            ),
            ListNode::from_vec(&vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&vec![]), ListNode::from_vec(&vec![])),
            ListNode::from_vec(&vec![])
        );
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(&vec![]), ListNode::from_vec(&vec![0])),
            ListNode::from_vec(&vec![0])
        );
    }
}
