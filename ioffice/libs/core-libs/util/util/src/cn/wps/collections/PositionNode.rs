#![allow(non_snake_case, dead_code)]

use super::Node::Node;
use super::PositionTreeExtension::IPositionNode;

// Java: public class PositionNode extends Node implements PositionTreeExtension.IPositionNode
#[repr(C)]
pub struct PositionNode {
    // Java field: (inherited from Node)
    pub base: Node,
    // Java field: int offset
    pub offset: i32,
}

// Java: public class PositionNode extends Node implements PositionTreeExtension.IPositionNode
impl PositionNode {
    // Java: PositionNode()
    pub fn new() -> Self {
        // Java: super();
        Self {
            base: Node::new(),
            offset: 0,
        }
    }

    // Java: PositionNode(int offset)
    pub fn new_with_offset(offset: i32) -> Self {
        // Java: super();
        // Java: this.offset = offset;
        Self {
            base: Node::new(),
            offset,
        }
    }

    // Java: int getOffset()
    pub fn getOffset(&self) -> i32 {
        // Java: return this.offset;
        self.offset
    }

    // Java: void setOffset(int offset)
    pub fn setOffset(&mut self, offset: i32) {
        // Java: this.offset = offset;
        self.offset = offset;
    }

    // Java: int position()
    pub fn position(&self) -> i32 {
        // Java: int cp = this.offset;
        let mut cp = self.offset;
        // Java: Node p = this.parent;
        let mut p = self.base.parent;
        // Java: if (p == null)
        if p.is_null() {
            // Java: return cp;
            return cp;
        }
        // Java: else
        else {
            unsafe {
                let mut node: *mut Node = &self.base as *const Node as *mut Node;
                // Java: for(Node node = this; !p.isHead(); p = p.parent)
                while !(*p).isHead() {
                    // Java: if (node == p.right)
                    if node == (*p).right {
                        // Java: cp += ((PositionNode)p).offset;
                        cp += (*(p as *mut PositionNode)).offset;
                    }
                    // Java: node = p;
                    node = p;
                    p = (*p).parent;
                }
            }
            // Java: return cp;
            return cp;
        }
    }

    // Java: void moveRight(int steps)
    pub fn moveRight(&mut self, steps: i32) {
        // Java: this.offset += steps;
        self.offset += steps;
        unsafe {
            // Java: for(Node n = this.right; n != null; n = n.left)
            let mut n = self.base.right;
            while !n.is_null() {
                // Java: PositionNode p = (PositionNode)n;
                let p = n as *mut PositionNode;
                // Java: p.offset -= steps;
                (*p).offset -= steps;
                n = (*n).left;
            }
        }
    }

    // Java: void moveLeft(int steps)
    pub fn moveLeft(&mut self, steps: i32) {
        // Java: this.moveRight(-steps);
        self.moveRight(-steps);
    }
}

impl IPositionNode for PositionNode {
    fn getOffset(&self) -> i32 {
        self.offset
    }
    fn setOffset(&mut self, var1: i32) {
        self.offset = var1;
    }
}
