//! https://leetcode.com/problems/validate-binary-search-tree/
//! Medium - [tree, depth-first-search, binary-search-tree, binary-tree]
//!
//! Given the root of a binary tree, determine if it is a valid binary search tree (BST).
//! A valid BST is defined as follows:
//! - The left subtree of a node contains only nodes with keys strictly less than the node's key.
//! - The right subtree of a node contains only nodes with keys strictly greater than the node's key.
//! - Both the left and right subtrees must also be binary search trees.
//!
//! Example 1:
//! Input: root = [2,1,3]
//! Output: true
//! Example 2:
//! Input: root = [5,1,4,null,null,3,6]
//! Output: false
//! Explanation: The root node's value is 5 but its right child's value is 4.
//!
//! Constraints:
//! - The number of nodes in the tree is in the range [1, 104].
//! - -2**31 <= Node.val <= 2**31 - 1

use crate::util::binary_tree::OptNode;

pub struct Solution {}

impl Solution {
    /// Solve in O(n) time, with n the number of nodes.
    pub fn is_valid_bst<T: Copy + PartialOrd>(root: OptNode<T>) -> bool {
        Self::is_valid_node(&root, None, None)
    }

    /// Verify the following comparisons for a BST.
    /// ```text
    ///                |  l  |  g  | l < node < g
    ///     a        --|-----|-----|
    ///      \       a | -oo | +oo |
    ///       b      b |  a  | +oo |
    ///      / \     c |  a  |  b  |
    ///     c   f    d |  a  |  c  |
    ///    / \       e |  c  |  b  |
    ///   d   e      f |  b  | +oo |
    /// ```
    fn is_valid_node<T: Copy + PartialOrd>(
        node: &OptNode<T>,
        min: Option<T>,
        max: Option<T>,
    ) -> bool {
        match node.as_ref() {
            None => true,
            Some(node) => {
                let current = node.borrow();
                // a < c < b
                min.is_none_or(|min_| current.val > min_) &&
                max.is_none_or(|max_| current.val < max_) &&
                // a < d < c
                Self::is_valid_node(&current.left, min, Some(current.val)) &&
                // c < e < b
                Self::is_valid_node(&current.right, Some(current.val), max)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::binary_tree::TreeNode;

    use super::Solution;

    #[test]
    fn test_0098() {
        //   2
        //  / \
        // 1   3
        let root = TreeNode::from_vec(&[Some(2), Some(1), Some(3)]);
        assert!(Solution::is_valid_bst(root));

        //   5
        //  / \
        // 1   4 <- this is larger than its left child 3
        //    / \
        //   3   6
        let root = TreeNode::from_vec(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!Solution::is_valid_bst(root));

        //   5
        //  / \
        // 4   6
        //    / \
        //   3   7
        //   ^- this is smaller than the root 5
        let root = TreeNode::from_vec(&[Some(5), Some(4), Some(6), None, None, Some(3), Some(7)]);
        assert!(!Solution::is_valid_bst(root));

        let root = TreeNode::from_vec(&[Some(i32::MAX)]);
        assert!(Solution::is_valid_bst(root));
    }
}
