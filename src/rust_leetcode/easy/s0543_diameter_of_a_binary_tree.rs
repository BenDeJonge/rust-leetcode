//! https://leetcode.com/problems/diameter-of-binary-tree//problems/move-zeroes/
//! Easy - [tree, depth-first search, binary tree]
//! Given the root of a binary tree, return the diameter of the tree.
//! The diameter is defined as the longest path, counted as the number of edges,
//! between any two nodes which may or may not pass through the root.

use crate::util::binary_tree::OptNode;
use std::cmp::max;

pub struct Solution {}

impl Solution {
    /// Compute the diameter of a tree, defined as the longest path by number of
    /// edges, by computing the heights of the left and right subtrees.
    pub fn diameter_of_binary_tree<T>(root: OptNode<T>) -> isize {
        let mut diameter = 0isize;
        Self::height(&root, &mut diameter);
        diameter
    }

    /// Compute the height of a tree by recursively computing the height of the
    /// left and right subtrees.
    ///
    /// For the following tree, the height is 3.
    ///       1
    ///      / \
    ///     2   3
    ///    / \
    ///   4   5
    ///  /
    /// 6
    pub fn height<T>(root: &OptNode<T>, current: &mut isize) -> isize {
        if let Some(n) = root {
            let node = n.borrow();
            let left = Self::height(&node.left, current);
            let right = Self::height(&node.right, current);
            *current = max(*current, 2 + left + right);
            1 + max(left, right)
        }
        // There is no root and thus no tree. This distinguishes between a tree
        // without children (height 0) and an empty tree (height 1).
        else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::binary_tree::TreeNode;

    #[test]
    fn test_0543() {
        let tree_1 = TreeNode::from_vec(&[Some(1), Some(2), Some(3), Some(4), Some(5), None, None]);
        assert_eq!(Solution::diameter_of_binary_tree(tree_1), 3);
        let tree_2 = TreeNode::from_vec(&[Some(1), Some(2), None]);
        assert_eq!(Solution::diameter_of_binary_tree(tree_2), 1);
        // TODO: this does not pass as the longest path does not pass through the root.
        let tree_3 = TreeNode::from_vec(&[
            Some(4),
            Some(-7),
            Some(-3),
            None,
            None,
            Some(-9),
            Some(-3),
            Some(9),
            Some(-7),
            Some(-4),
            None,
            Some(6),
            None,
            Some(-6),
            Some(-6),
            None,
            None,
            Some(0),
            Some(6),
            Some(5),
            None,
            Some(9),
            None,
            None,
            Some(-1),
            Some(-4),
            None,
            None,
            None,
            Some(-2),
        ]);
        assert_eq!(Solution::diameter_of_binary_tree(tree_3), 8);
    }
}
