//! https://leetcode.com/problems/maximum-depth-of-binary-tree/
//! Easy - [tree, depth-first search, breadth-first search, binary tree]
//! Given the root of a binary tree, return its maximum depth.
//! A binary tree's maximum depth is the number of nodes along the longest path
//! from the root node down to the farthest leaf node.

use crate::util::binary_tree::OptNode;
use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn max_depth<T>(root: OptNode<T>) -> i32 {
        match root {
            Some(node) => {
                // Move semantics instead of clone, since we own the root anyway.
                let mut node_ref = node.borrow_mut();
                cmp::max(
                    Self::max_depth(node_ref.left.take()),
                    Self::max_depth(node_ref.right.take()),
                ) + 1
            }
            // There is no tree.
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::binary_tree::TreeNode;
    #[test]
    fn test_0104() {
        let tree1 = TreeNode::from_vec(&[
            Some(3),  //
            Some(9),  //    3
            Some(20), //   / \
            None,     //  9   20
            None,     //     / \
            Some(15), //    15  7
            Some(7),  //
        ]);
        assert_eq!(Solution::max_depth(tree1), 3);

        let tree2 = TreeNode::from_vec(&[
            Some(1), // 1
            None,    //  \
            Some(2), //   2
        ]);
        assert_eq!(Solution::max_depth(tree2), 2);
    }
}
