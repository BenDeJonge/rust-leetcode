/// https://leetcode.com/problems/binary-tree-inorder-traversal/
/// Easy - [stack, tree, depth-first search, binary tree]
/// Given the root of a binary tree, return the inorder of its nodes' values.
use crate::util::binary_tree::OptNode;
pub struct Solution {}

impl Solution {
    pub fn inorder_traversal<T: Copy>(root: OptNode<T>) -> Vec<T> {
        let mut result = <Vec<T>>::new();
        Self::inorder_traversal_helper(&root, &mut result);
        result
    }

    /// ```mermaid
    /// flowchart TD
    /// 1 --> 2
    /// 2 --> 4
    /// 4 --> 8
    /// 4 --> 9
    /// 2 --> 5
    /// 5 --> 5l[.]
    /// 5 --> 5r[.]
    /// 1 --> 3
    /// 3 --> 6
    /// 6 --> 6l[.]
    /// 6 --> 10
    /// 3 --> 7
    /// 7 --> 11
    /// 7 --> 12
    /// ```
    /// 8 -> 4 -> 9 -> 2 -> 5 -> 6 -> 10 -> 3 -> 11 -> 7 -> 12
    /// - Traverse clockwise (left-center-right)
    /// OR
    /// - Visit leftmost subtree
    /// - visit root
    /// - visit other subtrees left to right
    pub fn inorder_traversal_helper<T: Copy>(node: &OptNode<T>, v: &mut Vec<T>) {
        if let Some(n) = node {
            let b = n.borrow();
            Self::inorder_traversal_helper(&b.left, v);
            // After recursing all the left children, proceed with pushing the value.
            // This will be the value of the nodes read inorder.
            v.push(b.val);
            // Continue with the right branch until there are no more left nodes.
            // At that point, the value of this right node will also be pushed.
            Self::inorder_traversal_helper(&b.right, v);
        }
        // There is no Node passed in. We can proceed beyond recursing on the left child.
    }
}

#[cfg(test)]

mod tests {
    use super::Solution;
    use crate::util::binary_tree::TreeNode;

    #[test]
    fn test_0094() {
        let tree1 = TreeNode::from_vec(&[
            Some(1), // 1
            None,    //  \
            Some(2), //   2
            Some(3), //  /
                     // 3
        ]);
        assert_eq!(Solution::inorder_traversal(tree1), vec![1, 3, 2]);
        let tree2 = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(Solution::inorder_traversal(tree2), vec![1]);
        let tree3 = TreeNode::<i32>::from_vec(&[]);
        assert_eq!(Solution::inorder_traversal::<i32>(tree3), <Vec<i32>>::new());
    }
}
