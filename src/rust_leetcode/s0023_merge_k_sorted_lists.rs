/// https://leetcode.com/problems/merge-k-sorted-lists/
/// Hard - [linked list, divide and conquer, heap (priority queue), merge sort]
/// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
/// Merge all the linked-lists into one sorted linked-list and return it.
use std::collections::BinaryHeap;

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

    // Convenience method to allow parsing of vecs.
    fn from_vec(vec: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        for &v in vec {
            let node = ListNode::new(v);
            current.next = Some(Box::new(node));
            current = current.next.as_mut().unwrap();
        }
        dummy.next
    }
}

// This implementation is needed for Ord.
impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<std::cmp::Ordering> {
        Some(other.val.cmp(&self.val))
    }
}

// This implementation is needed to allow sorting of ListNodes on a BinaryHeap.
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

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
