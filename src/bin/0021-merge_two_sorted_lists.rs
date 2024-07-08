/// https://leetcode.com/problems/merge-two-sorted-lists/description/
/// Easy - [linked list, recursion]
/// You are given the heads of two sorted linked lists list1 and list2.
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
/// Return the head of the merged linked list.

pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

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
fn main() {}
