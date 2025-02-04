//! https://leetcode.com/problems/symmetric-tree/
//! Easy - [tree, depth-first search, breadth-first search, binary tree]
//! Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

use crate::util::binary_tree::{OptNode, TreeNode};
use std::cell::Ref;

pub struct Solution {}

impl Solution {
    pub fn is_symmetric<T: std::cmp::PartialEq>(root: OptNode<T>) -> bool {
        if let Some(r) = &root {
            let broot: Ref<TreeNode<T>> = r.borrow();
            Self::is_symmetric_helper(&broot.left, &broot.right)
        }
        // An empty tree is symmetric.
        else {
            true
        }
    }

    pub fn is_symmetric_helper<T: std::cmp::PartialEq>(
        left: &OptNode<T>,
        right: &OptNode<T>,
    ) -> bool {
        match (left, right) {
            // An empty tree is symmetric.
            (None, None) => true,
            // A tree with both values may be symmetric.
            (Some(l), Some(r)) => {
                let (bl, br) = (l.borrow(), r.borrow());
                // The branches have the same value ...
                bl.val == br.val
				// ... but do their children too?
				&& Self::is_symmetric_helper(&bl.left, &br.right)
                    && Self::is_symmetric_helper(&bl.right, &br.left)
            }
            // A tree with only one value cannot be symmetric.
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::binary_tree::TreeNode;

    #[test]
    fn test_101() {
        let tree1 = TreeNode::from_vec(&[
            Some(1), //
            Some(2), //    1
            Some(2), //   / \
            Some(3), //  2   2
            Some(4), // / \ / \
            Some(4), // 3 4 4 3
            Some(3), //
        ]);
        assert!(Solution::is_symmetric(tree1));
        let tree2 = TreeNode::from_vec(&[
            Some(1), //
            Some(2), //    1
            Some(2), //   / \
            None,    //  2   2
            Some(3), //   \   \
            None,    //   3   3
            Some(3), //
        ]);
        assert!(!Solution::is_symmetric(tree2));
    }
}
