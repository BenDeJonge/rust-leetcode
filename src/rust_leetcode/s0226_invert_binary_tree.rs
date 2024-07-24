/// https://leetcode.com/problems/invert-binary-tree/
/// Easy - [tree, depth-first search, breadth-first search, binary tree]
/// Given the root of a binary tree, invert the tree, and return its root.
use crate::util::binary_tree::OptNode;

pub struct Solution {}
impl Solution {
    pub fn invert_tree<T>(root: OptNode<T>) -> OptNode<T> {
        // There is some subtree.
        // Match to a reference of the root, so we can return root at the end without Copy/Clone.
        if let Some(ref node) = root {
            // A RefCell keeps track of outstanding borrows and includes a guard type, letting references signal to the
            // original RefCell when they go out of scope. It can be compared to a mutable reference (&mut), however the
            // guard itself (the RefMut) still needs to be declared as mutable.
            let mut node_ref = node.borrow_mut();
            // We have now borrowed the node as mutable (node_ref). However, we cannot move certain attributes (left and
            // right) out of this reference, this would lead to multiple owners of the same data or a null pointer in
            // the original owner.
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();
            // Now the original node was reversed. Let's also reverse all child nodes recursively.
            node_ref.left = Self::invert_tree(right);
            node_ref.right = Self::invert_tree(left);
        }
        root
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;
    use crate::util::binary_tree::TreeNode;

    #[test]
    fn test_0226() {
        let tree1 = &[
            Some(4), //
            Some(2), //    4
            Some(7), //   / \
            Some(1), //  2   7
            Some(3), // / \ / \
            Some(6), // 1 3 6 9
            Some(9), //
        ];
        let tree2 = &[
            Some(4), //
            Some(7), //    4
            Some(2), //   / \
            Some(9), //  7   2
            Some(6), // / \ / \
            Some(3), // 9 6 3 1
            Some(1), //
        ];
        assert_eq!(
            Solution::invert_tree(TreeNode::from_vec(tree1)),
            TreeNode::from_vec(tree2)
        )
    }
}
