// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // Convenience method to allow parsing of vecs.
    pub fn from_vec(vec: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        for &v in vec {
            let node = ListNode::new(v);
            current.next = Some(Box::new(node));
            current = current.next.as_mut().unwrap();
        }
        dummy.next
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut v = <Vec<i32>>::new();
        let mut current = Some(self);
        while let Some(node) = current {
            v.push(node.val);
            current = node.next.as_deref();
        }
        v
    }

    pub fn print(listnode: &Option<Box<ListNode>>) {
        let mut current = listnode;
        while let Some(node) = current {
            print!("{} ", node.val);
            current = &node.next;
        }
        println!();
    }
}

// This implementation is needed for Ord.
impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// This implementation is needed to allow sorting of ListNodes on a BinaryHeap.
impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl Default for ListNode {
    fn default() -> Self {
        ListNode::new(0)
    }
}
