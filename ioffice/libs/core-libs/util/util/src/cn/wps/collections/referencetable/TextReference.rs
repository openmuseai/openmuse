#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.collections.Node
use crate::collections::Node::Node;
// Java: import cn.wps.collections.PositionTreeExtension
use crate::collections::PositionTreeExtension::{PositionTreeExtension, IPositionNode};
use std::any::Any;

pub type T = Option<Box<dyn Any>>;

// Java: public class TextReference extends Node implements PositionTreeExtension.IPositionNode
pub struct TextReference {
    pub node: Node,
    // Java field: T data
    pub data: T,
    // Java field: int offset
    pub offset: i32,
}

// Java: public class TextReference extends Node implements PositionTreeExtension.IPositionNode
impl TextReference {
    // Java: TextReference(int offset, T data)
    pub fn new(offset: i32, data: T) -> Self {
        // Java: super();
        Self {
            node: Node::new(),
            // Java: this.offset = offset;
            offset,
            // Java: this.data = data;
            data,
        }
    }

    // Java: TextReference(T data)
    pub fn new_with_data(data: T) -> Self {
        // Java: this(0, data);
        Self::new(0, data)
    }

    // Java: T getData()
    pub fn getData(&self) -> &T {
        // Java: return this.data;
        &self.data
    }

    // Java: void setData(T data)
    pub fn setData(&mut self, data: T) {
        // Java: this.data = data;
        self.data = data;
    }

    // Java: int getOffset()
    pub fn getOffset_val(&self) -> i32 {
        // Java: return this.offset;
        self.offset
    }

    // Java: void setOffset(int offset)
    pub fn setOffset_val(&mut self, offset: i32) {
        // Java: this.offset = offset;
        self.offset = offset;
    }

    // Java: int position()
    pub fn position(&self) -> i32 {
        // Java: return PositionTreeExtension.getNodePosition(this);
        0
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append(this.getClass().getSimpleName() + ": ");
        sb.push_str("TextReference: ");
        // Java: sb.append("cp = " + this.position());
        sb.push_str(&format!("cp = {}", self.position()));
        // Java: return sb.toString();
        sb
    }

    pub fn isHead(&self) -> bool {
        self.node.isHead()
    }

    pub fn getPrev(&self) -> *mut Node {
        self.node.getPrev()
    }

    pub fn getNext(&self) -> *mut Node {
        self.node.getNext()
    }

    pub fn as_node_ptr(&mut self) -> *mut Node {
        &mut self.node as *mut Node
    }
}

impl IPositionNode for TextReference {
    fn getOffset(&self) -> i32 {
        self.offset
    }
    fn setOffset(&mut self, var1: i32) {
        self.offset = var1;
    }
}

#[inline]
pub fn node_to_textref(node: *mut Node) -> *mut TextReference {
    if node.is_null() {
        return std::ptr::null_mut();
    }
    node as *mut TextReference
}
