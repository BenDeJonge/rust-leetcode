//! https://leetcode.com/problems/merge-k-sorted-lists/
//! Hard - [linked list, divide and conquer, heap (priority queue), merge sort]
//! You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
//! Merge all the linked-lists into one sorted linked-list and return it.

use crate::util::singly_linked_list::ListNode;
use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // We need a data structure that can house n digits in order, with n the number of non-exhausted lists.
        // We can use a BINARY HEAP, of which we know the capacity in advance.
        let mut heap = <BinaryHeap<ListNode>>::with_capacity(lists.len());
        // Initialize the heap with the starting nodes from each non-exhausted list.
        for node in lists.into_iter().flatten() {
            heap.push(*node)
        }
        // Initialize a dummy node and the current node.
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        // There are still numbers left to classify.
        // We get the next smallest number in O(log(n)) time, with n the number non-exhausted lists.
        while let Some(node) = heap.pop() {
            // Get the minimum value from the heap.
            let new_node = Box::new(ListNode::new(node.val));
            current.next = Some(new_node);
            current = current.next.as_mut().unwrap();
            // The list is not yet finished. Add the next value to the heap, which is automatically sorted in O(1).
            if node.next.is_some() {
                heap.push(*node.next.unwrap());
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    #[test]
    fn test_0023() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                ListNode::from_vec(&vec![1, 4, 5]),
                ListNode::from_vec(&vec![1, 3, 4]),
                ListNode::from_vec(&vec![2, 6])
            ]),
            ListNode::from_vec(&vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), ListNode::from_vec(&vec![]));
        assert_eq!(
            Solution::merge_k_lists(vec![ListNode::from_vec(&vec![])]),
            ListNode::from_vec(&vec![])
        );
    }
}
