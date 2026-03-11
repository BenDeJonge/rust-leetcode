//! https://leetcode.com/problems/binary-tree-level-order-traversal/
//! Medium - [tree, breadth-first-search, binary-tree]
//!
//! Given the root of a binary tree, return the level order traversal of its nodes' values
//! (i.e., from left to right, level by level).
//!
//! Example 1:
//! Input: root = [3,9,20,null,null,15,7]
//! Output: [[3],[9,20],[15,7]]
//! Example 2:
//! Input: root = [1]
//! Output: [[1]]
//! Example 3:
//! Input: root = []
//! Output: []
//!
//! Constraints:
//! - The number of nodes in the tree is in the range [0, 2000].
//! - -1000 <= Node.val <= 1000

use crate::util::binary_tree::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn level_order<T: Copy>(root: Option<&TreeNodeRef<T>>) -> Vec<Vec<T>> {
        let mut res = vec![];
        Self::visit(root, 0, &mut res);
        res
    }

    fn visit<T: Copy>(node: Option<&TreeNodeRef<T>>, level: usize, res: &mut Vec<Vec<T>>) {
        if let Some(current) = node {
            let borrow = current.borrow();
            if res.len() <= level {
                res.push(vec![]);
            }
            res[level].push(borrow.val);
            Self::visit(borrow.left.as_ref(), level + 1, res);
            Self::visit(borrow.right.as_ref(), level + 1, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::binary_tree::TreeNode;

    #[test]
    fn test_0102() {
        let root = TreeNode::from_vec(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(
            Solution::level_order(root.as_ref()),
            &[vec![3], vec![9, 20], vec![15, 7]]
        );

        let root = TreeNode::from_vec(&[Some(1)]);
        assert_eq!(Solution::level_order(root.as_ref()), &[vec![1]]);

        let root = TreeNode::from_vec(&Vec::<Option<i32>>::new());
        assert_eq!(Solution::level_order(root.as_ref()), Vec::<Vec<i32>>::new());
    }
}
