#![allow(non_snake_case, dead_code, unused_variables)]

use std::ptr;

// Java field: int COLOR_BLACK
pub(crate) const COLOR_BLACK: i32 = 1;
// Java field: int COLOR_RED
pub(crate) const COLOR_RED: i32 = 2;
// Java field: int RED_MASK
pub(crate) const RED_MASK: i32 = 1;

// Java: public class RBNode
pub struct RBNode {
    // Java field: RBNode parent
    pub(crate) parent: *mut RBNode,
    // Java field: RBNode left
    pub(crate) left: *mut RBNode,
    // Java field: RBNode right
    pub(crate) right: *mut RBNode,
    // Java field: int mFlags
    pub(crate) mFlags: i32,
}

// Java: public class RBNode
impl RBNode {
    // Java: RBNode()
    pub fn new() -> Self {
        // Java: super();
        Self {
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            mFlags: 0,
        }
    }

    // Java: boolean isRed()
    pub fn isRed(&self) -> bool {
        // Java: return (this.mFlags & 1) != 0;
        (self.mFlags & 1) != 0
    }

    // Java: boolean isBlack()
    pub fn isBlack(&self) -> bool {
        // Java: return (this.mFlags & 1) == 0;
        (self.mFlags & 1) == 0
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
        // Java: return this.isRed() ? 2 : 1;
        if self.isRed() { 2 } else { 1 }
    }

    // Java: void setColor(int nColor)
    pub fn setColor(&mut self, n_color: i32) {
        // Java: if (nColor == 2)
        if n_color == 2
        {
            // Java: this.mFlags |= 1;
            self.mFlags |= 1;
        }
        // Java: else
        else
        {
            // Java: this.mFlags &= -2;
            self.mFlags &= !1;
        }
    }

    // Java: void setRed()
    pub fn setRed(&mut self) {
        // Java: this.mFlags |= 1;
        self.mFlags |= 1;
    }

    // Java: void setBlack()
    pub fn setBlack(&mut self) {
        // Java: this.mFlags &= -2;
        self.mFlags &= !1;
    }

    // Java: boolean isHead()
    pub fn isHead(&self) -> bool {
        // Java: return this.parent == null || this.isRed() && this.parent.parent == this;
        self.parent.is_null() || (self.isRed() && unsafe { (*self.parent).parent } == self as *const RBNode as *mut RBNode)
    }

    // Java: boolean isRoot()
    pub fn isRoot(&self) -> bool {
        // Java: return this.isBlack() && this.parent.parent == this;
        self.isBlack() && unsafe { (*self.parent).parent } == self as *const RBNode as *mut RBNode
    }

    // Java: RBNode getPrev()
    pub fn getPrev(&self) -> *mut RBNode {
        // Java: RBNode node = this;
        let mut node = self as *const RBNode as *mut RBNode;
        // Java: if (this.isHead())
        if self.isHead()
        {
            // Java: node = this.right;
            node = self.right;
        }
        // Java: else if (this.left != null)
        else if !self.left.is_null()
        {
            // Java: RBNode y;
            // Java: for(y = this.left; y.right != null; y = y.right)
            let mut y = self.left;
            while !unsafe { (*y).right }.is_null()
            {
                y = unsafe { (*y).right };
            }
            // Java: node = y;
            node = y;
        }
        // Java: else
        else
        {
            // Java: RBNode y;
            // Java: for(y = this.parent; node == y.left; y = y.parent)
            let mut y = self.parent;
            while node == unsafe { (*y).left }
            {
                // Java: node = y;
                node = y;
                y = unsafe { (*y).parent };
            }
            // Java: if (node.left != y)
            if unsafe { (*node).left } != y
            {
                // Java: node = y;
                node = y;
            }
        }
        // Java: return node;
        node
    }

    // Java: RBNode getNext()
    pub fn getNext(&self) -> *mut RBNode {
        // Java: RBNode node = this;
        let mut node = self as *const RBNode as *mut RBNode;
        // Java: if (this.right != null)
        if !self.right.is_null()
        {
            // Java: for(node = this.right; node.left != null; node = node.left)
            node = self.right;
            while !unsafe { (*node).left }.is_null()
            {
                node = unsafe { (*node).left };
            }
        }
        // Java: else
        else
        {
            // Java: RBNode y;
            // Java: for(y = this.parent; node == y.right; y = y.parent)
            let mut y = self.parent;
            while node == unsafe { (*y).right }
            {
                // Java: node = y;
                node = y;
                y = unsafe { (*y).parent };
            }
            // Java: if (node.right != y)
            if unsafe { (*node).right } != y
            {
                // Java: node = y;
                node = y;
            }
        }
        // Java: return node;
        node
    }

    // Java: String getDiagramId()
    pub fn getDiagramId(&self) -> String {
        // Java: return "RBNode";
        "RBNode".to_string()
    }
}
