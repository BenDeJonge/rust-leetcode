//! https://leetcode.com/problems/kth-smallest-element-in-a-bst/
//! Medium - [tree, depth-first-search, binary-search-tree, binary-tree]
//!
//! Given the root of a binary search tree, and an integer k,
//! return the kth smallest value (1-indexed) of all the values of the nodes
//! in the tree.
//!
//! Example 1:
//! Input: root = [3,1,4,null,2], k = 1
//! Output: 1
//! Example 2:
//! Input: root = [5,3,6,2,4,null,null,1], k = 3
//! Output: 3
//!
//! Constraints:
//! - The number of nodes in the tree is n.
//! - 1 <= k <= n <= 104
//! - 0 <= Node.val <= 104
//!
//! Follow up: If the BST is modified often (i.e., we can do insert and delete
//! operations) and you need to find the kth smallest frequently,
//! how would you optimize?

use crate::util::binary_tree::{OptNode, TreeNode, TreeNodeRef};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
    pub fn kth_smallest(root: OptNode<i32>, k: i32) -> i32 {
        Solution::kth_smallest_ref(root.as_ref(), k as usize)
    }

    pub fn kth_smallest_ref<T: Copy>(root: Option<&Rc<RefCell<TreeNode<T>>>>, k: usize) -> T {
        let mut sol = None;
        let mut k = k;
        Solution::visit(root.unwrap(), &mut k, &mut sol);
        sol.unwrap()
    }

    /// Explore a binary search tree, for which holds `c >= a >= b`:
    /// ```text
    ///   a
    ///  / \
    /// b   c
    /// ```
    /// To find the k-th smallest element, nodes are explored in non-decreasing order: `b -> a -> c`.
    fn visit<T: Copy>(node: &TreeNodeRef<T>, k: &mut usize, sol: &mut Option<T>) {
        if sol.is_some() {
            return;
        }
        let current = node.borrow();

        // Explore the left subtree.
        if let Some(left) = &current.left {
            Solution::visit(left, k, sol);
        }
        // Explore the node itself.
        if sol.is_none() {
            *k -= 1;
            if *k == 0 {
                *sol = Some(current.val);
            }
        }
        // Explore the right subtree.
        if let Some(right) = &current.right {
            Solution::visit(right, k, sol);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::{OptNode, TreeNode};

    use super::Solution;

    #[test]
    fn test_0230() {
        let root: OptNode<i32> = TreeNode::from_vec(&[Some(3), Some(1), Some(4), None, Some(2)]);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 1), 1);

        let root: OptNode<i32> = TreeNode::from_vec(&[
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 1), 1);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 2), 2);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 3), 3);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 4), 4);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 5), 5);
        assert_eq!(Solution::kth_smallest_ref(root.as_ref(), 6), 6);
    }
}
