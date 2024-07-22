/// https://leetcode.com/problems/symmetric-tree/
/// Easy - [tree, depth-first search, breadth-first search, binary tree]
/// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
use crate::util::binary_tree::{OptNode, TreeNode};
pub struct Solution {}
use std::cell::Ref;
impl Solution {
    pub fn is_symmetric(root: OptNode) -> bool {
        if let Some(r) = &root {
            let broot: Ref<TreeNode> = r.borrow();
            Self::is_symmetric_helper(&broot.left, &broot.right)
        }
        // An empty tree is symmetric.
        else {
            true
        }
    }

    pub fn is_symmetric_helper(left: &OptNode, right: &OptNode) -> bool {
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_101() {
        //
        let tree = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        }));
        assert!(Solution::is_symmetric(Some(tree)));
    }
}
