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
use std::cell::RefCell;
use std::rc::Rc;
pub type TreeNodeRef<T> = Rc<RefCell<TreeNode<T>>>;
pub type OptNode<T> = Option<TreeNodeRef<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: OptNode<T>,
    pub right: OptNode<T>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn new_optnode(val: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    // https://github.com/pymongo/leetcode-rust/blob/master/src/binary_tree/serde_binary_tree_to_leetcode_vec.rs#L42
    pub fn from_vec(data: &[Option<T>]) -> OptNode<T>
    where
        T: Copy,
    {
        if data.is_empty() {
            return None;
        }

        let nodes = data
            .iter()
            .map(|&val| val.and_then(|v| TreeNode::new_optnode(v)))
            .collect::<Vec<OptNode<T>>>();
        let root = nodes.first()?.clone();
        let mut child_node_ptr = 1;
        for node in nodes.iter().flatten() {
            if let Some(child_node) = nodes.get(child_node_ptr) {
                node.borrow_mut().left.clone_from(child_node);
                child_node_ptr += 1;
            } else {
                break;
            }
            if let Some(child_node) = nodes.get(child_node_ptr) {
                node.borrow_mut().right.clone_from(child_node);
                child_node_ptr += 1;
            } else {
                break;
            }
        }
        root
    }
}
