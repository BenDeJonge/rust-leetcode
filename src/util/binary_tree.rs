use std::cell::RefCell;
use std::rc::Rc;

/// Explaining the datastructure of the ThreeNode
/// Option: we do not know if any given node will have a left or right child.
/// Rc: by definition, a TreeNode is a cyclical datastructure i.e., its attributes reference itself. Using a Reference
/// Counter Rc<T>, we can have shared (non-exclusive) ownership of the same data at multiple locations in the code. In
/// the case of a binary tree, a node can be the owner of two other nodes (left and right). The data is then allocated
/// on the heap. Additionally, Rc<T> automatically dereferences to T, so we have access to T's attributes and methods.
/// RefCell: Rust's native references, &, are tracked entirely statically at compile time. However, only one mutable
/// reference may exist at any time. This is difficult when working with a recursive data structure. It is more
/// convenient to use a RefCell<T>, which allows interior mutability i.e., mutating the contents through a immutable
/// reference &T instead of an exclusive reference &mut T. Borrows are still checked at runtime to ensure memory safety.
pub type TreeNodeRef = Rc<RefCell<TreeNode>>;
pub type OptNode = Option<TreeNodeRef>;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: OptNode,
    pub right: OptNode,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
