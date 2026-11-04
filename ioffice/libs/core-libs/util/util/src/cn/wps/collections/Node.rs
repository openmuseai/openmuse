use std::ptr;

// Java field: int FLAG_HEAD_BIT
const FLAG_HEAD_BIT: i32 = 31;

// Java: public class Node
pub struct Node {
    // Java field: Node parent
    pub parent: *mut Node,
    // Java field: Node left
    pub left: *mut Node,
    // Java field: Node right
    pub right: *mut Node,
    // Java field: int flags
    pub(crate) flags: i32,
}

// Java: public class Node
impl Node {
    // Java: Node()
    pub fn new() -> Self {
        // Java: super();
        Self {
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            flags: 0,
        }
    }

    // Java: void setHead()
    pub fn setHead(&mut self) {
        // Java: this.flags |= Integer.MIN_VALUE;
        self.flags |= i32::MIN;
    }

    // Java: boolean isHead()
    pub fn isHead(&self) -> bool {
        // Java: return (this.flags & Integer.MIN_VALUE) != 0;
        (self.flags & i32::MIN) != 0
    }

    // Java: boolean isRoot()
    pub fn isRoot(&self) -> bool {
        // Java: return this.parent != null && this.parent.isHead();
        unsafe {
            !self.parent.is_null() && (*self.parent).isHead()
        }
    }

    // Java: Node getPrev()
    pub fn getPrev(&self) -> *mut Node {
        // Java: Node node = this;
        let mut node = self as *const Node as *mut Node;
        // Java: if (this.isHead())
        if self.isHead() {
            // Java: return this.right;
            return self.right;
        // Java: else
        } else {
            // Java: if (this.left != null)
            if !self.left.is_null() {
                // Java: for(node = this.left; node.right != null; node = node.right)
                node = self.left;
                unsafe {
                    while !(*node).right.is_null() {
                        node = (*node).right;
                    }
                }
            // Java: else
            } else {
                // Java: Node y;
                // Java: for(y = this.parent; node == y.left; y = y.parent)
                unsafe {
                    let mut y = self.parent;
                    while node == (*y).left {
                        // Java: node = y;
                        node = y;
                        y = (*y).parent;
                    }
                    // Java: if (node.left != y)
                    if (*node).left != y {
                        // Java: node = y;
                        node = y;
                    }
                }
            }
            // Java: return node;
            node
        }
    }

    // Java: Node getNext()
    pub fn getNext(&self) -> *mut Node {
        // Java: Node node = this;
        let mut node = self as *const Node as *mut Node;
        // Java: if (this.isHead())
        if self.isHead() {
            // Java: return this.left;
            return self.left;
        // Java: else
        } else {
            // Java: if (this.right != null)
            if !self.right.is_null() {
                // Java: for(node = this.right; node.left != null; node = node.left)
                node = self.right;
                unsafe {
                    while !(*node).left.is_null() {
                        node = (*node).left;
                    }
                }
            // Java: else
            } else {
                // Java: Node y;
                // Java: for(y = this.parent; node == y.right; y = y.parent)
                unsafe {
                    let mut y = self.parent;
                    while node == (*y).right {
                        // Java: node = y;
                        node = y;
                        y = (*y).parent;
                    }
                    // Java: if (node.right != y)
                    if (*node).right != y {
                        // Java: node = y;
                        node = y;
                    }
                }
            }
            // Java: return node;
            node
        }
    }

    // Java: Node getHead()
    pub fn getHead(&self) -> *mut Node {
        // Java: Node p;
        // Java: for(p = this; !p.isHead(); p = p.parent)
        let mut p = self as *const Node as *mut Node;
        unsafe {
            while !(*p).isHead() {
                p = (*p).parent;
            }
        }
        // Java: return p;
        p
    }

    // Java: Node getLeft()
    pub fn getLeft(&self) -> *mut Node {
        // Java: return this.left;
        self.left
    }

    // Java: Node getRight()
    pub fn getRight(&self) -> *mut Node {
        // Java: return this.right;
        self.right
    }
}
