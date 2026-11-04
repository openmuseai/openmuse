#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.collections.IndexTreeExtension
use crate::collections::IndexTreeExtension::{IndexTreeExtension, IIndexTreeNode};
use crate::collections::Node::Node;
use crate::collections::PositionTreeExtension::IPositionNode;
use super::TextReference::{TextReference, T};

// Java: public class IndexedTextReference extends TextReference implements IndexTreeExtension.IIndexTreeNode
pub struct IndexedTextReference {
    pub text_ref: TextReference,
    // Java field: int mLeftCount
    pub(crate) mLeftCount: i32,
}

// Java: public class IndexedTextReference extends TextReference implements IndexTreeExtension.IIndexTreeNode
impl IndexedTextReference {
    // Java: IndexedTextReference(int offset, T data)
    pub fn new(offset: i32, data: T) -> Self {
        // Java: super(offset, data);
        Self {
            text_ref: TextReference::new(offset, data),
            mLeftCount: 0,
        }
    }

    // Java: IndexedTextReference(T data)
    pub fn new_with_data(data: T) -> Self {
        // Java: super(data);
        Self::new(0, data)
    }

    // Java: int getLeftCount()
    pub fn getLeftCount_val(&self) -> i32 {
        // Java: return this.mLeftCount;
        self.mLeftCount
    }

    // Java: void setLeftCount(int nLeftCount)
    pub fn setLeftCount_val(&mut self, nLeftCount: i32) {
        // Java: this.mLeftCount = nLeftCount;
        self.mLeftCount = nLeftCount;
    }

    // Java: int getIndex()
    pub fn getIndex(&self) -> i32 {
        // Java: return IndexTreeExtension.getNodeIndex(this);
        let node_ptr = &self.text_ref.node as *const Node as *mut Node;
        IndexTreeExtension::getNodeIndex(node_ptr)
    }
}

impl IIndexTreeNode for IndexedTextReference {
    fn getLeftCount(&self) -> i32 {
        self.mLeftCount
    }
    fn setLeftCount(&mut self, var1: i32) {
        self.mLeftCount = var1;
    }
}

impl IPositionNode for IndexedTextReference {
    fn getOffset(&self) -> i32 {
        self.text_ref.offset
    }
    fn setOffset(&mut self, var1: i32) {
        self.text_ref.offset = var1;
    }
}
