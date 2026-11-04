#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use super::Node::Node;
use super::PositionNode::PositionNode;
use super::IndexTreeExtension::{IndexTreeExtension, IIndexTreeNode};

// Java: public class IndexedPositionNode extends PositionNode implements IndexTreeExtension.IIndexTreeNode
#[repr(C)]
pub struct IndexedPositionNode {
    pub base: PositionNode,
    // Java field: int mLeftCount
    pub(crate) mLeftCount: i32,
}

// Java: public class IndexedPositionNode extends PositionNode implements IndexTreeExtension.IIndexTreeNode
impl IndexedPositionNode {
    // Java: IndexedPositionNode(int offset, int leftCount)
    pub fn new(offset: i32, leftCount: i32) -> Self {
        // Java: super(offset);
        // Java: this.mLeftCount = leftCount;
        Self {
            base: PositionNode::new_with_offset(offset),
            mLeftCount: leftCount,
        }
    }

    // Java: int index()
    pub fn index(&self) -> i32 {
        // Java: return IndexTreeExtension.getNodeIndex(this);
        let ptr = &self.base.base as *const Node as *mut Node;
        IndexTreeExtension::getNodeIndex(ptr)
    }
}

impl IIndexTreeNode for IndexedPositionNode {
    // Java: int getLeftCount()
    fn getLeftCount(&self) -> i32 {
        // Java: return this.mLeftCount;
        self.mLeftCount
    }

    // Java: void setLeftCount(int nLeftCount)
    fn setLeftCount(&mut self, nLeftCount: i32) {
        // Java: this.mLeftCount = nLeftCount;
        self.mLeftCount = nLeftCount;
    }
}
